// tests/mermaid_generator_test.rs - MermaidGenerator TDD 测试
// 严格遵循 TDD 原则：Red → Green → Refactor

use crab_shell::generator::MermaidGenerator;
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
                ],
                primary_key: Some(PrimaryKey {
                    name: "users_pkey".to_string(),
                    columns: vec!["id".to_string()],
                }),
                foreign_keys: vec![],
                indexes: vec![],
                constraints: vec![],
            },
        ],
        views: vec![],
        enums: vec![],
        generated_at: chrono::Utc::now(),
    }
}

/// 测试辅助函数：创建订单表
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
        indexes: vec![],
        constraints: vec![],
    }
}

// ============================================================
// TDD Red 阶段：编写测试用例
// ============================================================

#[cfg(test)]
mod mermaid_generator_tests {
    use super::*;

    // --- 基础功能测试 ---

    #[test]
    fn should_generate_mermaid_code_block() {
        let schema = create_test_schema_with_users();
        let generator = MermaidGenerator::new();
        let result = generator.generate(&schema).unwrap();

        assert!(result.starts_with("```mermaid"));
        assert!(result.contains("erDiagram"));
        assert!(result.ends_with("```\n"));
    }

    #[test]
    fn should_return_correct_file_extension() {
        let generator = MermaidGenerator::new();
        assert_eq!(generator.file_extension(), "mmd");
    }

    #[test]
    fn should_return_correct_generator_name() {
        let generator = MermaidGenerator::new();
        assert_eq!(generator.name(), "Mermaid");
    }

    // --- 表定义测试 ---

    #[test]
    fn should_define_each_table() {
        let schema = create_test_schema_with_users();
        let generator = MermaidGenerator::new();
        let result = generator.generate(&schema).unwrap();

        assert!(result.contains("users {"));
    }

    #[test]
    fn should_list_table_columns() {
        let schema = create_test_schema_with_users();
        let generator = MermaidGenerator::new();
        let result = generator.generate(&schema).unwrap();

        assert!(result.contains("integer id"));
        assert!(result.contains("character varying username"));
    }

    #[test]
    fn should_mark_primary_key_columns() {
        let schema = create_test_schema_with_users();
        let generator = MermaidGenerator::new();
        let result = generator.generate(&schema).unwrap();

        // Mermaid 中 PK 标记
        assert!(result.contains("integer id PK"));
    }

    #[test]
    fn should_mark_foreign_key_columns() {
        let mut schema = create_test_schema_with_users();
        schema.tables.push(create_orders_table());
        let generator = MermaidGenerator::new();
        let result = generator.generate(&schema).unwrap();

        // Mermaid 中 FK 标记
        assert!(result.contains("integer user_id FK"));
    }

    #[test]
    fn should_close_table_definition() {
        let schema = create_test_schema_with_users();
        let generator = MermaidGenerator::new();
        let result = generator.generate(&schema).unwrap();

        assert!(result.contains("    }\n"));
    }

    // --- 关系测试 ---

    #[test]
    fn should_define_foreign_key_relationships() {
        let mut schema = create_test_schema_with_users();
        schema.tables.push(create_orders_table());
        let generator = MermaidGenerator::new();
        let result = generator.generate(&schema).unwrap();

        // users 到 orders 的关系
        assert!(result.contains("users ||--o{ orders"));
    }

    #[test]
    fn should_include_constraint_name_in_relationship() {
        let mut schema = create_test_schema_with_users();
        schema.tables.push(create_orders_table());
        let generator = MermaidGenerator::new();
        let result = generator.generate(&schema).unwrap();

        // 关系应该包含外键约束名
        assert!(result.contains("orders_user_id_fkey"));
    }

    #[test]
    fn should_not_show_relationships_for_tables_without_fk() {
        let schema = create_test_schema_with_users();
        let generator = MermaidGenerator::new();
        let result = generator.generate(&schema).unwrap();

        // users 表没有外键，不应该有关系到其他表
        // 检查没有多对一等关系符号
        assert!(!result.contains("||--o{"));
        assert!(!result.contains("o--o{"));
        assert!(!result.contains("}o--||"));
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
        let generator = MermaidGenerator::new();
        let result = generator.generate(&schema).unwrap();

        // 空 schema 应该仍然生成代码块框架
        assert!(result.contains("```mermaid"));
        assert!(result.contains("erDiagram"));
        assert!(result.contains("```\n"));
    }

