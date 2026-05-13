// tests/schema_model_test.rs - Schema 模型单元测试
// 测试 DatabaseSchema、Table、Column 等领域模型的构造和方法

use crab_shell::schema::{
    DatabaseSchema, Table, Column, PrimaryKey, ForeignKey, Index, Constraint, View, EnumType
};

#[cfg(test)]
mod database_schema_tests {
    use super::*;

    #[test]
    fn should_create_database_schema_with_new() {
        let schema = DatabaseSchema::new("test_db", "public");

        assert_eq!(schema.database, "test_db");
        assert_eq!(schema.schema, "public");
        assert!(schema.tables.is_empty());
        assert!(schema.views.is_empty());
        assert!(schema.enums.is_empty());
    }

    #[test]
    fn should_initialize_generated_at_timestamp() {
        let before = chrono::Utc::now();
        let schema = DatabaseSchema::new("test_db", "public");
        let after = chrono::Utc::now();

        assert!(schema.generated_at >= before);
        assert!(schema.generated_at <= after);
    }

    #[test]
    fn should_return_correct_table_count() {
        let mut schema = DatabaseSchema::new("test_db", "public");
        assert_eq!(schema.table_count(), 0);

        schema.tables.push(Table::new("users"));
        assert_eq!(schema.table_count(), 1);

        schema.tables.push(Table::new("orders"));
        assert_eq!(schema.table_count(), 2);
    }

    #[test]
    fn should_return_correct_view_count() {
        let mut schema = DatabaseSchema::new("test_db", "public");
        assert_eq!(schema.view_count(), 0);

        schema.views.push(View {
            name: "active_users".to_string(),
            comment: None,
            definition: None,
            columns: vec![],
        });
        assert_eq!(schema.view_count(), 1);
    }

    #[test]
    fn should_find_table_by_name() {
        let mut schema = DatabaseSchema::new("test_db", "public");
        schema.tables.push(Table::new("users"));
        schema.tables.push(Table::new("orders"));

        let found = schema.find_table("users");
        assert!(found.is_some());
        assert_eq!(found.unwrap().name, "users");

        let not_found = schema.find_table("nonexistent");
        assert!(not_found.is_none());
    }
}

#[cfg(test)]
mod table_tests {
    use super::*;

    #[test]
    fn should_create_table_with_new() {
        let table = Table::new("users");

        assert_eq!(table.name, "users");
        assert!(table.comment.is_none());
        assert!(table.columns.is_empty());
        assert!(table.primary_key.is_none());
        assert!(table.foreign_keys.is_empty());
        assert!(table.indexes.is_empty());
        assert!(table.constraints.is_empty());
    }

    #[test]
    fn should_return_correct_column_count() {
        let mut table = Table::new("users");
        assert_eq!(table.column_count(), 0);

        table.columns.push(Column {
            name: "id".to_string(),
            data_type: "integer".to_string(),
            full_data_type: "integer".to_string(),
            nullable: false,
            default_value: None,
            comment: None,
            ordinal_position: 1,
            is_identity: false,
        });
        assert_eq!(table.column_count(), 1);
    }

    #[test]
    fn should_return_correct_index_count() {
        let mut table = Table::new("users");
        assert_eq!(table.index_count(), 0);

        table.indexes.push(Index {
            name: "idx_users_id".to_string(),
            columns: vec!["id".to_string()],
            is_unique: false,
            index_type: Some("btree".to_string()),
        });
        assert_eq!(table.index_count(), 1);
    }

    #[test]
    fn should_find_column_by_name() {
        let mut table = Table::new("users");
        table.columns.push(Column {
            name: "id".to_string(),
            data_type: "integer".to_string(),
            full_data_type: "integer".to_string(),
            nullable: false,
            default_value: None,
            comment: None,
            ordinal_position: 1,
            is_identity: false,
        });
        table.columns.push(Column {
            name: "username".to_string(),
            data_type: "varchar".to_string(),
            full_data_type: "varchar(50)".to_string(),
            nullable: false,
            default_value: None,
            comment: None,
            ordinal_position: 2,
            is_identity: false,
        });

        let found = table.find_column("username");
        assert!(found.is_some());
        assert_eq!(found.unwrap().name, "username");

        let not_found = table.find_column("nonexistent");
        assert!(not_found.is_none());
    }

