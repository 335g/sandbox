use anyhow::Result;
use serde_json::json;

use crate::{response::send_message, save};

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
    save("send initialize response").await?;
    Ok(())
}