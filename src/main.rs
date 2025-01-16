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
    let transport = transport::StdioTransport::new("./bin/mcp-github-server")?;
    let mut client = client::MCPClient::new(Box::new(transport));

    info!("Connecting to MCP server...");
    client.connect().await?;

    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    info!("Received input: {}", input);

    // 読み込んだJSONをパースして実行
    if let Ok(value) = serde_json::from_str::<serde_json::Value>(&input) {
        if let Some(params) = value.get("params") {
            if let Some(tool_params) = params.get("params") {
                if let Some(query) = tool_params.get("query") {
                    info!("Searching repositories with query: {}", query);
                    let result = client
                        .search_repositories(query.as_str().unwrap_or(""))
                        .await?;
                    println!("{}", serde_json::to_string_pretty(&result)?);
                }
            }
        }
    }

    Ok(())
}
