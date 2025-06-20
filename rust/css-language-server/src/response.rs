use std::io::Write as _;

use serde_json::json;
use anyhow::Result;

use crate::Message;

pub async fn log_message(msg: &str) -> Result<()> {
    let s = json!({
        "jsonrpc": "2.0",
        "method": "window/logMessage",
        "params": {
            "type": 3,
            "message": msg,
        },
    });
    send_message(s).await
}

pub async fn send_message(msg: serde_json::Value) -> Result<()> {
    let msg = Message::new(msg);

    let stdout = std::io::stdout();
    let mut locked = stdout.lock();
    writeln!(locked, "{}", msg.to_string())?;
    Ok(())
}

pub async fn send_error_response(id: Option<u32>, code: i64, message: &str) -> Result<()> {
    let val = json!({
        "jsonrpc": "2.0",
        "id": id,
        "error": {
            "code": code,
            "message": message,
        },
    });
    send_message(val).await
}

pub async fn send_method_not_found_response(id: Option<u32>, method: String) -> Result<()> {
    let val = json!({
        "jsonrpc": "2.0",
        "id": id,
        "error": {
            "code": -32600,
            "message": format!("`{method}` is not supported")
        }
    });
    send_message(val).await
}