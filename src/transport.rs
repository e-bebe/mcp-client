use anyhow::Result;
use async_trait::async_trait;
use std::process::Stdio;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::process::{Child, ChildStdin, ChildStdout};
use tokio::sync::Mutex;
use tracing::{info, warn};

#[async_trait]
pub trait Transport: Send + Sync {
    async fn read_message(&self) -> Result<String>;
    async fn write_message(&self, message: &str) -> Result<()>;
}

pub struct StdioTransport {
    reader: Arc<Mutex<BufReader<ChildStdout>>>,
    writer: Arc<Mutex<ChildStdin>>,
    _child: Child,
}

impl StdioTransport {
    pub fn new(server_path: &str) -> Result<Self> {
        let mut child = tokio::process::Command::new(server_path)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()?;

        let stdin = child.stdin.take().unwrap();
        let stdout = child.stdout.take().unwrap();

        Ok(Self {
            reader: Arc::new(Mutex::new(BufReader::new(stdout))),
            writer: Arc::new(Mutex::new(stdin)),
            _child: child,
        })
    }
}

#[async_trait]
impl Transport for StdioTransport {
    async fn read_message(&self) -> Result<String> {
        let mut reader = self.reader.lock().await;
        loop {
            let mut line = String::new();
            let bytes_read = reader.read_line(&mut line).await?;

            if bytes_read == 0 {
                warn!("Received empty message from server");
                anyhow::bail!("Server connection closed");
            }

            info!("Raw message received: {}", line);

            // JSONっぽい行を見つけたら返す
            if line.trim().starts_with('{') {
                info!("Found JSON message: {}", line);
                return Ok(line);
            }

            // ログメッセージなのでスキップ
            info!("Skipping non-JSON line: {}", line);
            continue;
        }
    }

    async fn write_message(&self, message: &str) -> Result<()> {
        let mut writer = self.writer.lock().await;
        info!("Sending message: {}", message);
        writer.write_all(message.as_bytes()).await?;
        writer.write_all(b"\n").await?;
        writer.flush().await?;
        Ok(())
    }
}
