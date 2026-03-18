use crate::models::{Account, AccountStatus, Task, TaskStatus, TaskType, Content, ContentType};
use anyhow::Result;
use rusqlite::{Connection, params};
use std::sync::{Arc, Mutex};

pub struct Database {
    conn: Arc<Mutex<Connection>>,
}

impl Database {
    pub fn new(conn: Connection) -> Self {
        Self {
            conn: Arc::new(Mutex::new(conn)),
        }
    }
}

pub trait DbOperations {
    fn create_account(&self, account: &Account) -> Result<()>;
    fn get_account(&self, id: &str) -> Result<Option<Account>>;
    fn list_accounts(&self) -> Result<Vec<Account>>;
    fn update_account_status(&self, id: &str, status: AccountStatus) -> Result<()>;
    fn delete_account(&self, id: &str) -> Result<()>;

    fn create_task(&self, task: &Task) -> Result<()>;
    fn get_task(&self, id: &str) -> Result<Option<Task>>;
    fn list_tasks(&self, status: Option<TaskStatus>) -> Result<Vec<Task>>;
    fn update_task_status(&self, id: &str, status: TaskStatus) -> Result<()>;
    fn delete_task(&self, id: &str) -> Result<()>;

    fn create_content(&self, content: &Content) -> Result<()>;
    fn get_content(&self, id: &str) -> Result<Option<Content>>;
    fn list_contents(&self) -> Result<Vec<Content>>;
    fn delete_content(&self, id: &str) -> Result<()>;
}

