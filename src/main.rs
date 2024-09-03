use std::io;
use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEvent},
    style::Stylize,
    text::{Line, Span},
    widgets::{block::{Position, Title}, Block, Borders, BorderType, List, ListItem, Paragraph, Padding},
    layout::{Layout, Constraint, Direction, Alignment},
    Frame,
};
mod tui;

struct Task {
    description: String,
    status: bool,
}

impl Task {
    fn new(description: String) -> Task {
        Task {
            description,
            status: false,
        }
    }
}

#[derive(Default)]
pub struct App {
    exit: bool,
    tasks: Vec<Task>,
}

impl App {
    pub fn run(&mut self, terminal: &mut tui::Tui) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn render_frame(&self, frame: &mut Frame) {
        let area = frame.area();
        let padding = Padding::new(1, 1, 1, 1);

        // Main block with title and instructions at the bottom
        let title = Title::from(" Rust Todo ".blue().bold());
        let instructions = Title::from(Line::from(vec![
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]));
        let block = Block::default()
            .title(title.alignment(Alignment::Center))
            .title(instructions
                .alignment(Alignment::Center)
                .position(Position::Bottom)
            )
            .borders(Borders::ALL)
            .border_type(BorderType::Thick)
            .title_alignment(Alignment::Center);

        // Render the main block
        frame.render_widget(block, area);

        // Split the block into sections for sidebar and task list
        let inner = Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .constraints([
                Constraint::Percentage(30),
                Constraint::Percentage(70),
            ].as_ref())
            .split(area);

        // Sidebar Panel
        let sidebar = Paragraph::new("Sidebar")
            .block(Block::default().borders(Borders::ALL).title(" Sidebar ".green().bold()).padding(padding));
        frame.render_widget(sidebar, inner[0]);

        // Task List
        let tasks: Vec<ListItem> = self.tasks.iter().map(|task| {
            let status = if task.status { "[x]" } else { "[ ]" };
            ListItem::new(Span::raw(format!("{} {}", status, task.description)))
        }).collect();
        let task_list = List::new(tasks)
            .block(Block::default().borders(Borders::ALL).title(" Task List ".green().bold()).padding(padding));
        frame.render_widget(task_list, inner[1]);
    }

    fn handle_events(&mut self) -> io::Result<()> {
        if let Event::Key(key_event) = event::read()? {
            self.handle_key_event(key_event);
        }
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        if key_event.code == KeyCode::Char('q') {
            self.exit = true;
        }
    }
}

fn main() -> io::Result<()> {
    let mut app = App::default();
    app.tasks.push(Task::new("First Task".into()));
    app.tasks.push(Task::new("Second Task".into()));
    app.tasks.push(Task::new("Third Task".into()));

    let mut terminal = tui::init()?;
    let app_result = app.run(&mut terminal);
    tui::restore()?;
    app_result
}
