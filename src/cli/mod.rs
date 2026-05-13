// src/cli/mod.rs - CLI 模块

pub mod commands;

use clap::{Parser, Subcommand};
use crate::error::Result;

/// crab-shell CLI 主结构
#[derive(Parser)]
#[command(name = "crab-shell")]
#[command(version, about = "PostgreSQL Schema 文档生成器", long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// 详细日志模式 (-vv 更详细)
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub verbose: u8,
}

/// 主命令枚举
#[derive(Subcommand)]
pub enum Commands {
    /// 生成数据库文档
    Generate {
        /// 数据库连接字符串
        #[arg(short, long, env = "DATABASE_URL")]
        url: Option<String>,

        /// 目标 schema 名称
        #[arg(short = 's', long, default_value = "public")]
        schema: String,

        /// 输出目录
        #[arg(short, long, default_value = "./docs")]
        output: String,

        /// 输出格式
        #[arg(short, long, default_value = "markdown")]
        format: String,

        /// 配置文件路径
        #[arg(short, long)]
        config: Option<String>,

        /// 包含视图
        #[arg(long)]
        include_views: bool,

        /// 排除的表（逗号分隔）
        #[arg(long)]
        exclude_tables: Option<String>,

        /// 每个图最大表数（超过则分组）
        #[arg(long, default_value = "30")]
        max_tables_per_diagram: usize,
    },

    /// Schema 快照管理
    Snapshot {
        #[command(subcommand)]
        action: SnapshotAction,
    },

    /// 对比两个快照
    Diff {
        /// 旧快照文件路径
        old: String,
        /// 新快照文件路径
        new: String,
        /// 输出文件路径
        #[arg(short, long)]
        output: Option<String>,
    },
}

/// 快照操作枚举
#[derive(Subcommand)]
pub enum SnapshotAction {
    /// 创建新快照
    Take {
        #[arg(short, long, env = "DATABASE_URL")]
        url: Option<String>,
        #[arg(short, long, default_value = "public")]
        schema: String,
        #[arg(short, long)]
        output: Option<String>,
    },
    /// 查看快照信息
    Info {
        snapshot: String,
    },
    /// 列出所有快照
    List {
        #[arg(short, long, default_value = "./snapshots")]
        dir: String,
    },
}
