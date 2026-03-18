use clap::Parser;
use social_auto::cli::{Cli, handle_command};
use tracing_subscriber;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info"))
        )
        .init();

    // Parse CLI arguments
    let cli = Cli::parse();

    // Handle command
    handle_command(cli.command).await?;

    Ok(())
}
