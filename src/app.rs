use std::io;
use crate::tui::Tui;
use crate::events::handle_events;
use crate::ui::render_frame;

pub struct Task {
    pub description: String,
    pub status: bool,
}

impl Task {
    pub fn new(description: String) -> Self {
        Self {
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
}

impl App {
    pub fn run(&mut self, terminal: &mut Tui) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| render_frame(self, frame))?;
            handle_events(self)?;
        }
        Ok(())
    }
}
