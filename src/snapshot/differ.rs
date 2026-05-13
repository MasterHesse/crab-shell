// src/snapshot/differ.rs - Schema Diff 引擎 (v1.1)

use crate::error::Result;
use crate::schema::{DatabaseSchema, Table, Column};
use std::collections::HashMap;

/// 结果类型别名
type DiffResult<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

/// Schema 差异类型
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DiffType {
    Added,
    Removed,
    Modified,
}

/// 表差异
#[derive(Debug, Clone)]
pub struct TableDiff {
    pub name: String,
    pub diff_type: DiffType,
    pub column_diffs: Vec<ColumnDiff>,
}

/// 列差异
#[derive(Debug, Clone)]
pub struct ColumnDiff {
    pub name: String,
    pub diff_type: DiffType,
    pub old_type: Option<String>,
    pub new_type: Option<String>,
}

/// Schema 差异报告
#[derive(Debug, Clone)]
pub struct SchemaDiff {
    pub old_schema: String,
    pub new_schema: String,
    pub table_diffs: Vec<TableDiff>,
}

/// Schema Diff 引擎
pub struct SchemaDiffer;

impl SchemaDiffer {
    /// 创建新的 Diff 引擎
    pub fn new() -> Self {
        Self
    }

    /// 对比两个 Schema
    pub fn diff(&self, old: &DatabaseSchema, new: &DatabaseSchema) -> SchemaDiff {
        let mut table_diffs = Vec::new();

        let old_tables: HashMap<&str, &Table> = old.tables.iter()
            .map(|t| (t.name.as_str(), t))
            .collect();
        let new_tables: HashMap<&str, &Table> = new.tables.iter()
            .map(|t| (t.name.as_str(), t))
            .collect();

        // 检查新增和变更的表
        for (name, new_table) in &new_tables {
            match old_tables.get(name) {
                Some(old_table) => {
                    // 表存在，检查列变更
                    let column_diffs = Self::diff_columns(old_table, new_table);
                    if !column_diffs.is_empty() {
                        table_diffs.push(TableDiff {
                            name: name.to_string(),
                            diff_type: DiffType::Modified,
                            column_diffs,
                        });
                    }
                }
                None => {
                    // 新增的表
                    table_diffs.push(TableDiff {
                        name: name.to_string(),
                        diff_type: DiffType::Added,
                        column_diffs: vec![],
                    });
                }
            }
        }

        // 检查删除的表
        for (name, _) in &old_tables {
            if !new_tables.contains_key(name) {
                table_diffs.push(TableDiff {
                    name: name.to_string(),
                    diff_type: DiffType::Removed,
                    column_diffs: vec![],
                });
            }
        }

        SchemaDiff {
            old_schema: old.database.clone(),
            new_schema: new.database.clone(),
            table_diffs,
        }
    }

    /// 对比两个表的列
    fn diff_columns(old: &Table, new: &Table) -> Vec<ColumnDiff> {
        let mut diffs = Vec::new();

        let old_cols: HashMap<&str, &Column> = old.columns.iter()
            .map(|c| (c.name.as_str(), c))
            .collect();
        let new_cols: HashMap<&str, &Column> = new.columns.iter()
            .map(|c| (c.name.as_str(), c))
            .collect();

        // 检查新增和变更的列
        for (name, new_col) in &new_cols {
            match old_cols.get(name) {
                Some(old_col) => {
                    // 列存在，检查类型变更
                    if old_col.data_type != new_col.data_type {
                        diffs.push(ColumnDiff {
                            name: name.to_string(),
                            diff_type: DiffType::Modified,
                            old_type: Some(old_col.data_type.clone()),
                            new_type: Some(new_col.data_type.clone()),
                        });
                    }
                }
                None => {
                    // 新增的列
                    diffs.push(ColumnDiff {
                        name: name.to_string(),
                        diff_type: DiffType::Added,
                        old_type: None,
                        new_type: Some(new_col.data_type.clone()),
                    });
                }
            }
        }

        // 检查删除的列
        for (name, _) in &old_cols {
            if !new_cols.contains_key(name) {
                diffs.push(ColumnDiff {
                    name: name.to_string(),
                    diff_type: DiffType::Removed,
                    old_type: None,
                    new_type: None,
                });
            }
        }

        diffs
    }

    /// 生成 Markdown 格式的变更报告
    pub fn generate_markdown_report(&self, diff: &SchemaDiff) -> String {
        let mut md = String::new();

        md.push_str("# Schema Diff Report\n\n");
        md.push_str(&format!("**Old**: {}\n", diff.old_schema));
        md.push_str(&format!("**New**: {}\n\n", diff.new_schema));
        md.push_str("---\n\n");

        for table_diff in &diff.table_diffs {
            match table_diff.diff_type {
                DiffType::Added => {
                    md.push_str(&format!("## Added Table: {}\n\n", table_diff.name));
                }
                DiffType::Removed => {
                    md.push_str(&format!("## Removed Table: {}\n\n", table_diff.name));
                }
                DiffType::Modified => {
                    md.push_str(&format!("## Modified Table: {}\n\n", table_diff.name));
                    if !table_diff.column_diffs.is_empty() {
                        md.push_str("### Column Changes\n\n");
                        for col_diff in &table_diff.column_diffs {
                            match col_diff.diff_type {
                                DiffType::Added => {
                                    md.push_str(&format!("- **Added column**: {} ({})\n", col_diff.name, col_diff.new_type.as_deref().unwrap_or("?")));
                                }
                                DiffType::Removed => {
                                    md.push_str(&format!("- **Removed column**: {}\n", col_diff.name));
                                }
                                DiffType::Modified => {
                                    md.push_str(&format!("- **Modified column**: {}: {} → {}\n",
                                        col_diff.name,
                                        col_diff.old_type.as_deref().unwrap_or("?"),
                                        col_diff.new_type.as_deref().unwrap_or("?")));
                                }
                            }
                        }
                        md.push('\n');
                    }
                }
            }
        }

        md
    }
}

impl Default for SchemaDiffer {
    fn default() -> Self {
        Self::new()
    }
}