impl DbOperations for Database {
    fn create_account(&self, account: &Account) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO accounts (id, platform, username, profile_path, proxy, fingerprint_preset, status, last_active, health_score, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            params![
                account.id,
                account.platform,
                account.username,
                account.profile_path,
                account.proxy,
                account.fingerprint_preset,
                account.status.as_str(),
                account.last_active,
                account.health_score,
                account.created_at,
            ],
        )?;
        Ok(())
    }

    fn get_account(&self, id: &str) -> Result<Option<Account>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, platform, username, profile_path, proxy, fingerprint_preset, status, last_active, health_score, created_at
             FROM accounts WHERE id = ?1"
        )?;

        let account = stmt.query_row(params![id], |row| {
            Ok(Account {
                id: row.get(0)?,
                platform: row.get(1)?,
                username: row.get(2)?,
                profile_path: row.get(3)?,
                proxy: row.get(4)?,
                fingerprint_preset: row.get(5)?,
                status: AccountStatus::from_str(&row.get::<_, String>(6)?).unwrap(),
                last_active: row.get(7)?,
                health_score: row.get(8)?,
                created_at: row.get(9)?,
            })
        }).optional()?;

        Ok(account)
    }

    fn list_accounts(&self) -> Result<Vec<Account>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, platform, username, profile_path, proxy, fingerprint_preset, status, last_active, health_score, created_at
             FROM accounts"
        )?;

        let accounts = stmt.query_map([], |row| {
            Ok(Account {
                id: row.get(0)?,
                platform: row.get(1)?,
                username: row.get(2)?,
                profile_path: row.get(3)?,
                proxy: row.get(4)?,
                fingerprint_preset: row.get(5)?,
                status: AccountStatus::from_str(&row.get::<_, String>(6)?).unwrap(),
                last_active: row.get(7)?,
                health_score: row.get(8)?,
                created_at: row.get(9)?,
            })
        })?.collect::<Result<Vec<_>, _>>()?;

        Ok(accounts)
    }

    fn update_account_status(&self, id: &str, status: AccountStatus) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "UPDATE accounts SET status = ?1 WHERE id = ?2",
            params![status.as_str(), id],
        )?;
        Ok(())
    }

    fn delete_account(&self, id: &str) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM accounts WHERE id = ?1", params![id])?;
        Ok(())
    }

    fn create_task(&self, task: &Task) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO tasks (id, type, account_id, content_id, status, scheduled_at, executed_at, retry_count, priority, metadata)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            params![
                task.id,
                task.task_type.as_str(),
                task.account_id,
                task.content_id,
                task.status.as_str(),
                task.scheduled_at,
                task.executed_at,
                task.retry_count,
                task.priority,
                task.metadata,
            ],
        )?;
        Ok(())
    }

    fn get_task(&self, id: &str) -> Result<Option<Task>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, type, account_id, content_id, status, scheduled_at, executed_at, retry_count, priority, metadata
             FROM tasks WHERE id = ?1"
        )?;

        let task = stmt.query_row(params![id], |row| {
            Ok(Task {
                id: row.get(0)?,
                task_type: TaskType::from_str(&row.get::<_, String>(1)?).unwrap(),
                account_id: row.get(2)?,
                content_id: row.get(3)?,
                status: TaskStatus::from_str(&row.get::<_, String>(4)?).unwrap(),
                scheduled_at: row.get(5)?,
                executed_at: row.get(6)?,
                retry_count: row.get(7)?,
                priority: row.get(8)?,
                metadata: row.get(9)?,
            })
        }).optional()?;

        Ok(task)
    }

    fn list_tasks(&self, status: Option<TaskStatus>) -> Result<Vec<Task>> {
        let conn = self.conn.lock().unwrap();

        let (query, params_vec): (String, Vec<String>) = match status {
            Some(s) => (
                "SELECT id, type, account_id, content_id, status, scheduled_at, executed_at, retry_count, priority, metadata
                 FROM tasks WHERE status = ?1".to_string(),
                vec![s.as_str().to_string()]
            ),
            None => (
                "SELECT id, type, account_id, content_id, status, scheduled_at, executed_at, retry_count, priority, metadata
                 FROM tasks".to_string(),
                vec![]
            ),
        };

        let mut stmt = conn.prepare(&query)?;
        let tasks = stmt.query_map(rusqlite::params_from_iter(params_vec.iter()), |row| {
            Ok(Task {
                id: row.get(0)?,
                task_type: TaskType::from_str(&row.get::<_, String>(1)?).unwrap(),
                account_id: row.get(2)?,
                content_id: row.get(3)?,
                status: TaskStatus::from_str(&row.get::<_, String>(4)?).unwrap(),
                scheduled_at: row.get(5)?,
                executed_at: row.get(6)?,
                retry_count: row.get(7)?,
                priority: row.get(8)?,
                metadata: row.get(9)?,
            })
        })?.collect::<Result<Vec<_>, _>>()?;

        Ok(tasks)
    }

    fn update_task_status(&self, id: &str, status: TaskStatus) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "UPDATE tasks SET status = ?1 WHERE id = ?2",
            params![status.as_str(), id],
        )?;
        Ok(())
    }

    fn delete_task(&self, id: &str) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM tasks WHERE id = ?1", params![id])?;
        Ok(())
    }

    fn create_content(&self, content: &Content) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        let media_paths = serde_json::to_string(&content.media_paths)?;
        let tags = serde_json::to_string(&content.tags)?;

        conn.execute(
            "INSERT INTO contents (id, type, platform, title, body, media_paths, tags, source, created_at, used_count)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            params![
                content.id,
                content.content_type.as_str(),
                content.platform,
                content.title,
                content.body,
                media_paths,
                tags,
                content.source,
                content.created_at,
                content.used_count,
            ],
        )?;
        Ok(())
    }

    fn get_content(&self, id: &str) -> Result<Option<Content>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, type, platform, title, body, media_paths, tags, source, created_at, used_count
             FROM contents WHERE id = ?1"
        )?;

        let content = stmt.query_row(params![id], |row| {
            let media_paths: String = row.get(5)?;
            let tags: String = row.get(6)?;

            Ok(Content {
                id: row.get(0)?,
                content_type: ContentType::from_str(&row.get::<_, String>(1)?).unwrap(),
                platform: row.get(2)?,
                title: row.get(3)?,
                body: row.get(4)?,
                media_paths: serde_json::from_str(&media_paths).unwrap_or_default(),
                tags: serde_json::from_str(&tags).unwrap_or_default(),
                source: row.get(7)?,
                created_at: row.get(8)?,
                used_count: row.get(9)?,
            })
        }).optional()?;

        Ok(content)
    }

    fn list_contents(&self) -> Result<Vec<Content>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, type, platform, title, body, media_paths, tags, source, created_at, used_count
             FROM contents"
        )?;

        let contents = stmt.query_map([], |row| {
            let media_paths: String = row.get(5)?;
            let tags: String = row.get(6)?;

            Ok(Content {
                id: row.get(0)?,
                content_type: ContentType::from_str(&row.get::<_, String>(1)?).unwrap(),
                platform: row.get(2)?,
                title: row.get(3)?,
                body: row.get(4)?,
                media_paths: serde_json::from_str(&media_paths).unwrap_or_default(),
                tags: serde_json::from_str(&tags).unwrap_or_default(),
                source: row.get(7)?,
                created_at: row.get(8)?,
                used_count: row.get(9)?,
            })
        })?.collect::<Result<Vec<_>, _>>()?;

        Ok(contents)
    }

    fn delete_content(&self, id: &str) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM contents WHERE id = ?1", params![id])?;
        Ok(())
    }
}

