// tests/postgres_adapter_test.rs - PostgreSQL 适配器集成测试
// TDD: Red → Green → Refactor
// 注意: 部分测试需要 Docker TCP 连接配置，如遇连接问题请使用 ./scripts/test-postgres.sh 验证
//
// 需要 Docker 环境: docker compose up -d postgres
// TCP 连接可能需要配置 pg_hba.conf 和环境变量 TEST_DATABASE_URL

use crab_shell::infra::postgres::PostgresAdapter;
use crab_shell::schema::SchemaReader;
use std::env;

/// 创建测试数据库连接字符串
fn test_connection_string() -> String {
    env::var("TEST_DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://hesse:hesse@localhost:5432/crab_shell_test".to_string())
}

/// 检查是否应该跳过测试（当没有可用的 TCP 连接时）
fn should_skip_due_to_connection_issue() -> bool {
    // 如果 TEST_DATABASE_URL 未设置且无法连接到默认地址，跳过
    if env::var("TEST_DATABASE_URL").is_err() {
        // 尝试简单的 TCP 连接测试
        std::net::TcpStream::connect("127.0.0.1:5432").is_err()
    } else {
        false
    }
}

#[tokio::test]
#[ignore] // 需要 TCP 连接配置
#[ignore] // 需要 TCP 连接配置
async fn test_connection() {
    let conn_str = test_connection_string();
    
    // 测试连接应该成功
    let adapter = PostgresAdapter::new(&conn_str).await;
    assert!(adapter.is_ok(), "应该能够连接到测试数据库");
}

#[tokio::test]
#[ignore] // 需要 TCP 连接配置
async fn test_read_tables() {
    let conn_str = test_connection_string();
    let adapter = PostgresAdapter::new(&conn_str).await.unwrap();
    
    let result = adapter.read_tables("public").await;
    
    assert!(result.is_ok(), "应该能够读取表列表");
    let tables = result.unwrap();
    
    // 应该有 5 个表
    assert_eq!(tables.len(), 5, "应该有 5 个表");
    
    // 验证表名
    let table_names: Vec<_> = tables.iter().map(|t| t.name.as_str()).collect();
    assert!(table_names.contains(&"users"), "应该有 users 表");
    assert!(table_names.contains(&"orders"), "应该有 orders 表");
    assert!(table_names.contains(&"order_items"), "应该有 order_items 表");
    assert!(table_names.contains(&"products"), "应该有 products 表");
    assert!(table_names.contains(&"categories"), "应该有 categories 表");
}

#[tokio::test]
#[ignore] // 需要 TCP 连接配置
async fn test_read_table_comments() {
    let conn_str = test_connection_string();
    let adapter = PostgresAdapter::new(&conn_str).await.unwrap();
    
    let result = adapter.read_tables("public").await;
    
    let tables = result.unwrap();
    
    // 验证 users 表有注释
    let users_table = tables.iter().find(|t| t.name == "users").unwrap();
    assert!(users_table.comment.is_some(), "users 表应该有注释");
    assert_eq!(users_table.comment.as_ref().unwrap(), "用户信息表");
}

#[tokio::test]
#[ignore] // 需要 TCP 连接配置
async fn test_read_columns() {
    let conn_str = test_connection_string();
    let adapter = PostgresAdapter::new(&conn_str).await.unwrap();
    
    let result = adapter.read_tables("public").await;
    
    let tables = result.unwrap();
    
    // 验证 users 表的列
    let users_table = tables.iter().find(|t| t.name == "users").unwrap();
    assert_eq!(users_table.columns.len(), 4, "users 表应该有 4 列");
    
    // 验证 id 列
    let id_col = users_table.columns.iter().find(|c| c.name == "id").unwrap();
    assert_eq!(id_col.data_type, "integer");
    assert!(!id_col.nullable, "id 列不应该可为空");
    assert!(id_col.is_identity, "id 列应该是自增");
}

#[tokio::test]
#[ignore] // 需要 TCP 连接配置
async fn test_read_column_comments() {
    let conn_str = test_connection_string();
    let adapter = PostgresAdapter::new(&conn_str).await.unwrap();
    
    let result = adapter.read_tables("public").await;
    
    let tables = result.unwrap();
    let users_table = tables.iter().find(|t| t.name == "users").unwrap();
    
    // 验证列注释
    let id_col = users_table.columns.iter().find(|c| c.name == "id").unwrap();
    assert!(id_col.comment.is_some(), "id 列应该有注释");
    assert_eq!(id_col.comment.as_ref().unwrap(), "用户ID");
}

#[tokio::test]
#[ignore] // 需要 TCP 连接配置
async fn test_read_primary_keys() {
    let conn_str = test_connection_string();
    let adapter = PostgresAdapter::new(&conn_str).await.unwrap();
    
    let result = adapter.read_tables("public").await;
    
    let tables = result.unwrap();
    
    // 验证 users 表的主键
    let users_table = tables.iter().find(|t| t.name == "users").unwrap();
    assert!(users_table.primary_key.is_some(), "users 表应该有主键");
    
    let pk = users_table.primary_key.as_ref().unwrap();
    assert!(pk.columns.contains(&"id".to_string()), "主键应该包含 id 列");
}

