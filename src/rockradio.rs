use reqwest::Url;
use serde_json::Value;
use tokio_stream::StreamExt;
use tokio_tungstenite::connect_async;

use crate::radio::WSSConfig;

pub fn listen_wss(wss_config: WSSConfig) {
    let (_, wss) = wss_config;
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(start_listening_wss(wss))
}

async fn start_listening_wss(wss_link: &str) {
    loop {
        let url = Url::parse(wss_link).expect("Failed to parse WebSocket link");
    
        let (mut ws_stream, _) = connect_async(url)
            .await
            .expect("Error connecting to the server");
    
        while let Some(msg) = ws_stream.next().await {
            match msg {
                Ok(msg) => {
                    if msg.is_text() || msg.is_binary() {
                        let text = msg.into_text().unwrap();
                        let data = match serde_json::from_str::<Value>(&text) {
                            Ok(value) => handle_json_value(value),
                            Err(e) => {
                                eprintln!("Error parsing JSON: {}", e);
                                continue;
                            }
                        };
                        if let Some((artist, title)) = data {
                            println!("StreamTitle:\t{} - {}", artist, title);
                        }
                    }
                }
                Err(e) => eprintln!("Error during receiving a message: {}", e),
            }
        }
    }
}

fn handle_json_value(value: Value) -> Option<(String, String)> {
    if let Some(item) = value
        .get("data")
        .and_then(|d| d.get("items"))
        .and_then(|items| items.get(0)) // Get the first item from the array
    {
        if let (Some(title), Some(artist)) = (
            item.get("title").and_then(|t| t.as_str()),
            item.get("artist").and_then(|a| a.as_str()),
        ) {
            if !title.is_empty() && !artist.is_empty() {
                return Some((artist.to_owned(), title.to_owned()));
            }
        }
    }
    None
}