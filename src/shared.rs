use anyhow::Result;
use futures::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio_util::codec::{Framed, LinesCodec};

use serde::{de::DeserializeOwned, Deserialize, Serialize};

pub type Id = u16;
pub type Port = u16;

#[derive(Debug, Serialize, Deserialize)]
pub enum ClientMessage {
    Listen { id: Port },
    Run { id: Port, secret: Option<String> },
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ServerMessage {
    Run { secret: Option<String> },
    Ping,
}

#[derive(Debug)]
pub struct Stream(pub Framed<TcpStream, LinesCodec>);

impl Stream {
    pub fn new(stream: TcpStream) -> Self {
        let framed = Framed::new(stream, LinesCodec::new());

        Self(framed)
    }

    pub async fn send<T: Serialize>(&mut self, msg: T) -> Result<()> {
        let json = serde_json::to_string(&msg)?;
        self.0.send(json).await?;

        Ok(())
    }

    pub async fn recv<T: DeserializeOwned>(&mut self) -> Result<Option<T>> {
        match self.0.next().await {
            Some(Ok(line)) => {
                let serialized_object = serde_json::from_str(&line)?;

                Ok(serialized_object)
            }
            _ => Ok(None),
        }
    }
}