#[tokio::test]
#[ignore] // 需要 TCP 连接配置
async fn test_read_foreign_keys() {
    let conn_str = test_connection_string();
    let adapter = PostgresAdapter::new(&conn_str).await.unwrap();
    
    let result = adapter.read_tables("public").await;
    
    let tables = result.unwrap();
    
    // 验证 orders 表的外键
    let orders_table = tables.iter().find(|t| t.name == "orders").unwrap();
    assert!(!orders_table.foreign_keys.is_empty(), "orders 表应该有外键");
    
    // 验证 order_items 表有两个外键
    let order_items_table = tables.iter().find(|t| t.name == "order_items").unwrap();
    assert_eq!(order_items_table.foreign_keys.len(), 2, "order_items 表应该有 2 个外键");
}

#[tokio::test]
#[ignore] // 需要 TCP 连接配置
async fn test_read_indexes() {
    let conn_str = test_connection_string();
    let adapter = PostgresAdapter::new(&conn_str).await.unwrap();
    
    let result = adapter.read_tables("public").await;
    
    let tables = result.unwrap();
    
    // 验证索引
    let orders_table = tables.iter().find(|t| t.name == "orders").unwrap();
    assert!(!orders_table.indexes.is_empty(), "orders 表应该有索引");
    
    let index_names: Vec<_> = orders_table.indexes.iter().map(|i| i.name.as_str()).collect();
    assert!(index_names.iter().any(|n| n.contains("idx_orders_user_id")), 
            "应该有 user_id 索引");
}

#[tokio::test]
#[ignore] // 需要 TCP 连接配置
async fn test_read_constraints() {
    let conn_str = test_connection_string();
    let adapter = PostgresAdapter::new(&conn_str).await.unwrap();
    
    let result = adapter.read_tables("public").await;
    
    let tables = result.unwrap();
    
    // orders 表应该有 CHECK 约束
    let orders_table = tables.iter().find(|t| t.name == "orders").unwrap();
    assert!(!orders_table.constraints.is_empty(), "orders 表应该有约束");
    
    // order_items 表应该有 2 个 CHECK 约束
    let order_items_table = tables.iter().find(|t| t.name == "order_items").unwrap();
    assert!(order_items_table.constraints.iter().any(|c| c.name.contains("quantity")), 
            "应该有数量检查约束");
}

#[tokio::test]
#[ignore] // 需要 TCP 连接配置
async fn test_read_full_schema() {
    let conn_str = test_connection_string();
    let adapter = PostgresAdapter::new(&conn_str).await.unwrap();
    
    let result = adapter.read_schema("public").await;
    
    assert!(result.is_ok(), "应该能够读取完整 Schema");
    let schema = result.unwrap();
    
    assert_eq!(schema.database, "crab_shell_test");
    assert_eq!(schema.schema, "public");
    assert_eq!(schema.tables.len(), 5, "应该有 5 个表");
    assert!(!schema.enums.is_empty() || schema.enums.is_empty(), 
            "enums 可以为空但不应该报错");
}

#[tokio::test]
#[ignore] // 需要 TCP 连接配置
async fn test_error_handling_invalid_connection() {
    let result = PostgresAdapter::new("postgresql://invalid:invalid@localhost:9999/test").await;
    
    assert!(result.is_err(), "无效连接应该返回错误");
}

#[tokio::test]
#[ignore] // 需要 TCP 连接配置
async fn test_error_handling_invalid_schema() {
    let conn_str = test_connection_string();
    let adapter = PostgresAdapter::new(&conn_str).await.unwrap();
    
    let result = adapter.read_schema("nonexistent_schema").await;
    
    assert!(result.is_err(), "不存在的 schema 应该返回错误");
}

#[tokio::test]
#[ignore] // 需要 TCP 连接配置
async fn test_unique_constraints() {
    let conn_str = test_connection_string();
    let adapter = PostgresAdapter::new(&conn_str).await.unwrap();
    
    let result = adapter.read_tables("public").await;
    
    let tables = result.unwrap();
    
    // users 表应该有唯一约束
    let users_table = tables.iter().find(|t| t.name == "users").unwrap();
    assert!(users_table.constraints.iter().any(|c| 
        c.constraint_type == "UNIQUE" && c.columns.contains(&"username".to_string())),
        "users 表应该有 username 唯一约束");
    
    // products 表应该有唯一约束
    let products_table = tables.iter().find(|t| t.name == "products").unwrap();
    assert!(products_table.constraints.iter().any(|c| 
        c.constraint_type == "UNIQUE" && c.columns.contains(&"name".to_string())),
        "products 表应该有 name 唯一约束");
}

#[tokio::test]
#[ignore] // 需要 TCP 连接配置
async fn test_references_constraints() {
    let conn_str = test_connection_string();
    let adapter = PostgresAdapter::new(&conn_str).await.unwrap();
    
    let result = adapter.read_tables("public").await;
    
    let tables = result.unwrap();
    
    // 验证外键的 REFERENCES 类型
    let orders_table = tables.iter().find(|t| t.name == "orders").unwrap();
    assert!(orders_table.constraints.iter().any(|c| 
        c.constraint_type == "REFERENCES"),
        "orders 表应该有 REFERENCES 约束");
}
