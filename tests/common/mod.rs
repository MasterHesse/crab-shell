// tests/common/mod.rs - 测试辅助模块

// 测试辅助函数和共享代码

/// 创建测试用的 DatabaseSchema
pub fn create_test_schema() -> crate::schema::DatabaseSchema {
    crate::schema::DatabaseSchema {
        database: "testdb".to_string(),
        schema: "public".to_string(),
        tables: vec![
            create_test_table("users"),
            create_test_table("orders"),
        ],
        views: vec![],
        enums: vec![],
        generated_at: chrono::Utc::now(),
    }
}

/// 创建测试用的 Table
pub fn create_test_table(name: &str) -> crate::schema::Table {
    crate::schema::Table {
        name: name.to_string(),
        comment: Some(format!("测试表: {}", name)),
        columns: vec![
            create_test_column("id", "integer", false),
            create_test_column("name", "varchar", false),
        ],
        primary_key: Some(crate::schema::PrimaryKey {
            name: format!("{}_pkey", name),
            columns: vec!["id".to_string()],
        }),
        foreign_keys: vec![],
        indexes: vec![],
        constraints: vec![],
    }
}

/// 创建测试用的 Column
pub fn create_test_column(name: &str, data_type: &str, nullable: bool) -> crate::schema::Column {
    crate::schema::Column {
        name: name.to_string(),
        data_type: data_type.to_string(),
        full_data_type: data_type.to_string(),
        nullable,
        default_value: None,
        comment: Some(format!("测试列: {}", name)),
        ordinal_position: 1,
        is_identity: false,
    }
}

/// 初始化测试日志
pub fn init_test_logger() {
    let _ = tracing_subscriber::fmt()
        .with_test_writer()
        .with_max_level(tracing::Level::DEBUG)
        .try_init();
}
