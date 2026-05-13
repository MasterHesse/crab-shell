// src/cli/commands.rs - 命令处理实现

use crate::error::{CrabShellError, Result};
use crate::generator::{MarkdownGenerator, MermaidGenerator, DocumentGenerator};
use crate::infra::PostgresAdapter;
use crate::schema::SchemaReader;
use crate::snapshot::{DefaultSnapshotManager, SnapshotManager, SchemaDiffer, DiffType};
use std::path::Path;

/// 从连接 URL 解析 host 和 port
fn parse_connection_info(url: &str) -> (String, u16) {
    if let Ok(parsed) = url::Url::parse(url) {
        let host = parsed.host_str().unwrap_or("localhost").to_string();
        let port = parsed.port().unwrap_or(5432);
        (host, port)
    } else {
        ("localhost".to_string(), 5432)
    }
}

/// 处理 generate 命令
pub async fn handle_generate(
    url: Option<&str>,
    schema: &str,
    output: &str,
    format: &str,
    _include_views: bool,
    _exclude_tables: Option<&str>,
) -> Result<()> {
    // 获取数据库连接 URL
    let database_url = url
        .map(String::from)
        .or_else(|| std::env::var("DATABASE_URL").ok())
        .ok_or_else(|| CrabShellError::ConfigError(
            "数据库连接 URL 未提供，请通过 --url 参数或 DATABASE_URL 环境变量指定".to_string()
        ))?;

    println!("正在连接到数据库...");
    println!("  URL: {}", mask_password(&database_url));

    // 解析连接信息用于错误报告
    let (host, port) = parse_connection_info(&database_url);

    // 创建 PostgreSQL 适配器
    let adapter = PostgresAdapter::new(&database_url)
        .await
        .map_err(|e| CrabShellError::ConnectionFailed {
            host,
            port,
            reason: e.to_string(),
            help: "请检查数据库连接参数是否正确".to_string(),
        })?;

    // 测试连接
    adapter.test_connection().await?;
    println!("  ✓ 连接成功");

    // 读取 Schema
    println!("正在读取 Schema '{}'...", schema);
    let db_schema = adapter.read_schema(schema).await
        .map_err(|e| CrabShellError::DatabaseError(e.to_string()))?;

    println!("  ✓ 读取到 {} 个表", db_schema.table_count());

    // 创建输出目录
    let output_path = Path::new(output);
    if !output_path.exists() {
        std::fs::create_dir_all(output_path)
            .map_err(|e| CrabShellError::OutputWriteFailed {
                path: output.to_string(),
                reason: e.to_string(),
            })?;
    }

    // 根据格式生成文档
    match format.to_lowercase().as_str() {
        "markdown" | "md" => {
            generate_markdown(&db_schema, output_path).await?;
        }
        "mermaid" | "mmd" => {
            generate_mermaid(&db_schema, output_path).await?;
        }
        "json" => {
            generate_json_snapshot(&db_schema, output_path)?;
        }
        "all" => {
            generate_markdown(&db_schema, output_path).await?;
            generate_mermaid(&db_schema, output_path).await?;
            generate_json_snapshot(&db_schema, output_path)?;
        }
        _ => {
            return Err(CrabShellError::ConfigError(
                format!("不支持的输出格式: {}，支持的格式: markdown, mermaid, json, all", format)
            ));
        }
    }

    println!("\n✓ 文档生成完成！");
    println!("  输出目录: {}", output);

    Ok(())
}

/// 生成 Markdown 文档
async fn generate_markdown(schema: &crate::schema::DatabaseSchema, output_dir: &Path) -> Result<()> {
    println!("正在生成 Markdown 文档...");

    let generator = MarkdownGenerator::new();
    let content = generator.generate(schema)?;

    let filename = format!("{}-schema.md", schema.database);
    let filepath = output_dir.join(&filename);

    std::fs::write(&filepath, &content)
        .map_err(|e| CrabShellError::OutputWriteFailed {
            path: filepath.to_string_lossy().to_string(),
            reason: e.to_string(),
        })?;

    let size = std::fs::metadata(&filepath)
        .map(|m| m.len())
        .unwrap_or(0);
    println!("  ✓ {}", filename);
    println!("    大小: {} bytes", size);

    Ok(())
}

