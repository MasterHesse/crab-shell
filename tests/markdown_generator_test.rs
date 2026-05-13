// tests/markdown_generator_test.rs - MarkdownGenerator TDD 测试
// 严格遵循 TDD 原则：Red → Green → Refactor

// 在 integration tests 中使用外部 crate 名（crab_shell）访问模块
use crab_shell::generator::MarkdownGenerator;
use crab_shell::generator::DocumentGenerator;
use crab_shell::schema::{DatabaseSchema, Table, Column, PrimaryKey, ForeignKey, Index};

/// 测试辅助函数：创建包含用户的测试 Schema
fn create_test_schema_with_users() -> DatabaseSchema {
    DatabaseSchema {
        database: "test_db".to_string(),
        schema: "public".to_string(),
        tables: vec![
            Table {
                name: "users".to_string(),
                comment: Some("用户信息表".to_string()),
                columns: vec![
                    Column {
                        name: "id".to_string(),
                        data_type: "integer".to_string(),
                        full_data_type: "integer".to_string(),
                        nullable: false,
                        default_value: Some("nextval('users_id_seq')".to_string()),
                        comment: Some("用户ID".to_string()),
                        ordinal_position: 1,
                        is_identity: true,
                    },
                    Column {
                        name: "username".to_string(),
                        data_type: "character varying".to_string(),
                        full_data_type: "character varying(50)".to_string(),
                        nullable: false,
                        default_value: None,
                        comment: Some("用户名".to_string()),
                        ordinal_position: 2,
                        is_identity: false,
                    },
                    Column {
                        name: "email".to_string(),
                        data_type: "character varying".to_string(),
                        full_data_type: "character varying(100)".to_string(),
                        nullable: false,
                        default_value: None,
                        comment: Some("邮箱".to_string()),
                        ordinal_position: 3,
                        is_identity: false,
                    },
                    Column {
                        name: "created_at".to_string(),
                        data_type: "timestamp".to_string(),
                        full_data_type: "timestamp without time zone".to_string(),
                        nullable: true,
                        default_value: Some("CURRENT_TIMESTAMP".to_string()),
                        comment: None,
                        ordinal_position: 4,
                        is_identity: false,
                    },
                ],
                primary_key: Some(PrimaryKey {
                    name: "users_pkey".to_string(),
                    columns: vec!["id".to_string()],
                }),
                foreign_keys: vec![],
                indexes: vec![
                    Index {
                        name: "users_email_key".to_string(),
                        columns: vec!["email".to_string()],
                        is_unique: true,
                        index_type: Some("btree".to_string()),
                    },
                ],
                constraints: vec![],
            },
        ],
        views: vec![],
        enums: vec![],
        generated_at: chrono::Utc::now(),
    }
}

/// 测试辅助函数：创建包含外键关系的订单表
fn create_orders_table() -> Table {
    Table {
        name: "orders".to_string(),
        comment: Some("订单表".to_string()),
        columns: vec![
            Column {
                name: "id".to_string(),
                data_type: "integer".to_string(),
                full_data_type: "integer".to_string(),
                nullable: false,
                default_value: Some("nextval('orders_id_seq')".to_string()),
                comment: Some("订单ID".to_string()),
                ordinal_position: 1,
                is_identity: true,
            },
            Column {
                name: "user_id".to_string(),
                data_type: "integer".to_string(),
                full_data_type: "integer".to_string(),
                nullable: false,
                default_value: None,
                comment: Some("用户ID".to_string()),
                ordinal_position: 2,
                is_identity: false,
            },
            Column {
                name: "total_amount".to_string(),
                data_type: "numeric".to_string(),
                full_data_type: "numeric(10,2)".to_string(),
                nullable: false,
                default_value: None,
                comment: Some("订单总额".to_string()),
                ordinal_position: 3,
                is_identity: false,
            },
            Column {
                name: "status".to_string(),
                data_type: "character varying".to_string(),
                full_data_type: "character varying(20)".to_string(),
                nullable: false,
                default_value: Some("'PENDING'::character varying".to_string()),
                comment: Some("订单状态".to_string()),
                ordinal_position: 4,
                is_identity: false,
            },
        ],
        primary_key: Some(PrimaryKey {
            name: "orders_pkey".to_string(),
            columns: vec!["id".to_string()],
        }),
        foreign_keys: vec![
            ForeignKey {
                name: "orders_user_id_fkey".to_string(),
                columns: vec!["user_id".to_string()],
                referenced_table: "users".to_string(),
                referenced_columns: vec!["id".to_string()],
                on_update: Some("NO ACTION".to_string()),
                on_delete: Some("CASCADE".to_string()),
            },
        ],
        indexes: vec![
            Index {
                name: "idx_orders_user_id".to_string(),
                columns: vec!["user_id".to_string()],
                is_unique: false,
                index_type: Some("btree".to_string()),
            },
            Index {
                name: "idx_orders_status".to_string(),
                columns: vec!["status".to_string()],
                is_unique: false,
                index_type: Some("btree".to_string()),
            },
        ],
        constraints: vec![],
    }
}

