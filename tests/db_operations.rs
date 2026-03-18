// Database operations unit tests

use anyhow::Result;
use social_auto::db::DbOperations;
use social_auto::models::{Account, AccountStatus, Content, ContentType, Task, TaskStatus, TaskType};

mod common;

#[test]
fn test_account_crud() -> Result<()> {
    let (conn, _temp_dir) = common::create_test_db()?;
    let db = social_auto::db::Database::new(conn);

    let account = Account {
        id: uuid::Uuid::new_v4().to_string(),
        platform: "xiaohongshu".to_string(),
        username: "test_user".to_string(),
        profile_path: "./profiles/test_user".to_string(),
        proxy: None,
        fingerprint_preset: None,
        status: AccountStatus::Active,
        health_score: 100,
        last_active: None,
        created_at: chrono::Utc::now(),
    };

    db.create_account(&account)?;

    let retrieved = db.get_account(&account.id)?;
    assert!(retrieved.is_some());
    let retrieved = retrieved.unwrap();
    assert_eq!(retrieved.username, "test_user");

    let accounts = db.list_accounts()?;
    assert_eq!(accounts.len(), 1);

    // Test update status
    db.update_account_status(&account.id, AccountStatus::Suspended)?;
    let retrieved = db.get_account(&account.id)?.unwrap();
    assert_eq!(retrieved.status, AccountStatus::Suspended);

    db.delete_account(&account.id)?;
    assert!(db.get_account(&account.id)?.is_none());

    Ok(())
}

#[test]
fn test_task_crud() -> Result<()> {
    let (conn, _temp_dir) = common::create_test_db()?;
    let db = social_auto::db::Database::new(conn);

    let account = Account {
        id: uuid::Uuid::new_v4().to_string(),
        platform: "twitter".to_string(),
        username: "test_user".to_string(),
        profile_path: "./profiles/test_user".to_string(),
        proxy: None,
        fingerprint_preset: None,
        status: AccountStatus::Active,
        health_score: 100,
        last_active: None,
        created_at: chrono::Utc::now(),
    };
    db.create_account(&account)?;

    let task = Task {
        id: uuid::Uuid::new_v4().to_string(),
        task_type: TaskType::Post,
        account_id: account.id.clone(),
        content_id: Some(uuid::Uuid::new_v4().to_string()),
        status: TaskStatus::Pending,
        scheduled_at: None,
        executed_at: None,
        retry_count: 0,
        priority: 1,
        metadata: None,
    };

    db.create_task(&task)?;

    let retrieved = db.get_task(&task.id)?;
    assert!(retrieved.is_some());

    let tasks = db.list_tasks(None)?;
    assert_eq!(tasks.len(), 1);

    Ok(())
}

#[test]
fn test_content_crud() -> Result<()> {
    let (conn, _temp_dir) = common::create_test_db()?;
    let db = social_auto::db::Database::new(conn);

    let content = Content {
        id: uuid::Uuid::new_v4().to_string(),
        content_type: ContentType::Text,
        platform: "xiaohongshu".to_string(),
        title: Some("Test Title".to_string()),
        body: "Test content body".to_string(),
        media_paths: vec![],
        tags: vec!["test".to_string()],
        source: None,
        created_at: chrono::Utc::now(),
        used_count: 0,
    };

    db.create_content(&content)?;

    let retrieved = db.get_content(&content.id)?;
    assert!(retrieved.is_some());

    let contents = db.list_contents()?;
    assert_eq!(contents.len(), 1);

    Ok(())
}
