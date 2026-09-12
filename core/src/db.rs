use crate::error::Result;
use crate::types::{Operation, Report};
use rusqlite::{params, Connection};
use std::path::Path;

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn open(path: &Path) -> Result<Self> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let conn = Connection::open(path)?;
        let db = Self { conn };
        db.migrate()?;
        Ok(db)
    }

    fn migrate(&self) -> Result<()> {
        self.conn.execute_batch(
            r#"
            CREATE TABLE IF NOT EXISTS devices (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                model TEXT,
                platform TEXT,
                transport TEXT,
                last_seen TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS operations (
                id TEXT PRIMARY KEY,
                kind TEXT NOT NULL,
                state TEXT NOT NULL,
                device_id TEXT NOT NULL,
                dry_run INTEGER NOT NULL,
                message TEXT,
                payload TEXT NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS reports (
                id TEXT PRIMARY KEY,
                operation_id TEXT NOT NULL,
                title TEXT NOT NULL,
                body TEXT NOT NULL,
                success INTEGER NOT NULL,
                created_at TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS preferences (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL
            );
            CREATE INDEX IF NOT EXISTS idx_ops_device ON operations(device_id);
            "#,
        )?;
        Ok(())
    }

    pub fn upsert_device(
        &self,
        id: &str,
        name: &str,
        model: &str,
        platform: &str,
        transport: &str,
    ) -> Result<()> {
        self.conn.execute(
            r#"INSERT INTO devices (id, name, model, platform, transport, last_seen)
               VALUES (?1, ?2, ?3, ?4, ?5, datetime('now'))
               ON CONFLICT(id) DO UPDATE SET
                 name=excluded.name,
                 model=excluded.model,
                 platform=excluded.platform,
                 transport=excluded.transport,
                 last_seen=datetime('now')"#,
            params![id, name, model, platform, transport],
        )?;
        Ok(())
    }

    pub fn save_operation(&self, op: &Operation) -> Result<()> {
        let payload = serde_json::to_string(op)?;
        self.conn.execute(
            r#"INSERT INTO operations (id, kind, state, device_id, dry_run, message, payload, created_at, updated_at)
               VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)
               ON CONFLICT(id) DO UPDATE SET
                 state=excluded.state,
                 message=excluded.message,
                 payload=excluded.payload,
                 updated_at=excluded.updated_at"#,
            params![
                op.id.to_string(),
                format!("{:?}", op.kind),
                format!("{:?}", op.state),
                op.device_id,
                op.dry_run as i32,
                op.message,
                payload,
                op.created_at.to_rfc3339(),
                op.updated_at.to_rfc3339(),
            ],
        )?;
        Ok(())
    }

    pub fn list_operations(&self, limit: usize) -> Result<Vec<Operation>> {
        let mut stmt = self.conn.prepare(
            "SELECT payload FROM operations ORDER BY updated_at DESC LIMIT ?1",
        )?;
        let rows = stmt.query_map(params![limit as i64], |row| {
            let payload: String = row.get(0)?;
            Ok(payload)
        })?;
        let mut out = Vec::new();
        for r in rows {
            let payload = r?;
            if let Ok(op) = serde_json::from_str(&payload) {
                out.push(op);
            }
        }
        Ok(out)
    }

    pub fn save_report(&self, report: &Report) -> Result<()> {
        self.conn.execute(
            r#"INSERT INTO reports (id, operation_id, title, body, success, created_at)
               VALUES (?1, ?2, ?3, ?4, ?5, ?6)"#,
            params![
                report.id.to_string(),
                report.operation_id.to_string(),
                report.title,
                report.body,
                report.success as i32,
                report.created_at.to_rfc3339(),
            ],
        )?;
        Ok(())
    }

    pub fn list_reports(&self, limit: usize) -> Result<Vec<Report>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, operation_id, title, body, success, created_at FROM reports ORDER BY created_at DESC LIMIT ?1",
        )?;
        let rows = stmt.query_map(params![limit as i64], |row| {
            let id: String = row.get(0)?;
            let operation_id: String = row.get(1)?;
            let title: String = row.get(2)?;
            let body: String = row.get(3)?;
            let success: i32 = row.get(4)?;
            let created_at: String = row.get(5)?;
            Ok((id, operation_id, title, body, success, created_at))
        })?;
        let mut out = Vec::new();
        for r in rows {
            let (id, operation_id, title, body, success, created_at) = r?;
            out.push(Report {
                id: uuid::Uuid::parse_str(&id).unwrap_or_else(|_| uuid::Uuid::nil()),
                operation_id: uuid::Uuid::parse_str(&operation_id)
                    .unwrap_or_else(|_| uuid::Uuid::nil()),
                title,
                body,
                success: success != 0,
                created_at: chrono::DateTime::parse_from_rfc3339(&created_at)
                    .map(|d| d.with_timezone(&chrono::Utc))
                    .unwrap_or_else(|_| chrono::Utc::now()),
            });
        }
        Ok(out)
    }

    pub fn set_pref(&self, key: &str, value: &str) -> Result<()> {
        self.conn.execute(
            "INSERT INTO preferences (key, value) VALUES (?1, ?2) ON CONFLICT(key) DO UPDATE SET value=excluded.value",
            params![key, value],
        )?;
        Ok(())
    }

    pub fn get_pref(&self, key: &str) -> Result<Option<String>> {
        let mut stmt = self
            .conn
            .prepare("SELECT value FROM preferences WHERE key = ?1")?;
        let mut rows = stmt.query(params![key])?;
        if let Some(row) = rows.next()? {
            Ok(Some(row.get(0)?))
        } else {
            Ok(None)
        }
    }
}
