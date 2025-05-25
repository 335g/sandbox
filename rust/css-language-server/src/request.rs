use anyhow::Result;
use serde_json::json;

use crate::response::send_message;

pub async fn initialize(id: u64) -> Result<()> {
    send_message(
        json!({
            "jsonrpc": "2.0",
            "id": id,
            "result": {
                "capabilities": {}
            }
        })
    ).await?;
    
    Ok(())
}