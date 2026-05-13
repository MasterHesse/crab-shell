// src/schema/traits.rs - Schema 领域 trait 接口

use async_trait::async_trait;
use crate::error::Result;
use crate::schema::DatabaseSchema;

/// Schema 读取器 trait - 定义读取数据库 Schema 的接口
/// 基础设施层需要实现此 trait
#[async_trait]
pub trait SchemaReader: Send + Sync {
    /// 读取指定 schema 的数据库结构
    async fn read_schema(&self, schema_name: &str) -> Result<DatabaseSchema>;

    /// 读取表列表
    async fn read_tables(&self, schema_name: &str) -> Result<Vec<crate::schema::Table>>;

    /// 读取指定表的列信息
    async fn read_columns(&self, schema_name: &str, table_name: &str) -> Result<Vec<crate::schema::Column>>;

    /// 读取指定表的约束信息
    async fn read_constraints(&self, schema_name: &str, table_name: &str) -> Result<Vec<crate::schema::Constraint>>;

    /// 读取指定表的索引信息
    async fn read_indexes(&self, schema_name: &str, table_name: &str) -> Result<Vec<crate::schema::Index>>;

    /// 读取指定表的外键信息
    async fn read_foreign_keys(&self, schema_name: &str, table_name: &str) -> Result<Vec<crate::schema::ForeignKey>>;

    /// 测试连接
    async fn test_connection(&self) -> Result<()>;
}
