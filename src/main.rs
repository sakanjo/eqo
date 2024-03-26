use anyhow::Result;
use clap::Parser;
use eqo::cli::{self, Args};
use tap::prelude::*;

fn init_tracing() {
    tracing_subscriber::fmt()
        .pipe(|fmt| {
            #[cfg(debug_assertions)]
            {
                fmt.pretty()
                    .with_max_level(tracing::Level::DEBUG)
                    .with_thread_names(true)
                    .with_thread_ids(true)
                    .with_file(true)
                    .with_line_number(true)
            }

            #[cfg(not(debug_assertions))]
            fmt
        })
        .init();
}

#[tokio::main]
async fn main() -> Result<()> {
    init_tracing();

    cli::run(Args::parse().command).await
}
