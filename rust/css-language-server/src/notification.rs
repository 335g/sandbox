
use anyhow::Result;

use crate::response::log_message;

pub async fn initialized() -> Result<()> {
    log_message("initialized").await?;
    Ok(())
}