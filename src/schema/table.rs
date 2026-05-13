// src/schema/table.rs - 表、列、约束等数据模型

use serde::{Deserialize, Serialize};

/// 表
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Table {
    pub name: String,
    pub comment: Option<String>,
    pub columns: Vec<Column>,
    pub primary_key: Option<PrimaryKey>,
    pub foreign_keys: Vec<ForeignKey>,
    pub indexes: Vec<Index>,
    pub constraints: Vec<Constraint>,
}

/// 列
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Column {
    pub name: String,
    pub data_type: String,
    pub full_data_type: String,
    pub nullable: bool,
    pub default_value: Option<String>,
    pub comment: Option<String>,
    pub ordinal_position: i32,
    pub is_identity: bool,
}

/// 主键
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimaryKey {
    pub name: String,
    pub columns: Vec<String>,
}

/// 外键
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForeignKey {
    pub name: String,
    pub columns: Vec<String>,
    pub referenced_table: String,
    pub referenced_columns: Vec<String>,
    pub on_update: Option<String>,
    pub on_delete: Option<String>,
}

/// 索引
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Index {
    pub name: String,
    pub columns: Vec<String>,
    pub is_unique: bool,
    pub index_type: Option<String>,
}

/// 约束
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Constraint {
    pub name: String,
    pub constraint_type: String,
    pub definition: Option<String>,
}

impl Table {
    /// 创建新表
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            comment: None,
            columns: vec![],
            primary_key: None,
            foreign_keys: vec![],
            indexes: vec![],
            constraints: vec![],
        }
    }

    /// 获取列数量
    pub fn column_count(&self) -> usize {
        self.columns.len()
    }

    /// 获取索引数量
    pub fn index_count(&self) -> usize {
        self.indexes.len()
    }

    /// 根据名称查找列
    pub fn find_column(&self, name: &str) -> Option<&Column> {
        self.columns.iter().find(|c| c.name == name)
    }
}
