// src/config.rs - 配置文件解析

use serde::{Deserialize, Serialize};
use std::path::Path;
use crate::error::Result;

/// 主配置结构
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    pub database: DatabaseConfig,
    pub output: OutputConfig,
    pub options: OptionsConfig,
}

/// 数据库配置
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DatabaseConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(default = "default_schema")]
    pub schema: String,
    #[serde(default = "default_connect_timeout")]
    pub connect_timeout: u64,
    #[serde(default = "default_max_retries")]
    pub max_retries: u32,
}

fn default_schema() -> String {
    "public".to_string()
}

fn default_connect_timeout() -> u64 {
    30
}

fn default_max_retries() -> u32 {
    3
}

/// 输出配置
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OutputConfig {
    #[serde(default = "default_format")]
    pub format: String,
    #[serde(default = "default_path")]
    pub path: String,
    #[serde(default = "default_filename_template")]
    pub filename_template: String,
}

fn default_format() -> String {
    "markdown".to_string()
}

fn default_path() -> String {
    "./docs".to_string()
}

fn default_filename_template() -> String {
    "{database}-schema-{date}".to_string()
}

/// 选项配置
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OptionsConfig {
    #[serde(default)]
    pub include_views: bool,
    #[serde(default)]
    pub include_enums: bool,
    #[serde(default = "default_true")]
    pub include_comments: bool,
    #[serde(default)]
    pub exclude_tables: Vec<String>,
    #[serde(default)]
    pub exclude_schemas: Vec<String>,
    #[serde(default = "default_max_tables")]
    pub max_tables_per_diagram: usize,
}

fn default_true() -> bool {
    true
}

fn default_max_tables() -> usize {
    30
}

impl Default for Config {
    fn default() -> Self {
        Self {
            database: DatabaseConfig {
                url: None,
                host: None,
                port: None,
                database: None,
                username: None,
                password: None,
                schema: default_schema(),
                connect_timeout: default_connect_timeout(),
                max_retries: default_max_retries(),
            },
            output: OutputConfig {
                format: default_format(),
                path: default_path(),
                filename_template: default_filename_template(),
            },
            options: OptionsConfig {
                include_views: false,
                include_enums: false,
                include_comments: true,
                exclude_tables: vec![],
                exclude_schemas: vec![],
                max_tables_per_diagram: default_max_tables(),
            },
        }
    }
}

impl Config {
    /// 从文件加载配置
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: Config = serde_yaml::from_str(&content)?;
        Ok(config)
    }

    /// 保存到文件
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let content = serde_yaml::to_string(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }
}
