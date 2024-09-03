use std::io;
use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEvent},
    style::{Stylize},
    symbols::border,
    text::Line,
    widgets::{block::{Position, Title}, Block, Paragraph, Widget},
    buffer::Buffer,
    layout::{Alignment, Rect},
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

fn main() -> io::Result<()> {
    let task = Task::new("First Task".into());
    println!("Task: {}, status: {}", task.description, task.status);
    let mut terminal = tui::init()?;
    let app_result = App::default().run(&mut terminal);
    tui::restore()?;
    app_result
}

#[derive(Default)]
pub struct App {
    exit: bool,
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
        frame.render_widget(self, frame.area());
    }
    fn handle_events(&mut self) -> io::Result<()> {
        if let Event::Key(key_event) = event::read()? {
            self.handle_key_event(key_event);
        }
        Ok(())
    }
    fn handle_key_event(&mut self, key_event: KeyEvent) {
        if let KeyCode::Char('q') = key_event.code {
            self.exit = true;
        }
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Title::from(" Rust Todo ".bold());
        let instructions = Title::from(Line::from(vec![
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]));
        let block = Block::bordered()
            .title(title.alignment(Alignment::Center))
            .title(instructions
                .alignment(Alignment::Center)
                .position(Position::Bottom)
            )
            .border_set(border::THICK);

        let text = Paragraph::new("Welcome to Rust Todo App!")
            .centered()
            .block(block);

        text.render(area, buf);
    }
}
