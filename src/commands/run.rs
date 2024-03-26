use anyhow::{Context, Result};
use tokio::net::TcpStream;

use derive_new::new;

use crate::shared::{self, ClientMessage, Id, Port};

#[derive(Debug, new)]
pub struct Run {
    pub host: String,
    pub port: Port,
    pub id: Id,
    pub secret: Option<String>,
}

impl Run {
    pub async fn connect(&self) -> Result<TcpStream> {
        TcpStream::connect(format!("{}:{}", self.host, self.port))
            .await
            .with_context(|| format!("Couldn't connect to {}:{}", self.host, self.port))
    }

    pub async fn run(&self) -> Result<()> {
        let mut stream = shared::Stream::new(self.connect().await?);
        stream
            .send(&ClientMessage::Run {
                id: self.id,
                secret: self.secret.clone(),
            })
            .await?;

        Ok(())
    }
}
