// src/main.rs - crab-shell 入口文件
// crab-shell: PostgreSQL Schema 文档生成器

use clap::{Parser, Subcommand};

// 模块声明
mod cli;
mod config;
mod schema;
mod generator;
mod snapshot;
mod infra;
mod error;

use cli::{Cli, Commands};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 初始化日志
    tracing_subscriber::fmt::init();

    // 解析 CLI 参数
    let cli = Cli::parse();

    // 根据子命令分发处理
    match cli.command {
        Commands::Generate { .. } => {
            // TODO: 实现 generate 命令
            println!("Generate command - 待实现");
        }
        Commands::Snapshot { .. } => {
            // TODO: 实现 snapshot 命令
            println!("Snapshot command - 待实现");
        }
        Commands::Diff { .. } => {
            // TODO: 实现 diff 命令
            println!("Diff command - 待实现");
        }
    }

    Ok(())
}