/// 生成 Mermaid ER 图
async fn generate_mermaid(schema: &crate::schema::DatabaseSchema, output_dir: &Path) -> Result<()> {
    println!("正在生成 Mermaid ER 图...");

    let generator = MermaidGenerator::new();
    let content = generator.generate(schema)?;

    let filename = format!("{}-er.mmd", schema.database);
    let filepath = output_dir.join(&filename);

    std::fs::write(&filepath, &content)
        .map_err(|e| CrabShellError::OutputWriteFailed {
            path: filepath.to_string_lossy().to_string(),
            reason: e.to_string(),
        })?;

    let size = std::fs::metadata(&filepath)
        .map(|m| m.len())
        .unwrap_or(0);
    println!("  ✓ {}", filename);
    println!("    大小: {} bytes", size);

    Ok(())
}

/// 生成 JSON 快照
fn generate_json_snapshot(schema: &crate::schema::DatabaseSchema, output_dir: &Path) -> Result<()> {
    println!("正在生成 JSON 快照...");

    let manager = DefaultSnapshotManager::new();
    let filename = format!("{}-snapshot.json", schema.database);
    let filepath = output_dir.join(&filename);

    manager.save(schema, &filepath)?;

    let size = std::fs::metadata(&filepath)
        .map(|m| m.len())
        .unwrap_or(0);
    println!("  ✓ {}", filename);
    println!("    大小: {} bytes", size);

    Ok(())
}

/// 处理 snapshot 命令
pub async fn handle_snapshot(
    action: &crate::cli::SnapshotAction,
) -> Result<()> {
    match action {
        crate::cli::SnapshotAction::Take { url, schema, output } => {
            handle_snapshot_take(url.as_deref(), schema, output.as_deref()).await
        }
        crate::cli::SnapshotAction::Info { snapshot } => {
            handle_snapshot_info(snapshot)
        }
        crate::cli::SnapshotAction::List { dir } => {
            handle_snapshot_list(dir)
        }
    }
}

/// 创建快照
async fn handle_snapshot_take(
    url: Option<&str>,
    schema: &str,
    output: Option<&str>,
) -> Result<()> {
    // 获取数据库连接 URL
    let database_url = url
        .map(String::from)
        .or_else(|| std::env::var("DATABASE_URL").ok())
        .ok_or_else(|| CrabShellError::ConfigError(
            "数据库连接 URL 未提供".to_string()
        ))?;

    println!("正在连接到数据库...");
    let (host, port) = parse_connection_info(&database_url);
    let adapter = PostgresAdapter::new(&database_url).await
        .map_err(|e| CrabShellError::ConnectionFailed {
            host,
            port,
            reason: e.to_string(),
            help: "请检查数据库连接参数".to_string(),
        })?;

    println!("正在读取 Schema '{}'...", schema);
    let db_schema = adapter.read_schema(schema).await
        .map_err(|e| CrabShellError::DatabaseError(e.to_string()))?;

    let manager = DefaultSnapshotManager::new();
    let output_str = output
        .map(String::from)
        .unwrap_or_else(|| format!("./{}-{}.json", db_schema.database, db_schema.schema));
    let output_path = Path::new(&output_str);

    manager.save(&db_schema, output_path)?;

    println!("✓ 快照已保存到: {}", output_path.display());

    Ok(())
}

/// 查看快照信息
fn handle_snapshot_info(snapshot_path: &str) -> Result<()> {
    let manager = DefaultSnapshotManager::new();
    let path = Path::new(snapshot_path);

    let schema = manager.load(path)
        .map_err(|e| CrabShellError::SnapshotError(e.to_string()))?;

    println!("快照信息:");
    println!("  数据库: {}", schema.database);
    println!("  Schema: {}", schema.schema);
    println!("  表数量: {}", schema.table_count());
    println!("  视图数量: {}", schema.view_count());
    println!("  生成时间: {}", schema.generated_at);

    Ok(())
}

