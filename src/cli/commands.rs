use crate::cli::args::{AccountAction, ContentAction, TaskAction};
use crate::cli::Commands;
use crate::config::load_config;
use crate::core::{AccountManager, TaskScheduler};
use crate::db::{initialize_database, Database, DbOperations};
use crate::models::{Account, AccountStatus, Content, ContentType, Task, TaskStatus, TaskType};
use anyhow::Result;
use chrono::Utc;
use std::path::PathBuf;
use std::sync::Arc;
use uuid::Uuid;

pub async fn handle_command(command: Commands) -> Result<()> {
    match command {
        Commands::Account { action } => handle_account_action(action).await,
        Commands::Task { action } => handle_task_action(action).await,
        Commands::Content { action } => handle_content_action(action).await,
        Commands::Start => handle_start().await,
        Commands::Stop => handle_stop().await,
        Commands::Status => handle_status().await,
        Commands::Web { port } => handle_web(port).await,
    }
}

async fn handle_account_action(action: AccountAction) -> Result<()> {
    let config = load_config("config.yaml")?;
    let db_path = PathBuf::from(&config.system.data_dir).join("database.db");
    let conn = initialize_database(&db_path)?;
    let db = Database::new(conn);

    match action {
        AccountAction::Add {
            platform,
            username,
            proxy,
        } => {
            let account = Account {
                id: Uuid::new_v4().to_string(),
                platform: platform.clone(),
                username: username.clone(),
                profile_path: format!("./profiles/{}_{}", platform, username),
                proxy,
                fingerprint_preset: None,
                status: AccountStatus::Inactive,
                last_active: None,
                health_score: 100,
                created_at: Utc::now(),
            };

            db.create_account(&account)?;
            println!("✓ Account added: {} ({})", account.username, account.id);
        }
        AccountAction::List => {
            let accounts = db.list_accounts()?;
            if accounts.is_empty() {
                println!("No accounts found");
            } else {
                println!("\nAccounts:");
                println!("{:<36} {:<15} {:<20} {:<10}", "ID", "Platform", "Username", "Status");
                println!("{}", "-".repeat(85));
                for account in accounts {
                    println!(
                        "{:<36} {:<15} {:<20} {:<10}",
                        account.id,
                        account.platform,
                        account.username,
                        account.status.as_str()
                    );
                }
            }
        }
        AccountAction::Status { id } => {
            if let Some(account) = db.get_account(&id)? {
                println!("\nAccount Details:");
                println!("  ID: {}", account.id);
                println!("  Platform: {}", account.platform);
                println!("  Username: {}", account.username);
                println!("  Status: {}", account.status.as_str());
                println!("  Health Score: {}", account.health_score);
                println!("  Profile Path: {}", account.profile_path);
                if let Some(last_active) = account.last_active {
                    println!("  Last Active: {}", last_active);
                }
            } else {
                println!("Account not found: {}", id);
            }
        }
        AccountAction::Remove { id } => {
            db.delete_account(&id)?;
            println!("✓ Account removed: {}", id);
        }
    }

    Ok(())
}

async fn handle_task_action(action: TaskAction) -> Result<()> {
    let config = load_config("config.yaml")?;
    let db_path = PathBuf::from(&config.system.data_dir).join("database.db");
    let conn = initialize_database(&db_path)?;
    let db = Database::new(conn);

    match action {
        TaskAction::Create {
            task_type,
            account,
            content,
        } => {
            let task_type_enum = TaskType::from_str(&task_type)
                .ok_or_else(|| anyhow::anyhow!("Invalid task type: {}", task_type))?;

            let task = Task {
                id: Uuid::new_v4().to_string(),
                task_type: task_type_enum,
                account_id: account,
                content_id: content,
                status: TaskStatus::Pending,
                scheduled_at: None,
                executed_at: None,
                retry_count: 0,
                priority: 0,
                metadata: None,
            };

            db.create_task(&task)?;
            println!("✓ Task created: {} ({})", task.task_type.as_str(), task.id);
        }
        TaskAction::List { status } => {
            let status_filter = status
                .as_ref()
                .and_then(|s| TaskStatus::from_str(s));

            let tasks = db.list_tasks(status_filter)?;
            if tasks.is_empty() {
                println!("No tasks found");
            } else {
                println!("\nTasks:");
                println!("{:<36} {:<10} {:<36} {:<10}", "ID", "Type", "Account", "Status");
                println!("{}", "-".repeat(95));
                for task in tasks {
                    println!(
                        "{:<36} {:<10} {:<36} {:<10}",
                        task.id,
                        task.task_type.as_str(),
                        task.account_id,
                        task.status.as_str()
                    );
                }
            }
        }
        TaskAction::Cancel { id } => {
            db.update_task_status(&id, TaskStatus::Cancelled)?;
            println!("✓ Task cancelled: {}", id);
        }
    }

    Ok(())
}