    #[test]
    fn should_handle_multiple_tables_with_relationships() {
        let mut schema = create_test_schema_with_users();
        schema.tables.push(create_orders_table());
        let generator = MermaidGenerator::new();
        let result = generator.generate(&schema).unwrap();

        assert!(result.contains("users {"));
        assert!(result.contains("orders {"));
        // 应该有至少一个关系定义
        assert!(result.contains("||--o{"));
    }

    #[test]
    fn should_handle_table_without_primary_key() {
        let schema = DatabaseSchema {
            database: "test_db".to_string(),
            schema: "public".to_string(),
            tables: vec![Table {
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
            }],
            views: vec![],
            enums: vec![],
            generated_at: chrono::Utc::now(),
        };
        let generator = MermaidGenerator::new();
        let result = generator.generate(&schema).unwrap();

        // 没有 PK 的列不应该有 PK 或 FK 标记
        assert!(result.contains("text name"));
        assert!(!result.contains("text name PK"));
    }

    // --- Mermaid 语法正确性测试 ---

    #[test]
    fn should_produce_valid_mermaid_syntax() {
        let mut schema = create_test_schema_with_users();
        schema.tables.push(create_orders_table());
        let generator = MermaidGenerator::new();
        let result = generator.generate(&schema).unwrap();

        // 验证基本语法结构
        assert!(result.starts_with("```mermaid\n"));
        assert!(result.contains("erDiagram\n"));
        assert!(result.ends_with("```\n"));

        // 每个表定义应该以 { 开头，以 } 结尾
        assert!(result.contains("users {"));
        assert!(result.contains("orders {"));

        // 关系语法应该是 "table ||--o{ table"
        assert!(result.contains("users ||--o{ orders"));
    }

    #[test]
    fn should_not_have_duplicate_relationships() {
        // 创建一个有自引用的表（不应该有重复关系）
        let schema = DatabaseSchema {
            database: "test_db".to_string(),
            schema: "public".to_string(),
            tables: vec![Table {
                name: "categories".to_string(),
                comment: Some("分类表".to_string()),
                columns: vec![
                    Column {
                        name: "id".to_string(),
                        data_type: "integer".to_string(),
                        full_data_type: "integer".to_string(),
                        nullable: false,
                        default_value: None,
                        comment: Some("分类ID".to_string()),
                        ordinal_position: 1,
                        is_identity: true,
                    },
                    Column {
                        name: "parent_id".to_string(),
                        data_type: "integer".to_string(),
                        full_data_type: "integer".to_string(),
                        nullable: true,
                        default_value: None,
                        comment: Some("父分类ID".to_string()),
                        ordinal_position: 2,
                        is_identity: false,
                    },
                ],
                primary_key: Some(PrimaryKey {
                    name: "categories_pkey".to_string(),
                    columns: vec!["id".to_string()],
                }),
                foreign_keys: vec![
                    ForeignKey {
                        name: "categories_parent_id_fkey".to_string(),
                        columns: vec!["parent_id".to_string()],
                        referenced_table: "categories".to_string(),
                        referenced_columns: vec!["id".to_string()],
                        on_update: None,
                        on_delete: None,
                    },
                ],
                indexes: vec![],
                constraints: vec![],
            }],
            views: vec![],
            enums: vec![],
            generated_at: chrono::Utc::now(),
        };
        let generator = MermaidGenerator::new();
        let result = generator.generate(&schema).unwrap();

        // 自引用应该只出现一次
        let self_ref_count = result.matches("categories ||--o{ categories").count();
        assert_eq!(self_ref_count, 1, "自引用关系应该只出现一次");
    }

    // --- generate_to_file 测试 ---

    #[test]
    fn should_generate_to_file() {
        let schema = create_test_schema_with_users();
        let generator = MermaidGenerator::new();
        let temp_dir = std::env::temp_dir();
        let output_path = temp_dir.join("test_output.mmd");

        generator.generate_to_file(&schema, &output_path).unwrap();

        // 验证文件已创建
        assert!(output_path.exists());

        // 验证文件内容
        let content = std::fs::read_to_string(&output_path).unwrap();
        assert!(content.contains("```mermaid"));
        assert!(content.contains("erDiagram"));

        // 清理
        std::fs::remove_file(output_path).ok();
    }
}
