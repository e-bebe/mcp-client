use anyhow::Result;
use tracing::info;

mod client;
mod protocol;
mod transport;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_ansi(true)
        .with_target(true)
        .with_thread_ids(true)
        .with_line_number(true)
        .with_file(false)
        .with_level(true)
        .try_init()
        .expect("Failed to initialize logger");

    info!("Starting MCP client...");

    let command = "hoge";
    let transport = transport::StdioTransport::new(command)?;
    let mut client = client::MCPClient::new(Box::new(transport));

    info!("Connecting to MCP server...");
    client.connect().await?;

    info!("Searching repositories...");

    let result = client
        .search_repositories("language:rust stars:>1000")
        .await?;
    info!("Search results: {:?}", result);

    Ok(())
}
