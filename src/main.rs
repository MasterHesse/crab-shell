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
        Commands::Generate { 
            url, 
            schema, 
            output, 
            format, 
            config: _,
            include_views, 
            exclude_tables,
            max_tables_per_diagram: _,
        } => {
            if let Err(e) = cli::commands::handle_generate(
                url.as_deref(),
                &schema,
                &output,
                &format,
                include_views,
                exclude_tables.as_deref(),
            ).await {
                eprintln!("错误: {}", e);
                std::process::exit(1);
            }
        }
        Commands::Snapshot { action } => {
            if let Err(e) = cli::commands::handle_snapshot(&action).await {
                eprintln!("错误: {}", e);
                std::process::exit(1);
            }
        }
        Commands::Diff { old, new, output } => {
            if let Err(e) = cli::commands::handle_diff(&old, &new, output.as_deref()) {
                eprintln!("错误: {}", e);
                std::process::exit(1);
            }
        }
    }

    Ok(())
}
