use poem_mcpserver::{McpServer, Tools, stdio::stdio, tool::Text};
use uuid::Uuid;

struct IdGenerator;

#[Tools]
impl IdGenerator {
    async fn generate(&self) -> Text<String> {
        Text(Uuid::new_v4().to_string())
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    stdio(McpServer::new().tools(IdGenerator)).await
}
