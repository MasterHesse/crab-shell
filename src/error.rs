// src/error.rs - 错误类型定义
// 使用 thiserror 定义精确错误类型（库层）
// 使用 miette 提供漂亮的错误报告（展示层）
// 使用 anyhow 简化错误传播（应用层）

use miette::{Diagnostic, SourceSpan};
use thiserror::Error;
use std::io;

/// crab-shell 主错误类型
#[derive(Error, Debug, Diagnostic)]
pub enum CrabShellError {
    #[error("数据库连接失败: {host}:{port} - {reason}")]
    #[diagnostic(code(crab_shell::db_connection))]
    ConnectionFailed {
        host: String,
        port: u16,
        reason: String,
        #[help]
        help: String,
    },

    #[error("数据库权限不足: {0}")]
    #[diagnostic(code(crab_shell::permission))]
    PermissionDenied(String),

    #[error("Schema '{0}' 不存在")]
    #[diagnostic(code(crab_shell::schema_not_found))]
    SchemaNotFound(String),

    #[error("无法写入输出目录: {path} - {reason}")]
    #[diagnostic(code(crab_shell::io_error))]
    OutputWriteFailed {
        path: String,
        reason: String,
    },

    #[error("配置文件格式错误: {0}")]
    #[diagnostic(code(crab_shell::config_error))]
    ConfigError(String),

    #[error("快照加载失败: {0}")]
    #[diagnostic(code(crab_shell::snapshot_error))]
    SnapshotError(String),

    #[error("数据库错误: {0}")]
    #[diagnostic(code(crab_shell::db_error))]
    DatabaseError(String),

    #[error("IO 错误: {0}")]
    #[diagnostic(code(crab_shell::io_error))]
    IoError(#[from] io::Error),

    #[error("JSON 错误: {0}")]
    #[diagnostic(code(crab_shell::json_error))]
    JsonError(#[from] serde_json::Error),

    #[error("YAML 错误: {0}")]
    #[diagnostic(code(crab_shell::yaml_error))]
    YamlError(#[from] serde_yaml::Error),
}

/// 从 tokio_postgres::Error 转换
impl From<tokio_postgres::Error> for CrabShellError {
    fn from(err: tokio_postgres::Error) -> Self {
        CrabShellError::DatabaseError(err.to_string())
    }
}

/// 结果类型别名
pub type Result<T> = std::result::Result<T, CrabShellError>;