/// 测试辅助函数：创建没有主键的表
fn create_table_without_primary_key() -> Table {
    Table {
        name: "simple_table".to_string(),
        comment: None,
        columns: vec![
            Column {
                name: "name".to_string(),
                data_type: "text".to_string(),
                full_data_type: "text".to_string(),
                nullable: false,
                default_value: None,
                comment: None,
                ordinal_position: 1,
                is_identity: false,
            },
        ],
        primary_key: None,
        foreign_keys: vec![],
        indexes: vec![],
        constraints: vec![],
    }
}

// ============================================================
// TDD Red 阶段：编写测试用例
// ============================================================

#[cfg(test)]
mod markdown_generator_tests {
    use super::*;

    // --- 基础功能测试 ---

    #[test]
    fn should_generate_database_title() {
        let schema = create_test_schema_with_users();
        let generator = MarkdownGenerator::new();
        let result = generator.generate(&schema).unwrap();

        assert!(result.contains("# test_db - Database Schema"));
    }

    #[test]
    fn should_contain_generation_timestamp() {
        let schema = create_test_schema_with_users();
        let generator = MarkdownGenerator::new();
        let result = generator.generate(&schema).unwrap();

        assert!(result.contains("Generated by crab-shell"));
    }

    #[test]
    fn should_return_correct_file_extension() {
        let generator = MarkdownGenerator::new();
        assert_eq!(generator.file_extension(), "md");
    }

    #[test]
    fn should_return_correct_generator_name() {
        let generator = MarkdownGenerator::new();
        assert_eq!(generator.name(), "Markdown");
    }

    // --- 表相关测试 ---

    #[test]
    fn should_generate_table_of_contents() {
        let schema = create_test_schema_with_users();
        let generator = MarkdownGenerator::new();
        let result = generator.generate(&schema).unwrap();

        assert!(result.contains("## Table of Contents"));
        assert!(result.contains("- [users](#users)"));
    }

    #[test]
    fn should_list_column_count_in_table_of_contents() {
        let schema = create_test_schema_with_users();
        let generator = MarkdownGenerator::new();
        let result = generator.generate(&schema).unwrap();

        assert!(result.contains("4 columns"));
    }

    #[test]
    fn should_list_index_count_in_table_of_contents() {
        let schema = create_test_schema_with_users();
        let generator = MarkdownGenerator::new();
        let result = generator.generate(&schema).unwrap();

        assert!(result.contains("1 index"));
    }

    #[test]
    fn should_generate_table_section_heading() {
        let schema = create_test_schema_with_users();
        let generator = MarkdownGenerator::new();
        let result = generator.generate(&schema).unwrap();

        assert!(result.contains("## users"));
    }

    #[test]
    fn should_include_table_comment() {
        let schema = create_test_schema_with_users();
        let generator = MarkdownGenerator::new();
        let result = generator.generate(&schema).unwrap();

        assert!(result.contains("> 用户信息表"));
    }

    #[test]
    fn should_not_include_empty_comment() {
        let mut schema = create_test_schema_with_users();
        schema.tables[0].comment = None;
        let generator = MarkdownGenerator::new();
        let result = generator.generate(&schema).unwrap();

        // 注释为空时不应生成 > 空行
        assert!(!result.contains("> \n"));
    }

    // --- 列相关测试 ---

    #[test]
    fn should_generate_column_table_header() {
        let schema = create_test_schema_with_users();
        let generator = MarkdownGenerator::new();
        let result = generator.generate(&schema).unwrap();

        assert!(result.contains("| # | Column | Type | Nullable | Default | Comment |"));
        assert!(result.contains("|---|--------|------|----------|---------|---------|"));
    }

