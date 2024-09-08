use std::io;
use crate::tui::Tui;
use crate::events::handle_events;
use crate::ui::render_frame;
use crate::db::Database;

#[derive(Clone)]
pub struct Task {
    pub id: i32,
    pub description: String,
    pub status: bool,
}

impl Task {
    pub fn new(description: String) -> Self {
        Self {
            id: -1,  // Temporary placeholder
            description,
            status: false,
        }
    }
}

#[derive(Default)]
pub struct App {
    pub exit: bool,
    pub tasks: Vec<Task>,
    pub selected_task: usize,
    pub input_mode: bool,
    pub input_buffer: String,
    pub db: Option<Database>,
}

impl App {
    pub fn run(&mut self, terminal: &mut Tui) -> io::Result<()> {
        if let Some(db) = &self.db {
            self.tasks = db.get_tasks().unwrap_or_default(); // Load tasks from DB
        }

        while !self.exit {
            terminal.draw(|frame| render_frame(self, frame))?;
            handle_events(self)?;
        }

        Ok(())
    }

    pub fn add_task(&mut self, description: String) {
        let task = Task::new(description);
        self.tasks.push(task.clone()); // Clone the task before pushing it
        if let Some(db) = &self.db {
            db.add_task(&task).expect("Failed to add task to the database");
        }
    }

    pub fn delete_task(&mut self, task_index: usize) {
        if task_index < self.tasks.len() {
            let task_id = self.tasks[task_index].id; // Save task ID before deletion

            if let Some(db) = &self.db {
                db.delete_task(task_id as i32).expect("Failed to delete task from the database");
                self.tasks.remove(task_index); // Remove task from the list
            }

            // Adjust selected_task index
            if self.selected_task >= self.tasks.len() {
                self.selected_task = self.tasks.len().saturating_sub(1);
            }
        }
    }

    pub fn update_task_status(&mut self, task_index: usize, status: bool) {
        if let Some(task) = self.tasks.get_mut(task_index) {
            task.status = status;
            if let Some(db) = &self.db {
                db.update_task_status(task.id as i32, status)
                    .expect("Failed to update task in the database");
            }
        }
    }
}
