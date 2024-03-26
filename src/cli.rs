use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::{
    commands::{listen::Listen, run::Run, server::Server},
    constants::{DEFAULT_HOST, DEFAULT_PORT},
    shared::{Id, Port},
};

#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct Args {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Run the server
    Server {
        /// The host to listen on
        #[arg(long, default_value = DEFAULT_HOST, env = "EQO_HOST")]
        host: String,

        /// The port to listen on
        #[arg(long, default_value = DEFAULT_PORT, env = "EQO_PORT")]
        port: Port,

        /// Whether to suppress output
        #[arg(short, long, default_value = "false")]
        quiet: bool,
    },
    /// Listen for a command to run
    Listen {
        /// the command to run
        #[arg()]
        command: String,

        /// the id of the listener
        #[arg(long, default_value = "1")]
        id: Id,

        /// The host to listen on
        #[arg(long, default_value = DEFAULT_HOST, env = "EQO_HOST")]
        host: String,

        /// The port to listen on
        #[arg(long, default_value = DEFAULT_PORT, env = "EQO_PORT")]
        port: Port,

        /// Whether to suppress output
        #[arg(short, long, default_value = "false")]
        quiet: bool,

        /// Whether to clear the screen before running the command
        #[arg(short, long, default_value = "false")]
        clear: bool,

        /// Whether to only run the command once
        #[arg(long, default_value = "false")]
        once: bool,

        /// The secret to use for authentication
        #[arg(long, env = "EQO_SECRET", hide_env_values = true)]
        secret: Option<String>,
    },
    /// Run a command
    Run {
        /// the id of the listener
        #[arg(long, default_value = "1")]
        id: Id,

        /// The host to listen on
        #[arg(long, default_value = DEFAULT_HOST, env = "EQO_HOST")]
        host: String,

        /// The port to listen on
        #[arg(long, default_value = DEFAULT_PORT, env = "EQO_PORT")]
        port: Port,

        /// The secret to use for authentication
        #[arg(long, env = "EQO_SECRET", hide_env_values = true)]
        secret: Option<String>,
    },
}

pub async fn run(command: Command) -> Result<()> {
    match command {
        Command::Server { host, port, quiet } => Server::new(host, port, quiet).run().await,
        Command::Listen {
            host,
            port,
            command,
            id,
            quiet,
            clear,
            once,
            secret,
        } => {
            Listen::new(host, port, command, id, quiet, clear, once, secret)
                .run()
                .await
        }
        Command::Run {
            host,
            port,
            id,
            secret,
        } => Run::new(host, port, id, secret).run().await,
    }
}