async fn handle_content_action(action: ContentAction) -> Result<()> {
    let config = load_config("config.yaml")?;
    let db_path = PathBuf::from(&config.system.data_dir).join("database.db");
    let conn = initialize_database(&db_path)?;
    let db = Database::new(conn);

    match action {
        ContentAction::Add { file, platform } => {
            let body = std::fs::read_to_string(&file)?;

            let content = Content {
                id: Uuid::new_v4().to_string(),
                content_type: ContentType::Text,
                platform,
                title: None,
                body,
                media_paths: vec![],
                tags: vec![],
                source: Some(file),
                created_at: Utc::now(),
                used_count: 0,
            };

            db.create_content(&content)?;
            println!("✓ Content added: {}", content.id);
        }
        ContentAction::List => {
            let contents = db.list_contents()?;
            if contents.is_empty() {
                println!("No content found");
            } else {
                println!("\nContent:");
                println!("{:<36} {:<10} {:<15} {:<10}", "ID", "Type", "Platform", "Used");
                println!("{}", "-".repeat(75));
                for content in contents {
                    println!(
                        "{:<36} {:<10} {:<15} {:<10}",
                        content.id,
                        content.content_type.as_str(),
                        content.platform,
                        content.used_count
                    );
                }
            }
        }
        ContentAction::Remove { id } => {
            db.delete_content(&id)?;
            println!("✓ Content removed: {}", id);
        }
    }

    Ok(())
}

async fn handle_start() -> Result<()> {
    println!("Starting social-auto service...");

    let config = load_config("config.yaml")?;
    let db_path = PathBuf::from(&config.system.data_dir).join("database.db");
    let conn = initialize_database(&db_path)?;
    let db = Arc::new(Database::new(conn));

    // Create account manager
    let account_manager = Arc::new(AccountManager::new(db.clone()));
    account_manager.load_active_accounts().await?;

    // Create task scheduler
    let scheduler = TaskScheduler::new(db.clone(), config.clone());

    println!("✓ Service started");
    println!("  - Active accounts: {}", account_manager.get_active_accounts().await.len());
    println!("  - Pending tasks: {}", scheduler.get_pending_count().await?);
    println!("\nPress Ctrl+C to stop...");

    // Start scheduler (this will block)
    scheduler.start().await?;

    Ok(())
}

async fn handle_stop() -> Result<()> {
    println!("Stopping social-auto service...");
    println!("Service stopped (not implemented yet)");
    Ok(())
}

async fn handle_status() -> Result<()> {
    let config = load_config("config.yaml")?;
    let db_path = PathBuf::from(&config.system.data_dir).join("database.db");
    let conn = initialize_database(&db_path)?;
    let db = Arc::new(Database::new(conn));

    let account_manager = Arc::new(AccountManager::new(db.clone()));
    account_manager.load_active_accounts().await?;

    let scheduler = TaskScheduler::new(db.clone(), config);

    println!("\nService Status:");
    println!("  Running: {}", if scheduler.is_running().await { "Yes" } else { "No" });
    println!("\nAccounts:");
    println!("  Active: {}", account_manager.get_account_count_by_status(AccountStatus::Active).await?);
    println!("  Inactive: {}", account_manager.get_account_count_by_status(AccountStatus::Inactive).await?);
    println!("  Suspended: {}", account_manager.get_account_count_by_status(AccountStatus::Suspended).await?);
    println!("\nTasks:");
    println!("  Pending: {}", scheduler.get_pending_count().await?);
    println!("  Running: {}", scheduler.get_running_count().await?);

    Ok(())
}

async fn handle_web(port: u16) -> Result<()> {
    println!("Starting web interface on port {}...", port);
    println!("Web interface not implemented yet");
    Ok(())
}
