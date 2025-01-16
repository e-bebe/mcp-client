use anyhow::Result;

mod client;
mod protocol;
mod transport;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let transport = transport::StdioTransport::new();
    let mut client = client::MCPClient::new(Box::new(transport));

    // サーバーに接続（初期化シーケンス）
    client.connect().await?;

    // GitHub リポジトリを検索
    let result = client
        .search_repositories("language:rust stars:>1000")
        .await?;
    println!("Search results: {:?}", result);

    Ok(())
}