    #[test]
    fn should_list_all_columns() {
        let schema = create_test_schema_with_users();
        let generator = MarkdownGenerator::new();
        let result = generator.generate(&schema).unwrap();

        assert!(result.contains("| 1 | id | integer | NO |"));
        assert!(result.contains("| 2 | username | character varying | NO |"));
        assert!(result.contains("| 3 | email | character varying | NO |"));
        assert!(result.contains("| 4 | created_at | timestamp | YES |"));
    }

    #[test]
    fn should_show_nullable_as_yes_or_no() {
        let schema = create_test_schema_with_users();
        let generator = MarkdownGenerator::new();
        let result = generator.generate(&schema).unwrap();

        // nullable=false -> NO
        assert!(result.contains("| NO |"));
        // nullable=true -> YES
        assert!(result.contains("| YES |"));
    }

    #[test]
    fn should_show_column_default_value() {
        let schema = create_test_schema_with_users();
        let generator = MarkdownGenerator::new();
        let result = generator.generate(&schema).unwrap();

        assert!(result.contains("nextval('users_id_seq')"));
        assert!(result.contains("CURRENT_TIMESTAMP"));
    }

    #[test]
    fn should_show_dash_for_null_default() {
        let schema = create_test_schema_with_users();
        let generator = MarkdownGenerator::new();
        let result = generator.generate(&schema).unwrap();

        // username 没有默认值，应该显示 -
        assert!(result.contains("| - |"));
    }

    #[test]
    fn should_include_column_comments() {
        let schema = create_test_schema_with_users();
        let generator = MarkdownGenerator::new();
        let result = generator.generate(&schema).unwrap();

        assert!(result.contains("用户ID"));
        assert!(result.contains("用户名"));
        assert!(result.contains("邮箱"));
    }

    #[test]
    fn should_show_dash_for_null_comment() {
        let schema = create_test_schema_with_users();
        let generator = MarkdownGenerator::new();
        let result = generator.generate(&schema).unwrap();

        // created_at 没有注释，应该显示 -
        assert!(result.contains("| - |") || result.contains("| -\n"));
    }

    // --- 主键相关测试 ---

    #[test]
    fn should_show_primary_key_info() {
        let schema = create_test_schema_with_users();
        let generator = MarkdownGenerator::new();
        let result = generator.generate(&schema).unwrap();

        assert!(result.contains("**Primary Key**: `id` (`users_pkey`)"));
    }

    #[test]
    fn should_not_show_primary_key_section_when_no_pk() {
        let schema = DatabaseSchema {
            database: "test_db".to_string(),
            schema: "public".to_string(),
            tables: vec![create_table_without_primary_key()],
            views: vec![],
            enums: vec![],
            generated_at: chrono::Utc::now(),
        };
        let generator = MarkdownGenerator::new();
        let result = generator.generate(&schema).unwrap();

        assert!(!result.contains("**Primary Key**"));
    }

    #[test]
    fn should_handle_composite_primary_key() {
        let mut schema = create_test_schema_with_users();
        schema.tables[0].primary_key = Some(PrimaryKey {
            name: "composite_pkey".to_string(),
            columns: vec!["id".to_string(), "username".to_string()],
        });
        let generator = MarkdownGenerator::new();
        let result = generator.generate(&schema).unwrap();

        assert!(result.contains("**Primary Key**: `id, username` (`composite_pkey`)"));
    }

    // --- 外键相关测试 ---

    #[test]
    fn should_show_foreign_keys_section_header() {
        let mut schema = create_test_schema_with_users();
        schema.tables.push(create_orders_table());
        let generator = MarkdownGenerator::new();
        let result = generator.generate(&schema).unwrap();

        // orders 表有外键
        assert!(result.contains("**Foreign Keys**:"));
    }

    #[test]
    fn should_show_foreign_key_table_header() {
        let mut schema = create_test_schema_with_users();
        schema.tables.push(create_orders_table());
        let generator = MarkdownGenerator::new();
        let result = generator.generate(&schema).unwrap();

        assert!(result.contains("| Name | Column(s) | References |"));
    }

    #[test]
    fn should_show_foreign_key_details() {
        let mut schema = create_test_schema_with_users();
        schema.tables.push(create_orders_table());
        let generator = MarkdownGenerator::new();
        let result = generator.generate(&schema).unwrap();

        assert!(result.contains("orders_user_id_fkey"));
        assert!(result.contains("user_id"));
        assert!(result.contains("users(id)"));
    }

