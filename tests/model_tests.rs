// Model conversion and validation unit tests

use social_auto::models::*;

#[test]
fn test_account_status_conversion() {
    assert_eq!(AccountStatus::Active.as_str(), "active");
    assert_eq!(AccountStatus::Suspended.as_str(), "suspended");

    assert_eq!(AccountStatus::from_str("active"), Some(AccountStatus::Active));
    assert_eq!(AccountStatus::from_str("suspended"), Some(AccountStatus::Suspended));
    assert_eq!(AccountStatus::from_str("invalid"), None);
}

#[test]
fn test_task_type_conversion() {
    assert_eq!(TaskType::Post.as_str(), "post");
    assert_eq!(TaskType::Like.as_str(), "like");
    assert_eq!(TaskType::Comment.as_str(), "comment");
    assert_eq!(TaskType::Follow.as_str(), "follow");

    assert_eq!(TaskType::from_str("post"), Some(TaskType::Post));
    assert_eq!(TaskType::from_str("like"), Some(TaskType::Like));
    assert_eq!(TaskType::from_str("invalid"), None);
}

#[test]
fn test_task_status_conversion() {
    assert_eq!(TaskStatus::Pending.as_str(), "pending");
    assert_eq!(TaskStatus::Running.as_str(), "running");
    assert_eq!(TaskStatus::Completed.as_str(), "completed");
    assert_eq!(TaskStatus::Failed.as_str(), "failed");
    assert_eq!(TaskStatus::Cancelled.as_str(), "cancelled");

    assert_eq!(TaskStatus::from_str("pending"), Some(TaskStatus::Pending));
    assert_eq!(TaskStatus::from_str("running"), Some(TaskStatus::Running));
    assert_eq!(TaskStatus::from_str("invalid"), None);
}

#[test]
fn test_content_type_conversion() {
    assert_eq!(ContentType::Text.as_str(), "text");
    assert_eq!(ContentType::Image.as_str(), "image");
    assert_eq!(ContentType::Video.as_str(), "video");
    assert_eq!(ContentType::Mixed.as_str(), "mixed");

    assert_eq!(ContentType::from_str("text"), Some(ContentType::Text));
    assert_eq!(ContentType::from_str("image"), Some(ContentType::Image));
    assert_eq!(ContentType::from_str("invalid"), None);
}
