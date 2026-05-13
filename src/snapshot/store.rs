// src/snapshot/store.rs - 快照保存/加载

use crate::error::Result;
use crate::schema::DatabaseSchema;
use std::path::Path;

/// 快照管理器 trait
pub trait SnapshotManager {
    /// 保存快照到文件
    fn save(&self, schema: &DatabaseSchema, path: &Path) -> Result<()>;

    /// 从文件加载快照
    fn load(&self, path: &Path) -> Result<DatabaseSchema>;
}

/// 默认快照管理器实现
pub struct DefaultSnapshotManager;

impl DefaultSnapshotManager {
    /// 创建新的快照管理器
    pub fn new() -> Self {
        Self
    }
}

impl Default for DefaultSnapshotManager {
    fn default() -> Self {
        Self::new()
    }
}

impl SnapshotManager for DefaultSnapshotManager {
    fn save(&self, schema: &DatabaseSchema, path: &Path) -> Result<()> {
        let json = serde_json::to_string_pretty(schema)?;
        std::fs::write(path, json)?;
        Ok(())
    }

    fn load(&self, path: &Path) -> Result<DatabaseSchema> {
        let content = std::fs::read_to_string(path)?;
        let schema: DatabaseSchema = serde_json::from_str(&content)?;
        Ok(schema)
    }
}
