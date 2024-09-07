use rusqlite::{params, Connection, Result};
use crate::app::Task;

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new(db_path: &str) -> Result<Self> {
        let conn = Connection::open(db_path)?;
        Ok(Self { conn })
    }

    pub fn initialize(&self) -> Result<()> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS tasks (
                id INTEGER PRIMARY KEY,
                description TEXT NOT NULL,
                status INTEGER NOT NULL
            )",
            [],
        )?;
        Ok(())
    }

    pub fn add_task(&self, task: &Task) -> Result<()> {
        self.conn.execute(
            "INSERT INTO tasks (description, status) VALUES (?1, ?2)",
            params![task.description, task.status as i32],
        )?;
        Ok(())
    }

    pub fn update_task_status(&self, id: i32, status: bool) -> Result<()> {
        self.conn.execute(
            "UPDATE tasks SET status = ?1 WHERE id = ?2",
            params![status as i32, id],
        )?;
        Ok(())
    }

    pub fn get_tasks(&self) -> Result<Vec<Task>> {
        let mut stmt = self.conn.prepare("SELECT id, description, status FROM tasks")?;
        let task_iter = stmt.query_map([], |row| {
            Ok(Task {
                id: row.get(0)?,
                description: row.get(1)?,
                status: row.get(2)?,
            })
        })?;

        let mut tasks = Vec::new();
        for task in task_iter {
            tasks.push(task?);
        }
        Ok(tasks)
    }
}
