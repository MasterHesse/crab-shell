// src/lib.rs - crab-shell 库入口
// 提供公共 API 给集成测试和其他 crate 使用

pub mod cli;
pub mod config;
pub mod schema;
pub mod generator;
pub mod snapshot;
pub mod infra;
pub mod error;

// 重新导出常用类型
pub use schema::{DatabaseSchema, Table, Column, PrimaryKey, ForeignKey, Index, Constraint, SchemaReader};
pub use generator::{DocumentGenerator, MarkdownGenerator, MermaidGenerator, HtmlGenerator, DocumentFormat};
pub use snapshot::{SnapshotManager, SchemaDiffer};
pub use error::{CrabShellError, Result};
