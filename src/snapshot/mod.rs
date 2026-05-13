// src/snapshot/mod.rs - Snapshot 领域模块

pub mod store;
pub mod differ;

// 重新导出主要类型
pub use store::{SnapshotManager, DefaultSnapshotManager};
pub use differ::{SchemaDiffer, SchemaDiff, TableDiff, ColumnDiff, DiffType};
