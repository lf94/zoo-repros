use reqwest::Upgraded;
use tokio_tungstenite::WebSocketStream;
use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::tungstenite;

type ResultTokio = Result<(), Box<dyn std::error::Error>>;

async fn msg(ws: &mut WebSocketStream<Upgraded>, text: &str) -> ResultTokio {
  let (mut tx, mut rx) = ws.split();

  tx.send(tungstenite::Message::Text(format!("{{
    \"type\": \"modeling_cmd_req\",
    \"cmd\": {{
      \"type\": \"start_path\"
    }},
    \"cmd_id\": \"{text}\"
  }}"))).await.unwrap();

  while let Some(msg) = rx.next().await {
    match msg.unwrap() {
      tungstenite::Message::Text(bytes) => println!("{bytes}"),
      _ => println!("unknown"),
    }
  }

  Ok(())
}

#[tokio::main]
async fn main() -> ResultTokio {
  let mut client = kittycad::Client::new_from_env();
  client.set_base_url("https://api.dev.zoo.dev");
  let upgraded = client.modeling().commands_ws(
    None, None, None, None, None, None, None
  ).await?;

  let mut ws = tokio_tungstenite::WebSocketStream::from_raw_socket(
    upgraded,
    tokio_tungstenite::tungstenite::protocol::Role::Client,
    None,
  ).await;

  msg(ws.by_ref(), "01234567890123456789012345678901").await?;

  Ok(())
}