    #[test]
    fn should_store_primary_key() {
        let mut table = Table::new("users");
        table.primary_key = Some(PrimaryKey {
            name: "users_pkey".to_string(),
            columns: vec!["id".to_string()],
        });

        assert!(table.primary_key.is_some());
        let pk = table.primary_key.unwrap();
        assert_eq!(pk.name, "users_pkey");
        assert_eq!(pk.columns, vec!["id"]);
    }

    #[test]
    fn should_store_foreign_keys() {
        let mut table = Table::new("orders");
        table.foreign_keys.push(ForeignKey {
            name: "orders_user_id_fkey".to_string(),
            columns: vec!["user_id".to_string()],
            referenced_table: "users".to_string(),
            referenced_columns: vec!["id".to_string()],
            on_update: Some("CASCADE".to_string()),
            on_delete: Some("CASCADE".to_string()),
        });

        assert_eq!(table.foreign_keys.len(), 1);
        let fk = &table.foreign_keys[0];
        assert_eq!(fk.name, "orders_user_id_fkey");
        assert_eq!(fk.referenced_table, "users");
    }

    #[test]
    fn should_store_indexes() {
        let mut table = Table::new("users");
        table.indexes.push(Index {
            name: "idx_users_email".to_string(),
            columns: vec!["email".to_string()],
            is_unique: true,
            index_type: Some("btree".to_string()),
        });

        assert_eq!(table.indexes.len(), 1);
        let idx = &table.indexes[0];
        assert!(idx.is_unique);
        assert_eq!(idx.columns, vec!["email"]);
    }
}

#[cfg(test)]
mod column_tests {
    use super::*;

    #[test]
    fn should_create_column_with_all_fields() {
        let column = Column {
            name: "id".to_string(),
            data_type: "integer".to_string(),
            full_data_type: "integer".to_string(),
            nullable: false,
            default_value: Some("nextval('seq')".to_string()),
            comment: Some("主键ID".to_string()),
            ordinal_position: 1,
            is_identity: true,
        };

        assert_eq!(column.name, "id");
        assert_eq!(column.data_type, "integer");
        assert!(!column.nullable);
        assert!(column.default_value.is_some());
        assert!(column.comment.is_some());
        assert_eq!(column.ordinal_position, 1);
        assert!(column.is_identity);
    }

    #[test]
    fn should_handle_nullable_column() {
        let column = Column {
            name: "email".to_string(),
            data_type: "varchar".to_string(),
            full_data_type: "varchar(100)".to_string(),
            nullable: true,
            default_value: None,
            comment: None,
            ordinal_position: 2,
            is_identity: false,
        };

        assert!(column.nullable);
        assert!(column.default_value.is_none());
        assert!(column.comment.is_none());
    }
}

#[cfg(test)]
mod primary_key_tests {
    use super::*;

    #[test]
    fn should_create_single_column_primary_key() {
        let pk = PrimaryKey {
            name: "users_pkey".to_string(),
            columns: vec!["id".to_string()],
        };

        assert_eq!(pk.name, "users_pkey");
        assert_eq!(pk.columns.len(), 1);
        assert_eq!(pk.columns[0], "id");
    }

    #[test]
    fn should_create_composite_primary_key() {
        let pk = PrimaryKey {
            name: "order_items_pkey".to_string(),
            columns: vec!["order_id".to_string(), "item_id".to_string()],
        };

        assert_eq!(pk.columns.len(), 2);
        assert_eq!(pk.columns, vec!["order_id", "item_id"]);
    }
}

#[cfg(test)]
mod foreign_key_tests {
    use super::*;

    #[test]
    fn should_create_foreign_key() {
        let fk = ForeignKey {
            name: "orders_user_id_fkey".to_string(),
            columns: vec!["user_id".to_string()],
            referenced_table: "users".to_string(),
            referenced_columns: vec!["id".to_string()],
            on_update: Some("CASCADE".to_string()),
            on_delete: Some("CASCADE".to_string()),
        };

        assert_eq!(fk.name, "orders_user_id_fkey");
        assert_eq!(fk.columns, vec!["user_id"]);
        assert_eq!(fk.referenced_table, "users");
        assert_eq!(fk.referenced_columns, vec!["id"]);
        assert_eq!(fk.on_update, Some("CASCADE".to_string()));
        assert_eq!(fk.on_delete, Some("CASCADE".to_string()));
    }

