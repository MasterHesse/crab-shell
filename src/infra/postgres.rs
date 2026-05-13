// src/infra/postgres.rs - PostgreSQL 适配器（实现 SchemaReader）

use async_trait::async_trait;
use tokio_postgres::{NoTls, Row};
use crate::error::Result;
use crate::schema::{DatabaseSchema, Table, Column, PrimaryKey, ForeignKey, Index, Constraint};
use crate::schema::traits::SchemaReader;

/// PostgreSQL 适配器
pub struct PostgresAdapter {
    client: tokio_postgres::Client,
    connection_string: String,
}

impl PostgresAdapter {
    /// 从连接字符串解析 host 和 port
    fn parse_connection_info(url: &str) -> (String, u16) {
        if let Ok(parsed) = url::Url::parse(url) {
            let host = parsed.host_str().unwrap_or("localhost").to_string();
            let port = parsed.port().unwrap_or(5432);
            (host, port)
        } else {
            ("localhost".to_string(), 5432)
        }
    }

    /// 创建新的 PostgreSQL 适配器
    pub async fn new(connection_string: &str) -> Result<Self> {
        let (host, port) = Self::parse_connection_info(connection_string);
        let (client, connection) = tokio_postgres::connect(connection_string, NoTls).await
            .map_err(|e| crate::error::CrabShellError::ConnectionFailed {
                host,
                port,
                reason: e.to_string(),
                help: "请检查数据库是否在运行，连接参数是否正确。".to_string(),
            })?;

        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("Database connection error: {}", e);
            }
        });

        Ok(Self {
            client,
            connection_string: connection_string.to_string(),
        })
    }
}

#[async_trait]
impl SchemaReader for PostgresAdapter {
    async fn read_schema(&self, schema_name: &str) -> Result<DatabaseSchema> {
        let mut schema = DatabaseSchema::new("database", schema_name);

        // 读取表列表
        let tables = self.read_tables(schema_name).await?;
        schema.tables = tables;

        Ok(schema)
    }

    async fn read_tables(&self, schema_name: &str) -> Result<Vec<Table>> {
        let rows = self.client.query(
            "SELECT table_name FROM information_schema.tables 
             WHERE table_schema = $1 AND table_type = 'BASE TABLE'
             ORDER BY table_name",
            &[&schema_name],
        ).await?;

        let mut tables = Vec::new();
        for row in rows {
            let table_name: String = row.get(0);
            let mut table = Table::new(&table_name);

            // 读取表注释
            let comment_rows = self.client.query(
                "SELECT obj_description(c.oid) 
                 FROM pg_class c 
                 JOIN pg_namespace n ON c.relnamespace = n.oid
                 WHERE n.nspname = $1 AND c.relname = $2",
                &[&schema_name, &table_name],
            ).await?;
            if let Some(comment_row) = comment_rows.first() {
                let comment: Option<String> = comment_row.get(0);
                table.comment = comment;
            }

            // 读取列信息
            let columns = self.read_columns(schema_name, &table_name).await?;
            table.columns = columns;

            // 读取主键
            let primary_keys = self.read_primary_keys(schema_name, &table_name).await?;
            table.primary_key = primary_keys;

            // 读取外键
            let foreign_keys = self.read_foreign_keys(schema_name, &table_name).await?;
            table.foreign_keys = foreign_keys;

            // 读取索引
            let indexes = self.read_indexes(schema_name, &table_name).await?;
            table.indexes = indexes;

            // 读取约束
            let constraints = self.read_constraints(schema_name, &table_name).await?;
            table.constraints = constraints;

            tables.push(table);
        }

        Ok(tables)
    }

    async fn read_columns(&self, schema_name: &str, table_name: &str) -> Result<Vec<Column>> {
        let rows = self.client.query(
            "SELECT column_name, data_type, is_nullable, column_default, 
                    character_maximum_length, ordinal_position
             FROM information_schema.columns
             WHERE table_schema = $1 AND table_name = $2
             ORDER BY ordinal_position",
            &[&schema_name, &table_name],
        ).await?;

        let mut columns = Vec::new();
        for row in rows {
            let name: String = row.get(0);
            let data_type: String = row.get(1);
            let is_nullable: String = row.get(2);
            let column_default: Option<String> = row.get(3);
            let ordinal_position: i32 = row.get(5);

            // 读取列注释
            let comment_rows = self.client.query(
                "SELECT col_description(c.oid, a.attnum)
                 FROM pg_class c
                 JOIN pg_namespace n ON c.relnamespace = n.oid
                 JOIN pg_attribute a ON a.attrelid = c.oid
                 WHERE n.nspname = $1 AND c.relname = $2 
                   AND a.attname = $3 AND a.attnum > 0 AND NOT a.attisdropped",
                &[&schema_name, &table_name, &name],
            ).await?;
            let comment = if let Some(comment_row) = comment_rows.first() {
                comment_row.get(0)
            } else {
                None
            };

            columns.push(Column {
                name,
                data_type: data_type.clone(),
                full_data_type: data_type,
                nullable: is_nullable == "YES",
                default_value: column_default,
                comment,
                ordinal_position,
                is_identity: false, // TODO: 检测 IDENTITY
            });
        }

        Ok(columns)
    }

