use anyhow::Result;
use tracing::info;

mod client;
mod protocol;
mod transport;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_ansi(true) // ANSIカラーを有効化
        .with_target(true) // モジュールパスを表示
        .with_thread_ids(true) // スレッドIDを表示
        .with_line_number(true) // 行番号を表示
        .with_file(false) // ファイル名を表示
        .with_level(true) // ログレベルを表示
        .try_init()
        .expect("Failed to initialize logger");

    info!("Starting MCP client...");

    let command = "hoge";
    let transport = transport::StdioTransport::new(command)?;
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
