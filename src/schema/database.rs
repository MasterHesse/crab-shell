// src/schema/database.rs - 数据库 Schema 领域模型

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use crate::schema::Table;
use crate::schema::Column;

/// 完整的数据库 Schema
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseSchema {
    pub database: String,
    pub schema: String,
    pub tables: Vec<Table>,
    pub views: Vec<View>,
    pub enums: Vec<EnumType>,
    pub generated_at: DateTime<Utc>,
}

/// 视图
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct View {
    pub name: String,
    pub comment: Option<String>,
    pub definition: Option<String>,
    pub columns: Vec<Column>,
}

/// 枚举类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnumType {
    pub name: String,
    pub values: Vec<String>,
}

impl DatabaseSchema {
    /// 创建新的 DatabaseSchema
    pub fn new(database: &str, schema: &str) -> Self {
        Self {
            database: database.to_string(),
            schema: schema.to_string(),
            tables: vec![],
            views: vec![],
            enums: vec![],
            generated_at: Utc::now(),
        }
    }

    /// 获取表数量
    pub fn table_count(&self) -> usize {
        self.tables.len()
    }

    /// 获取视图数量
    pub fn view_count(&self) -> usize {
        self.views.len()
    }

    /// 根据名称查找表
    pub fn find_table(&self, name: &str) -> Option<&Table> {
        self.tables.iter().find(|t| t.name == name)
    }
}
