use anyhow::{Context, Result};
use colored::*;
use subprocess::{Exec, ExitStatus};
use tokio::net::TcpStream;

use derive_new::new;

use crate::shared::{ClientMessage, Id, Port, ServerMessage, Stream};

#[allow(clippy::too_many_arguments)]
#[derive(new, Debug)]
pub struct Listen {
    pub host: String,
    pub port: Port,
    pub command: String,
    pub id: Id,
    pub quiet: bool,
    pub clear: bool,
    pub once: bool,
    pub secret: Option<String>,
}

impl Listen {
    pub async fn connect(&self) -> Result<TcpStream> {
        TcpStream::connect(format!("{}:{}", self.host, self.port))
            .await
            .with_context(|| format!("Couldn't connect to {}:{}", self.host, self.port))
    }

    pub fn clear_output(&self) {
        clearscreen::clear().unwrap();
    }

    pub fn run_command(&self) {
        match Exec::shell(&self.command).join() {
            Ok(exit_status) => {
                if let ExitStatus::Exited(code) = exit_status {
                    println!(
                        "\n{} returned exit code {}",
                        "[*]".purple(),
                        code.to_string().bold()
                    );
                }
            }
            Err(e) => println!("{e}"),
        };
    }

    fn match_secret(&self, secret: &Option<String>) -> bool {
        if let Some(this_secret) = &self.secret {
            if let Some(secret) = secret {
                return this_secret == secret;
            }

            return false;
        }

        true
    }
    pub async fn run(&self) -> Result<()> {
        let mut stream = Stream::new(self.connect().await?);
        stream.send(&ClientMessage::Listen { id: self.id }).await?;

        if !self.quiet {
            tracing::info!("Listening on id: {}", self.id);
        }

        loop {
            let message = match stream.recv::<ServerMessage>().await {
                Ok(t) => t,
                Err(_) => continue,
            };
            let message = match message {
                Some(t) => t,
                None => return Ok(()),
            };

            match message {
                ServerMessage::Run { secret } => {
                    if !self.match_secret(&secret) {
                        continue;
                    }

                    if self.clear {
                        self.clear_output();
                    }

                    #[cfg(debug_assertions)]
                    {
                        if !self.quiet {
                            tracing::info!("Running command: '{}'", self.command);
                        }
                    }

                    self.run_command();

                    if self.once {
                        return Ok(());
                    }
                }
                ServerMessage::Ping => {}
            }
        }
    }
}
