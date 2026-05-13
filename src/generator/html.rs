// src/generator/html.rs - HTML 文档生成器 (v1.0)

use crate::error::Result;
use crate::generator::DocumentGenerator;
use crate::schema::DatabaseSchema;
use chrono::Utc;

/// HTML 文档生成器
pub struct HtmlGenerator {
    template_dir: std::path::PathBuf,
}

impl HtmlGenerator {
    /// 创建新的 HTML 生成器
    pub fn new() -> Self {
        Self {
            template_dir: std::path::PathBuf::from("templates"),
        }
    }

    /// 设置模板目录
    pub fn with_template_dir<P: Into<std::path::PathBuf>>(dir: P) -> Self {
        Self {
            template_dir: dir.into(),
        }
    }
}

impl Default for HtmlGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl DocumentGenerator for HtmlGenerator {
    fn generate(&self, schema: &DatabaseSchema) -> Result<String> {
        // TODO: 使用 Tera 模板引擎生成 HTML
        // 暂时返回简单的 HTML 骨架
        let mut html = String::new();

        html.push_str("<!DOCTYPE html>\n");
        html.push_str("<html lang=\"zh-CN\">\n");
        html.push_str("<head>\n");
        html.push_str("    <meta charset=\"UTF-8\">\n");
        html.push_str(&format!("    <title>{} - Schema Documentation</title>\n", schema.database));
        html.push_str("    <script src=\"https://cdn.jsdelivr.net/npm/mermaid/dist/mermaid.min.js\"></script>\n");
        html.push_str("</head>\n");
        html.push_str("<body>\n");
        html.push_str(&format!("    <h1>Database: {}</h1>\n", schema.database));
        html.push_str(&format!("    <p>Generated on: {}</p>\n", Utc::now().format("%Y-%m-%d %H:%M:%S")));

        // 表格列表
        html.push_str("    <h2>Tables</h2>\n");
        for table in &schema.tables {
            html.push_str(&format!("    <h3 id=\"{}\">{}</h3>\n", table.name, table.name));
            if let Some(comment) = &table.comment {
                html.push_str(&format!("    <p><strong>Description</strong>: {}</p>\n", comment));
            }
            // TODO: 生成表格详情
        }

        html.push_str("    <h2>ER Diagram</h2>\n");
        html.push_str("    <div class=\"mermaid\"></div>\n");

        html.push_str("    <script>mermaid.initialize({ startOnLoad: true });</script>\n");
        html.push_str("</body>\n");
        html.push_str("</html>");

        Ok(html)
    }

    fn file_extension(&self) -> &str {
        "html"
    }

    fn name(&self) -> &str {
        "HTML"
    }
}
