use anyhow::Result;
use dashmap::DashMap;
use derive_new::new;
use std::{net::SocketAddr, sync::Arc};
use tokio::{
    net::{TcpListener, TcpStream},
    sync::Mutex,
    time::sleep,
};
use uuid::Uuid;

use crate::{
    constants::HEARTBEAT_INTERVAL,
    shared::{ClientMessage, Id, Port, ServerMessage, Stream},
};

#[derive(Debug, new)]
pub struct Client {
    #[new(value = "Uuid::new_v4()")]
    id: Uuid,
    stream: Arc<Mutex<Stream>>,
}

#[derive(Debug, new)]
pub struct Server {
    pub host: String,
    pub port: Port,
    #[new(default)]
    pub conns: DashMap<Id, Vec<Client>>,
    pub quiet: bool,
}

impl Server {
    pub async fn run(self) -> Result<()> {
        let addr = SocketAddr::new(self.host.parse().unwrap(), self.port);
        let this = Arc::new(self);
        let listener = TcpListener::bind(addr).await?;

        if !this.quiet {
            tracing::info!("Listening on: {}", addr);
        }

        loop {
            let (stream, _) = listener.accept().await?;
            let this = this.clone();

            tokio::spawn(async move {
                this.handle_client(stream).await.unwrap();
            });
        }
    }

    pub async fn handle_client(&self, stream: TcpStream) -> Result<()> {
        let mut stream = Stream::new(stream);

        let message = match stream.recv::<ClientMessage>().await {
            Ok(t) => t,
            Err(_) => return Ok(()),
        };
        let message = match message {
            Some(t) => t,
            None => return Ok(()),
        };

        match message {
            ClientMessage::Listen { id } => {
                let stream = Arc::new(Mutex::new(stream));
                let client = Client::new(stream.clone());
                let uid = client.id;

                self.conns
                    .entry(id)
                    .or_insert_with(|| Vec::with_capacity(25))
                    .push(client);

                if !self.quiet {
                    tracing::info!("New listening id: {}", id);
                }

                loop {
                    sleep(HEARTBEAT_INTERVAL).await;

                    if self.check_and_remove(&stream, &id, &uid).await.is_err() {
                        return Ok(());
                    }
                }
            }
            ClientMessage::Run { id, secret } => {
                if let Some(users) = self.conns.get_mut(&id) {
                    for user in users.iter() {
                        let mut user = user.stream.lock().await;

                        let _ = (*user)
                            .send(&ServerMessage::Run {
                                secret: secret.clone(),
                            })
                            .await;
                    }
                };

                Ok(())
            }
        }
    }

    async fn check_and_remove(
        &self,
        stream: &Arc<Mutex<Stream>>,
        id: &u16,
        uid: &Uuid,
    ) -> Result<(), ()> {
        let mut stream = stream.lock().await;
        if stream.send(ServerMessage::Ping).await.is_err() {
            if !self.quiet {
                tracing::info!("{:?}", "Connection closed");
            }

            if let Some(mut users) = self.conns.get_mut(id) {
                if let Some(index) = users.iter().position(|x| x.id == *uid) {
                    users.remove(index);
                }
            }

            return Err(());
        }

        Ok(())
    }
}
