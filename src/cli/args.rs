use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "social-auto")]
#[command(about = "Social media automation tool", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Account management
    Account {
        #[command(subcommand)]
        action: AccountAction,
    },
    /// Task management
    Task {
        #[command(subcommand)]
        action: TaskAction,
    },
    /// Content management
    Content {
        #[command(subcommand)]
        action: ContentAction,
    },
    /// Start the service
    Start,
    /// Stop the service
    Stop,
    /// Show service status
    Status,
    /// Start web interface
    Web {
        #[arg(short, long, default_value = "8080")]
        port: u16,
    },
}

#[derive(Subcommand)]
pub enum AccountAction {
    /// Add a new account
    Add {
        #[arg(long)]
        platform: String,
        #[arg(long)]
        username: String,
        #[arg(long)]
        proxy: Option<String>,
    },
    /// List all accounts
    List,
    /// Show account status
    Status {
        #[arg(value_name = "ACCOUNT_ID")]
        id: String,
    },
    /// Remove an account
    Remove {
        #[arg(value_name = "ACCOUNT_ID")]
        id: String,
    },
}

#[derive(Subcommand)]
pub enum TaskAction {
    /// Create a new task
    Create {
        #[arg(long)]
        task_type: String,
        #[arg(long)]
        account: String,
        #[arg(long)]
        content: Option<String>,
    },
    /// List tasks
    List {
        #[arg(long)]
        status: Option<String>,
    },
    /// Cancel a task
    Cancel {
        #[arg(value_name = "TASK_ID")]
        id: String,
    },
}

#[derive(Subcommand)]
pub enum ContentAction {
    /// Add new content
    Add {
        #[arg(long)]
        file: String,
        #[arg(long)]
        platform: String,
    },
    /// List all content
    List,
    /// Remove content
    Remove {
        #[arg(value_name = "CONTENT_ID")]
        id: String,
    },
}