    async fn read_constraints(&self, schema_name: &str, table_name: &str) -> Result<Vec<Constraint>> {
        let mut constraints = Vec::new();
        
        // 读取 CHECK 约束
        let check_rows = self.client.query(
            "SELECT conname, pg_get_constraintdef(oid) 
             FROM pg_constraint 
             JOIN pg_class ON conrelid = pg_class.oid
             JOIN pg_namespace ON relnamespace = pg_namespace.oid
             WHERE contype = 'c' 
               AND nspname = $1 AND relname = $2",
            &[&schema_name, &table_name],
        ).await?;
        
        for row in check_rows {
            let name: String = row.get(0);
            let definition: String = row.get(1);
            
            // 从定义中提取列名 (CHECK (column > 0))
            let columns = PostgresAdapter::extract_columns_from_check(&definition);
            
            constraints.push(Constraint {
                name,
                constraint_type: "CHECK".to_string(),
                columns,
                definition: Some(definition),
            });
        }
        
        // 读取 UNIQUE 约束
        let unique_rows = self.client.query(
            "SELECT tc.constraint_name, kcu.column_name
             FROM information_schema.table_constraints tc
             JOIN information_schema.key_column_usage kcu
               ON tc.constraint_name = kcu.constraint_name
             WHERE tc.table_schema = $1 AND tc.table_name = $2
               AND tc.constraint_type = 'UNIQUE'
             ORDER BY tc.constraint_name, kcu.ordinal_position",
            &[&schema_name, &table_name],
        ).await?;
        
        // 按约束名分组
        let mut unique_map: std::collections::HashMap<String, Vec<String>> = std::collections::HashMap::new();
        for row in unique_rows {
            let name: String = row.get(0);
            let column: String = row.get(1);
            unique_map.entry(name).or_default().push(column);
        }
        
        for (name, columns) in unique_map {
            constraints.push(Constraint {
                name,
                constraint_type: "UNIQUE".to_string(),
                columns,
                definition: None,
            });
        }
        
        // 读取 REFERENCES 约束 (外键)
        let fk_rows = self.client.query(
            "SELECT tc.constraint_name, kcu.column_name
             FROM information_schema.table_constraints tc
             JOIN information_schema.key_column_usage kcu
               ON tc.constraint_name = kcu.constraint_name
             WHERE tc.table_schema = $1 AND tc.table_name = $2
               AND tc.constraint_type = 'FOREIGN KEY'
             ORDER BY tc.constraint_name, kcu.ordinal_position",
            &[&schema_name, &table_name],
        ).await?;
        
        let mut fk_map: std::collections::HashMap<String, Vec<String>> = std::collections::HashMap::new();
        for row in fk_rows {
            let name: String = row.get(0);
            let column: String = row.get(1);
            fk_map.entry(name).or_default().push(column);
        }
        
        for (name, columns) in fk_map {
            constraints.push(Constraint {
                name,
                constraint_type: "REFERENCES".to_string(),
                columns,
                definition: None,
            });
        }
        
        Ok(constraints)
    }

    async fn read_indexes(&self, schema_name: &str, table_name: &str) -> Result<Vec<Index>> {
        let rows = self.client.query(
            "SELECT i.relname AS index_name, 
                    array_agg(a.attname ORDER BY array_position(ix.indkey, a.attnum)) AS columns,
                    ix.indisunique AS is_unique
             FROM pg_index ix
             JOIN pg_class t ON ix.indrelid = t.oid
             JOIN pg_class i ON ix.indexrelid = i.oid
             JOIN pg_namespace n ON t.relnamespace = n.oid
             CROSS JOIN LATERAL unnest(ix.indkey) WITH ORDINALITY AS k(attnum, ord) 
             LEFT JOIN pg_attribute a ON a.attrelid = t.oid AND a.attnum = k.attnum
             WHERE n.nspname = $1 AND t.relname = $2
             GROUP BY i.relname, ix.indisunique",
            &[&schema_name, &table_name],
        ).await?;

        let mut indexes = Vec::new();
        for row in rows {
            let name: String = row.get(0);
            let columns: Vec<Option<String>> = row.get(1);
            let is_unique: bool = row.get(2);

            let column_names = columns.into_iter()
                .filter_map(|c| c)
                .collect();

            indexes.push(Index {
                name,
                columns: column_names,
                is_unique,
                index_type: Some("btree".to_string()), // TODO: 检测索引类型
            });
        }

        Ok(indexes)
    }

