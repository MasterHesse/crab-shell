// src/generator/mermaid.rs - Mermaid ER 图生成器

use crate::error::Result;
use crate::generator::DocumentGenerator;
use crate::schema::DatabaseSchema;

/// Mermaid ER 图生成器
pub struct MermaidGenerator;

impl MermaidGenerator {
    /// 创建新的 Mermaid 生成器
    pub fn new() -> Self {
        Self
    }
}

impl Default for MermaidGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl DocumentGenerator for MermaidGenerator {
    fn generate(&self, schema: &DatabaseSchema) -> Result<String> {
        let mut mermaid = String::new();

        mermaid.push_str("```mermaid\n");
        mermaid.push_str("erDiagram\n");

        // 定义实体
        for table in &schema.tables {
            mermaid.push_str(&format!("    {} {{\n", table.name));
            for col in &table.columns {
                let pk_marker = if table.primary_key.as_ref()
                    .map_or(false, |pk| pk.columns.contains(&col.name)) {
                    " PK"
                } else if table.foreign_keys.iter().any(|fk| fk.columns.contains(&col.name)) {
                    " FK"
                } else {
                    ""
                };
                mermaid.push_str(&format!("        {} {}{}\n", col.data_type, col.name, pk_marker));
            }
            mermaid.push_str("    }\n");
        }

        // 定义关系
        for table in &schema.tables {
            for fk in &table.foreign_keys {
                // 简单关系：假设多对一
                mermaid.push_str(&format!("    {} ||--o{{ {} : \"{}\"\n",
                    fk.referenced_table, table.name, fk.name));
            }
        }

        mermaid.push_str("```\n");

        Ok(mermaid)
    }

    fn file_extension(&self) -> &str {
        "mmd"
    }

    fn name(&self) -> &str {
        "Mermaid"
    }
}
