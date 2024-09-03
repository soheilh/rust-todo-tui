use std::io;
use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEvent},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{block::{Position, Title}, Block, Borders, BorderType, List, ListItem, Paragraph, Padding},
    layout::{Layout, Constraint, Direction, Alignment},
    Frame,
};
use ratatui::prelude::*;
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
    selected_task: usize,
    input_mode: bool,
    input_buffer: String,
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
        let h_padding = Padding::new(1, 1, 0, 0);

        // Main block with title and instructions at the bottom
        let title = Title::from(" Rust Todo ".blue().bold());
        let instructions = Title::from(Line::from(vec![
            " Quit ".into(),
            "<Q>".blue().bold(),
            " | ".into(),
            "Navigate ".into(),
            "<Up/Down>".blue().bold(),
            " | ".into(),
            if self.input_mode {
                "Insert ".into()
            } else {
                "Toggle Status ".into()
            },
            "<Enter>".green().bold(),
            " | ".into(),
            "Add Task ".into(),
            "<n> ".yellow().bold(),
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

        // Split the block into sections for sidebar, task list, and input area
        let outer_layout = Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .constraints([
                Constraint::Percentage(30),
                Constraint::Percentage(70),
            ].as_ref())
            .split(area);

        let inner_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(1),
                Constraint::Length(3),
            ].as_ref())
            .split(outer_layout[1]);

        // Sidebar Panel
        let sidebar = Paragraph::new("Sidebar")
            .block(Block::default().borders(Borders::ALL).title(" Sidebar ".green().bold()).padding(padding));
        frame.render_widget(sidebar, outer_layout[0]);

        // Task List Panel
        let tasks: Vec<ListItem> = self.tasks.iter().enumerate().map(|(i, task)| {
            let status = if task.status { "[x]" } else { "[ ]" };
            let content = format!("{} {}", status, task.description);

            // Highlight the selected task
            if i == self.selected_task {
                ListItem::new(Span::styled(content, Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)))
            } else {
                ListItem::new(Span::raw(content))
            }
        }).collect();
        let task_list = List::new(tasks)
            .block(Block::default().borders(Borders::ALL).title(" Task List ".green().bold()).padding(padding));
        frame.render_widget(task_list, inner_layout[0]);

        // Input area for new tasks
        let input = Paragraph::new(self.input_buffer.as_str())
            .block(Block::default().borders(Borders::ALL).title(" Add New Task ".yellow().bold()).padding(h_padding))
            .style(if self.input_mode {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default()
            });
        frame.render_widget(input, inner_layout[1]);
    }

    fn handle_events(&mut self) -> io::Result<()> {
        if let Event::Key(key_event) = event::read()? {
            self.handle_key_event(key_event);
        }
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => {
                self.exit = true;
            }
            KeyCode::Up => {
                if !self.input_mode && self.selected_task > 0 {
                    self.selected_task -= 1;
                }
            }
            KeyCode::Down => {
                if !self.input_mode && self.selected_task < self.tasks.len() - 1 {
                    self.selected_task += 1;
                }
            }
            KeyCode::Enter => {
                if self.input_mode {
                    if !self.input_buffer.is_empty() {
                        self.tasks.push(Task::new(self.input_buffer.drain(..).collect()));
                    }
                    self.input_mode = false;
                } else if let Some(task) = self.tasks.get_mut(self.selected_task) {
                    task.status = !task.status;
                }
            }
            KeyCode::Char('n') => {
                self.input_mode = true;
                self.input_buffer.clear();
            }
            KeyCode::Char(c) => {
                if self.input_mode {
                    self.input_buffer.push(c);
                }
            }
            KeyCode::Backspace => {
                if self.input_mode {
                    self.input_buffer.pop();
                }
            }
            _ => {}
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
