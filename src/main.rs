use anyhow::Result;
use tracing::info;

mod client;
mod protocol;
mod transport;

#[tokio::main]
async fn main() -> Result<()> {
    info!("Starting MCP client...");

    let transport = transport::StdioTransport::new();
    let mut client = client::MCPClient::new(Box::new(transport));

    info!("Connecting to MCP server...");
    client.connect().await?;

    info!("Searching repositories...");

    // GitHub リポジトリを検索
    let result = client
        .search_repositories("language:rust stars:>1000")
        .await?;
    info!("Search results: {:?}", result);

    Ok(())
}