/// 列出快照
fn handle_snapshot_list(dir: &str) -> Result<()> {
    let path = Path::new(dir);

    if !path.exists() {
        println!("目录不存在: {}", dir);
        return Ok(());
    }

    let entries = std::fs::read_dir(path)
        .map_err(CrabShellError::IoError)?;

    let snapshots: Vec<_> = entries
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map(|s| s == "json").unwrap_or(false))
        .collect();

    if snapshots.is_empty() {
        println!("目录中没有找到快照文件: {}", dir);
    } else {
        println!("快照列表 ({}):", snapshots.len());
        for entry in snapshots {
            let filename = entry.file_name();
            let filename_str = filename.to_string_lossy();
            let modified = entry.metadata()
                .and_then(|m| m.modified());
            let modified_str = modified
                .map(|t| {
                    let datetime: chrono::DateTime<chrono::Local> = t.into();
                    datetime.format("%Y-%m-%d %H:%M:%S").to_string()
                })
                .unwrap_or_else(|_| "未知".to_string());
            println!("  - {} ({})", filename_str, modified_str);
        }
    }

    Ok(())
}

/// 处理 diff 命令
pub fn handle_diff(
    old_path: &str,
    new_path: &str,
    output: Option<&str>,
) -> Result<()> {
    let manager = DefaultSnapshotManager::new();

    println!("加载快照...");

    let old_schema = manager.load(Path::new(old_path))
        .map_err(|e| CrabShellError::SnapshotError(
            format!("无法加载旧快照: {}", e)
        ))?;

    let new_schema = manager.load(Path::new(new_path))
        .map_err(|e| CrabShellError::SnapshotError(
            format!("无法加载新快照: {}", e)
        ))?;

    println!("  ✓ 旧快照: {} ({} 个表)", old_schema.database, old_schema.table_count());
    println!("  ✓ 新快照: {} ({} 个表)", new_schema.database, new_schema.table_count());

    // 使用 SchemaDiffer 进行对比
    let differ = SchemaDiffer::new();
    let diff = differ.diff(&old_schema, &new_schema);

    // 生成差异报告
    let report = differ.generate_markdown_report(&diff);

    // 输出到文件或控制台
    if let Some(output_path) = output {
        std::fs::write(Path::new(output_path), &report)
            .map_err(|e| CrabShellError::OutputWriteFailed {
                path: output_path.to_string(),
                reason: e.to_string(),
            })?;
        println!("\n✓ 差异报告已保存到: {}", output_path);
    } else {
        println!("\n{}", report);
    }

    // 总结
    let added_tables = diff.table_diffs.iter().filter(|t| t.diff_type == DiffType::Added).count();
    let removed_tables = diff.table_diffs.iter().filter(|t| t.diff_type == DiffType::Removed).count();
    let modified_tables = diff.table_diffs.iter().filter(|t| t.diff_type == DiffType::Modified).count();

    println!("\n差异摘要:");
    println!("  新增表: {}", added_tables);
    println!("  删除表: {}", removed_tables);
    println!("  修改表: {}", modified_tables);

    Ok(())
}

/// 隐藏密码
fn mask_password(url: &str) -> String {
    // 简单的密码隐藏逻辑
    if url.contains("@") {
        // postgres://user:password@host/db -> postgres://user:***@host/db
        let parts: Vec<&str> = url.splitn(2, ':').collect();
        if parts.len() == 2 && !parts[1].starts_with('/') {
            let rest = parts[1];
            if let Some(at_pos) = rest.find('@') {
                return format!("{}:***@{}", parts[0], &rest[at_pos + 1..]);
            }
        }
    }
    url.to_string()
}