    #[test]
    fn should_handle_composite_foreign_key() {
        let fk = ForeignKey {
            name: "order_items_order_fkey".to_string(),
            columns: vec!["order_id".to_string(), "line_number".to_string()],
            referenced_table: "order_items".to_string(),
            referenced_columns: vec!["order_id".to_string(), "line_number".to_string()],
            on_update: None,
            on_delete: None,
        };

        assert_eq!(fk.columns.len(), 2);
        assert_eq!(fk.referenced_columns.len(), 2);
    }
}

#[cfg(test)]
mod index_tests {
    use super::*;

    #[test]
    fn should_create_unique_index() {
        let idx = Index {
            name: "users_email_key".to_string(),
            columns: vec!["email".to_string()],
            is_unique: true,
            index_type: Some("btree".to_string()),
        };

        assert_eq!(idx.name, "users_email_key");
        assert!(idx.is_unique);
        assert_eq!(idx.index_type, Some("btree".to_string()));
    }

    #[test]
    fn should_create_non_unique_index() {
        let idx = Index {
            name: "idx_orders_user_id".to_string(),
            columns: vec!["user_id".to_string()],
            is_unique: false,
            index_type: Some("btree".to_string()),
        };

        assert!(!idx.is_unique);
    }

    #[test]
    fn should_create_multi_column_index() {
        let idx = Index {
            name: "idx_orders_composite".to_string(),
            columns: vec!["user_id".to_string(), "created_at".to_string()],
            is_unique: false,
            index_type: Some("btree".to_string()),
        };

        assert_eq!(idx.columns.len(), 2);
    }

    #[test]
    fn should_handle_gin_index() {
        let idx = Index {
            name: "idx_products_tags".to_string(),
            columns: vec!["tags".to_string()],
            is_unique: false,
            index_type: Some("gin".to_string()),
        };

        assert_eq!(idx.index_type, Some("gin".to_string()));
    }
}

#[cfg(test)]
mod constraint_tests {
    use super::*;

    #[test]
    fn should_create_unique_constraint() {
        let constraint = Constraint {
            name: "users_email_unique".to_string(),
            constraint_type: "UNIQUE".to_string(),
            columns: vec!["email".to_string()],
            definition: Some("UNIQUE (email)".to_string()),
        };

        assert_eq!(constraint.name, "users_email_unique");
        assert_eq!(constraint.constraint_type, "UNIQUE");
        assert_eq!(constraint.columns, vec!["email".to_string()]);
    }

    #[test]
    fn should_create_check_constraint() {
        let constraint = Constraint {
            name: "orders_amount_check".to_string(),
            constraint_type: "CHECK".to_string(),
            columns: vec!["amount".to_string()],
            definition: Some("CHECK (amount >= 0)".to_string()),
        };

        assert_eq!(constraint.constraint_type, "CHECK");
        assert!(constraint.definition.is_some());
        assert_eq!(constraint.columns, vec!["amount".to_string()]);
    }
}

#[cfg(test)]
mod view_tests {
    use super::*;

    #[test]
    fn should_create_view() {
        let view = View {
            name: "active_users".to_string(),
            comment: Some("活跃用户视图".to_string()),
            definition: Some("SELECT * FROM users WHERE active = true".to_string()),
            columns: vec![],
        };

        assert_eq!(view.name, "active_users");
        assert!(view.comment.is_some());
        assert!(view.definition.is_some());
    }
}

#[cfg(test)]
mod enum_type_tests {
    use super::*;

    #[test]
    fn should_create_enum_type() {
        let enum_type = EnumType {
            name: "order_status".to_string(),
            values: vec![
                "PENDING".to_string(),
                "PAID".to_string(),
                "SHIPPED".to_string(),
                "DELIVERED".to_string(),
            ],
        };

        assert_eq!(enum_type.name, "order_status");
        assert_eq!(enum_type.values.len(), 4);
        assert_eq!(enum_type.values[0], "PENDING");
    }
}
