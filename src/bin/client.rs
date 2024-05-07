use futures_util::stream::StreamExt;
use futures_util::SinkExt;
use http::Uri;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio_websockets::{ClientBuilder, Message};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let uri = "ws://127.0.0.1:8080";
    let (mut ws_stream, _) = ClientBuilder::<'_>::from_uri(Uri::from_static(uri)).connect().await?;

    let stdin = tokio::io::stdin();
    let mut stdin = BufReader::new(stdin).lines();

    loop {
        tokio::select! {
            incoming = ws_stream.next() => {
                match incoming {
                    Some(Ok(msg)) => {
                        if let Some(text) = msg.as_text() {
                            println!("Server broadcast: {}", text);
                        }
                    }
                    Some(Err(err)) => return Err(err.into()),
                    None => return Ok(()),
                }
            }
            res = stdin.next_line() => {
                match res {
                    Ok(None) => return Ok(()),
                    Ok(Some(line)) => {
                        if let Err(err) = ws_stream.send(Message::text(line)).await {
                            return Err(err.into());
                        }
                    }
                    Err(err) => return Err(err.into()),
                }
            }
        }
    }
}
