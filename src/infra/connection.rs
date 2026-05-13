// src/infra/connection.rs - 连接池管理

use tokio_postgres::{Client, NoTls};
use tokio_postgres::config::Config;
use crate::error::{Result, CrabShellError};

/// 连接管理器
pub struct ConnectionManager {
    connection_string: String,
}

impl ConnectionManager {
    /// 创建新的连接管理器
    pub fn new(connection_string: &str) -> Self {
        Self {
            connection_string: connection_string.to_string(),
        }
    }

    /// 获取连接
    pub async fn connect(&self) -> Result<Client> {
        let (client, connection) = tokio_postgres::connect(&self.connection_string, NoTls).await
            .map_err(|e| CrabShellError::ConnectionFailed {
                host: "localhost".to_string(),
                port: 5432,
                reason: e.to_string(),
                help: "请检查数据库连接参数".to_string(),
            })?;

        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("Database connection error: {}", e);
            }
        });

        Ok(client)
    }
}
