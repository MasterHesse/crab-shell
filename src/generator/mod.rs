// src/generator/mod.rs - Generator 领域模块

pub mod traits;
pub mod markdown;
pub mod mermaid;
pub mod html;

/// 文档格式枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DocumentFormat {
    Markdown,
    Mermaid,
    Html,
}

impl DocumentFormat {
    /// 从字符串解析格式
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "markdown" | "md" => Some(DocumentFormat::Markdown),
            "mermaid" | "mmd" => Some(DocumentFormat::Mermaid),
            "html" | "htm" => Some(DocumentFormat::Html),
            _ => None,
        }
    }

    /// 获取默认文件扩展名
    pub fn default_extension(&self) -> &str {
        match self {
            DocumentFormat::Markdown => "md",
            DocumentFormat::Mermaid => "mmd",
            DocumentFormat::Html => "html",
        }
    }
}

// 重新导出主要类型
pub use traits::DocumentGenerator;
pub use markdown::MarkdownGenerator;
pub use mermaid::MermaidGenerator;
pub use html::HtmlGenerator;
