
mod request;
mod response;
mod notification;

use response::{send_error_response, send_method_not_found_response};
use tokio::{fs::OpenOptions, io::AsyncWriteExt};

use anyhow::Result;

async fn save(msg: &str) -> Result<()> {
    let path = std::env::var("LOG_PATH")?;
    let mut f = OpenOptions::new()
        .append(true)
        .open(path).await?;
    f.write_all(msg.as_bytes()).await?;
    f.write_all("\r\n".as_bytes()).await?;
    Ok(())
}

#[derive(Debug)]
struct Message {
    content_length: usize,
    body: serde_json::Value,
}

impl Message {
    fn new(body: serde_json::Value) -> Self {
        let content_length = body.to_string().as_bytes().len();
        Self {
            content_length,
            body,
        }
    }
}

impl ToString for Message {
    fn to_string(&self) -> String {
        let mut s = String::new();
        s.push_str(&format!("Content-Length: {}\r\n", self.content_length));
        s.push_str("\r\n");
        s.push_str(&serde_json::to_string(&self.body).expect("valid json-rpc"));
        s
    }
}

async fn parse_message() -> Result<Message> {
    let mut is_ready = false;
    let mut content_length = 0;
    let mut body = vec![];

    loop {
        let stdin = std::io::stdin();
        let mut msg = String::new();
        stdin.read_line(&mut msg)?;

        if !is_ready {
            if msg == "\r\n" {
                is_ready = true;
                continue;
            }
            let mut kvs = msg.split(":");
            let Some(k) = kvs.next() else { continue };
            let k = k.trim();

            if k == "Content-Length" {
                content_length = kvs.next()
                    .unwrap_or_default()
                    .trim()
                    .parse::<usize>()
                    .expect("content length is number");
            }
        } else {
            let mut msg = msg.as_bytes().to_vec();
            body.append(&mut msg);

            let body_length = body.len();
            if 0 < content_length && content_length <= body_length {
                let loaded = body.splice(..content_length, []).collect::<Vec<_>>();
                let value = serde_json::from_slice::<serde_json::Value>(&loaded)?;
                let msg = Message::new(value);

                return Ok(msg)
            }
        }
    }
}

async fn dispatch(msg: Message) -> Result<()> {
    let id = msg.body.as_object()
        .and_then(|x| x.get("id"))
        .and_then(|x| x.as_u64());
    let method = msg.body.as_object()
        .and_then(|x| x.get("method"))
        .and_then(|x| x.as_str().map(|s| s.to_string()));

    match (id, method) {
        (None, None) => {
            send_error_response(None, -32600, "Received an invalid request").await?;
        }
        (None, Some(method)) => {
            match method.as_str() {
                "initialized" => {
                    save("got initialized notification").await?;
                    notification::initialized().await?;
                }
                _ => {}
            }
        }
        (Some(id), None) => {}
        (Some(id), Some(method)) => {
            match method.as_str() {
                "initialize" => {
                    save("got initialize method").await?;
                    request::initialize(id).await?;
                }
                _ => {
                    send_method_not_found_response(Some(id as u32), method).await?;
                }
            }
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    loop {
        match parse_message().await {
            Ok(msg) => {
                save(msg.to_string().as_str()).await?;
                dispatch(msg).await?;
            }
            Err(_) => {
                send_error_response(None, -32700, "received an invalid JSON").await?;
            }
        }
    }
}