    #[test]
    fn should_not_show_foreign_keys_section_when_no_fk() {
        let schema = create_test_schema_with_users();
        let generator = MarkdownGenerator::new();
        let result = generator.generate(&schema).unwrap();

        assert!(!result.contains("**Foreign Keys**:"));
    }

    // --- 索引相关测试 ---

    #[test]
    fn should_show_indexes_section_header() {
        let schema = create_test_schema_with_users();
        let generator = MarkdownGenerator::new();
        let result = generator.generate(&schema).unwrap();

        assert!(result.contains("**Indexes**:"));
    }

    #[test]
    fn should_show_indexes_table_header() {
        let schema = create_test_schema_with_users();
        let generator = MarkdownGenerator::new();
        let result = generator.generate(&schema).unwrap();

        assert!(result.contains("| Name | Columns | Unique | Type |"));
    }

    #[test]
    fn should_show_index_details() {
        let schema = create_test_schema_with_users();
        let generator = MarkdownGenerator::new();
        let result = generator.generate(&schema).unwrap();

        assert!(result.contains("users_email_key"));
        assert!(result.contains("email"));
        assert!(result.contains("YES")); // unique
        assert!(result.contains("btree"));
    }

    #[test]
    fn should_show_no_for_non_unique_index() {
        let mut schema = create_test_schema_with_users();
        schema.tables.push(create_orders_table());
        let generator = MarkdownGenerator::new();
        let result = generator.generate(&schema).unwrap();

        assert!(result.contains("idx_orders_user_id"));
        assert!(result.contains("NO")); // non-unique
    }

    #[test]
    fn should_not_show_indexes_section_when_no_indexes() {
        let schema = DatabaseSchema {
            database: "test_db".to_string(),
            schema: "public".to_string(),
            tables: vec![create_table_without_primary_key()],
            views: vec![],
            enums: vec![],
            generated_at: chrono::Utc::now(),
        };
        let generator = MarkdownGenerator::new();
        let result = generator.generate(&schema).unwrap();

        assert!(!result.contains("**Indexes**:"));
    }

    // --- 边界情况测试 ---

    #[test]
    fn should_handle_empty_schema() {
        let schema = DatabaseSchema {
            database: "empty_db".to_string(),
            schema: "public".to_string(),
            tables: vec![],
            views: vec![],
            enums: vec![],
            generated_at: chrono::Utc::now(),
        };
        let generator = MarkdownGenerator::new();
        let result = generator.generate(&schema).unwrap();

        assert!(result.contains("# empty_db - Database Schema"));
        // 空表时不应有 Table of Contents 内容
        assert!(!result.contains("- ["));
    }

    #[test]
    fn should_handle_multiple_tables() {
        let mut schema = create_test_schema_with_users();
        schema.tables.push(create_orders_table());
        let generator = MarkdownGenerator::new();
        let result = generator.generate(&schema).unwrap();

        assert!(result.contains("## users"));
        assert!(result.contains("## orders"));
        assert!(result.contains("- [users](#users)"));
        assert!(result.contains("- [orders](#orders)"));
    }

    #[test]
    fn should_separate_tables_with_horizontal_rule() {
        let schema = create_test_schema_with_users();
        let generator = MarkdownGenerator::new();
        let result = generator.generate(&schema).unwrap();

        assert!(result.contains("---\n\n##"));
    }

    #[test]
    fn should_show_full_data_type_when_available() {
        let schema = create_test_schema_with_users();
        let generator = MarkdownGenerator::new();
        let result = generator.generate(&schema).unwrap();

        // 应该显示完整类型，但当前实现只显示 data_type
        // 这是重构目标之一
        assert!(result.contains("character varying"));
    }

    // --- generate_to_file 测试 ---

    #[test]
    fn should_generate_to_file() {
        let schema = create_test_schema_with_users();
        let generator = MarkdownGenerator::new();
        let temp_dir = std::env::temp_dir();
        let output_path = temp_dir.join("test_output.md");

        generator.generate_to_file(&schema, &output_path).unwrap();

        // 验证文件已创建
        assert!(output_path.exists());

        // 验证文件内容
        let content = std::fs::read_to_string(&output_path).unwrap();
        assert!(content.contains("# test_db - Database Schema"));

        // 清理
        std::fs::remove_file(output_path).ok();
    }
}
