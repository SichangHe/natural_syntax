use std::io::stderr;

use anyhow::Result;
use natural_syntax_ls::run_part_of_speech_ls;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_ansi(false)
        .with_writer(stderr)
        .with_env_filter(EnvFilter::from_default_env())
        .init();
    run_part_of_speech_ls().await
}
