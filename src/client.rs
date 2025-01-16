use crate::protocol::{Notification, Request, Response};
use crate::transport::Transport;
use anyhow::Result;
use serde::de::DeserializeOwned;
use serde_json::{json, Value};
use std::sync::atomic::{AtomicU64, Ordering};
use tracing::info;

pub struct MCPClient {
    transport: Box<dyn Transport>,
    initialized: bool,
    request_id: AtomicU64,
}

impl MCPClient {
    pub fn new(transport: Box<dyn Transport>) -> Self {
        Self {
            transport,
            initialized: false,
            request_id: AtomicU64::new(1),
        }
    }

    pub async fn connect(&mut self) -> Result<()> {
        info!("Initializing MCP connection...");

        // initialize リクエストを送信
        let init_request = Request {
            jsonrpc: "2.0".to_string(),
            method: "initialize".to_string(),
            params: Some(json!({
                "version": "1.0",
                "capabilities": {
                    "tools": {}
                }
            })),
            id: Some(json!(self.next_request_id())),
        };

        self.transport
            .write_message(&serde_json::to_string(&init_request)?)
            .await?;
        let response = self.transport.read_message().await?;
        info!("Initialize response: {}", response);

        // initialized 通知を送信
        let init_notification = Notification {
            jsonrpc: "2.0".to_string(),
            method: "initialized".to_string(),
            params: Some(json!({})),
        };

        self.transport
            .write_message(&serde_json::to_string(&init_notification)?)
            .await?;
        self.initialized = true;
        info!("MCP connection initialized");

        Ok(())
    }

    fn next_request_id(&self) -> u64 {
        self.request_id.fetch_add(1, Ordering::Relaxed)
    }

    async fn call_tool<T: DeserializeOwned>(&self, tool_name: &str, params: Value) -> Result<T> {
        if !self.initialized {
            anyhow::bail!("Client not initialized");
        }

        let request = Request {
            jsonrpc: "2.0".to_string(),
            method: "callTool".to_string(),
            params: Some(json!({
                "name": tool_name,
                "params": params
            })),
            id: Some(json!(self.next_request_id())),
        };

        self.transport
            .write_message(&serde_json::to_string(&request)?)
            .await?;
        let response = self.transport.read_message().await?;
        let response: Response<T> = serde_json::from_str(&response)?;

        match response.result {
            Some(result) => Ok(result),
            None => {
                if let Some(error) = response.error {
                    anyhow::bail!("Tool call failed: {}", error.message);
                }
                anyhow::bail!("Tool call failed with no result or error");
            }
        }
    }

    pub async fn search_repositories(&self, query: &str) -> Result<Value> {
        self.call_tool(
            "search_repositories",
            json!({
                "query": query
            }),
        )
        .await
    }
}
