use anyhow::Result;
use rusqlite::Connection;

pub fn create_tables(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS accounts (
            id TEXT PRIMARY KEY,
            platform TEXT NOT NULL,
            username TEXT NOT NULL,
            profile_path TEXT NOT NULL,
            proxy TEXT,
            fingerprint_preset TEXT,
            status TEXT NOT NULL,
            last_active TIMESTAMP,
            health_score INTEGER DEFAULT 100,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS tasks (
            id TEXT PRIMARY KEY,
            type TEXT NOT NULL,
            account_id TEXT NOT NULL,
            content_id TEXT,
            status TEXT NOT NULL,
            scheduled_at TIMESTAMP,
            executed_at TIMESTAMP,
            retry_count INTEGER DEFAULT 0,
            priority INTEGER DEFAULT 0,
            metadata TEXT,
            FOREIGN KEY (account_id) REFERENCES accounts(id)
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS contents (
            id TEXT PRIMARY KEY,
            type TEXT NOT NULL,
            platform TEXT NOT NULL,
            title TEXT,
            body TEXT,
            media_paths TEXT,
            tags TEXT,
            source TEXT,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            used_count INTEGER DEFAULT 0
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS interactions (
            id TEXT PRIMARY KEY,
            account_id TEXT NOT NULL,
            target_type TEXT NOT NULL,
            target_id TEXT NOT NULL,
            action_type TEXT NOT NULL,
            status TEXT NOT NULL,
            executed_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (account_id) REFERENCES accounts(id)
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS logs (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            level TEXT NOT NULL,
            module TEXT NOT NULL,
            message TEXT NOT NULL,
            account_id TEXT,
            task_id TEXT,
            timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )",
        [],
    )?;

    Ok(())
}
