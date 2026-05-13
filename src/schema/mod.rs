// src/schema/mod.rs - Schema 领域模块

pub mod database;
pub mod table;
pub mod traits;

// 重新导出主要类型
pub use database::{DatabaseSchema, View, EnumType};
pub use table::{Table, Column, PrimaryKey, ForeignKey, Index, Constraint};
pub use traits::SchemaReader;
