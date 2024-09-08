use crate::app::App;
use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{block::{Position, Title}, Block, Borders, BorderType, List, ListItem, Paragraph, Padding},
    layout::{Layout, Constraint, Direction, Alignment},
    Frame,
};
use ratatui::prelude::Stylize;

pub fn render_frame(app: &App, frame: &mut Frame) {
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
        "Add Task ".into(),
        "<N>".yellow().bold(),
        " | ".into(),
        "Delete Task ".into(),
        "<D> ".red().bold(),
    ]));
    let block = Block::default()
        .title(title.alignment(Alignment::Center))
        .title(instructions
            .alignment(Alignment::Center)
            .position(Position::Bottom)
        )
        .borders(Borders::ALL)
        .border_type(BorderType::Thick);

    // Render the main block
    frame.render_widget(block, area);

    // Split the block into sections for sidebar, task list, and input area
    let outer_layout = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints([
            Constraint::Percentage(25),
            Constraint::Percentage(75),
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
    let tasks: Vec<ListItem> = app.tasks.iter().enumerate().map(|(i, task)| {
        let status = if task.status { "[x]" } else { "[ ]" };
        let content = format!("{} {}", status, task.description);

        // Highlight the selected task
        ListItem::new(
            if i == app.selected_task {
                Span::styled(content, Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
            } else {
                Span::raw(content)
            }
        )
    }).collect();
    let task_list = List::new(tasks)
        .block(Block::default().borders(Borders::ALL).title(" Task List ".green().bold()).padding(padding));
    frame.render_widget(task_list, inner_layout[0]);

    // Input area for new tasks
    let input = Paragraph::new(app.input_buffer.as_str())
        .block(Block::default().borders(Borders::ALL).title(" Add New Task ".yellow().bold()).padding(h_padding))
        .style(if app.input_mode {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default()
        });
    frame.render_widget(input, inner_layout[1]);
}