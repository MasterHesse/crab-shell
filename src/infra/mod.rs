// src/infra/mod.rs - 基础设施层模块

pub mod postgres;
pub mod connection;

// 重新导出主要类型
pub use postgres::PostgresAdapter;
pub use connection::ConnectionManager;