    async fn read_foreign_keys(&self, schema_name: &str, table_name: &str) -> Result<Vec<ForeignKey>> {
        let rows = self.client.query(
            "SELECT tc.constraint_name,
                    kcu.column_name,
                    ccu.table_name AS referenced_table,
                    ccu.column_name AS referenced_column
             FROM information_schema.table_constraints AS tc
             JOIN information_schema.key_column_usage AS kcu
               ON tc.constraint_name = kcu.constraint_name
             JOIN information_schema.constraint_column_usage AS ccu
               ON ccu.constraint_name = tc.constraint_name
             WHERE tc.table_schema = $1 AND tc.table_name = $2
               AND tc.constraint_type = 'FOREIGN KEY'",
            &[&schema_name, &table_name],
        ).await?;

        // 按约束名分组
        let mut fk_map: std::collections::HashMap<String, ForeignKey> = std::collections::HashMap::new();
        for row in rows {
            let constraint_name: String = row.get(0);
            let column_name: String = row.get(1);
            let referenced_table: String = row.get(2);
            let referenced_column: String = row.get(3);

            let entry = fk_map.entry(constraint_name.clone()).or_insert_with(|| ForeignKey {
                name: constraint_name,
                columns: vec![],
                referenced_table: referenced_table.clone(),
                referenced_columns: vec![],
                on_update: None,
                on_delete: None,
            });

            entry.columns.push(column_name);
            entry.referenced_columns.push(referenced_column);
        }

        Ok(fk_map.into_values().collect())
    }

    async fn test_connection(&self) -> Result<()> {
        self.client.query("SELECT 1", &[]).await?;
        Ok(())
    }
}

impl PostgresAdapter {
    /// 读取主键信息
    async fn read_primary_keys(&self, schema_name: &str, table_name: &str) -> Result<Option<PrimaryKey>> {
        let rows = self.client.query(
            "SELECT tc.constraint_name, kcu.column_name
             FROM information_schema.table_constraints tc
             JOIN information_schema.key_column_usage kcu
               ON tc.constraint_name = kcu.constraint_name
             WHERE tc.table_schema = $1 AND tc.table_name = $2
               AND tc.constraint_type = 'PRIMARY KEY'
             ORDER BY kcu.ordinal_position",
            &[&schema_name, &table_name],
        ).await?;

        if rows.is_empty() {
            return Ok(None);
        }

        let constraint_name: String = rows[0].get(0);
        let columns: Vec<String> = rows.iter().map(|r| r.get(1)).collect();

        Ok(Some(PrimaryKey {
            name: constraint_name,
            columns,
        }))
    }
    
    /// 从 CHECK 约束定义中提取列名
    fn extract_columns_from_check(definition: &str) -> Vec<String> {
        // CHECK 约束格式: (column > 0) 或 CHECK (column > 0)
        let definition = definition.trim();
        let content = if definition.starts_with("CHECK (") {
            &definition[7..definition.len()-1]
        } else {
            definition
        };
        
        // 简单提取：找到列名字符（字母数字下划线）
        let mut columns = Vec::new();
        let mut current = String::new();
        let mut in_identifier = false;
        
        for ch in content.chars() {
            if ch.is_alphanumeric() || ch == '_' {
                current.push(ch);
                in_identifier = true;
            } else {
                if in_identifier && !current.is_empty() {
                    // 过滤掉数字开头的（可能是值而非列名）
                    if !current.chars().next().map(|c| c.is_numeric()).unwrap_or(false) {
                        columns.push(current.clone());
                    }
                    current.clear();
                }
                in_identifier = false;
            }
        }
        
        if in_identifier && !current.is_empty() {
            if !current.chars().next().map(|c| c.is_numeric()).unwrap_or(false) {
                columns.push(current);
            }
        }
        
        columns
    }
}
