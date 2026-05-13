// src/generator/traits.rs - 文档生成器 trait

use crate::error::Result;
use crate::schema::DatabaseSchema;
use std::path::Path;

/// 文档生成器 trait - 不同输出格式实现此 trait
pub trait DocumentGenerator {
    /// 生成文档内容
    fn generate(&self, schema: &DatabaseSchema) -> Result<String>;

    /// 获取输出文件扩展名
    fn file_extension(&self) -> &str;

    /// 获取生成器名称
    fn name(&self) -> &str;

    /// 生成并写入文件
    fn generate_to_file(&self, schema: &DatabaseSchema, output_path: &Path) -> Result<()> {
        let content = self.generate(schema)?;
        std::fs::write(output_path, content)?;
        Ok(())
    }
}
