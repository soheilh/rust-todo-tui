use std::io;
use crate::app::App;
use ratatui::crossterm::event::{self, Event, KeyCode, KeyEvent};

pub fn handle_events(app: &mut App) -> io::Result<()> {
    if let Event::Key(key_event) = event::read()? {
        handle_key_event(app, key_event);
    }
    Ok(())
}

fn handle_key_event(app: &mut App, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Char('q') => app.exit = true,
        KeyCode::Up => {
            if !app.input_mode && app.selected_task > 0 {
                app.selected_task -= 1;
            }
        }
        KeyCode::Down => {
            if !app.input_mode && app.selected_task < app.tasks.len() - 1 {
                app.selected_task += 1;
            }
        }
        KeyCode::Enter => {
            if app.input_mode {
                if !app.input_buffer.is_empty() {
                    let input = app.input_buffer.drain(..).collect(); // Store drained content
                    app.add_task(input);
                }
                app.input_mode = false;
            } else {
                app.update_task_status(app.selected_task, !app.tasks[app.selected_task].status);
            }
        }
        KeyCode::Esc => {
            if app.input_mode {
                app.input_buffer.clear();
                app.input_mode = false;
            }
        }
        KeyCode::Char('n') => {
            if !app.input_mode {
                app.input_mode = true;
                app.input_buffer.clear();
            } else {
                app.input_buffer.push('n');
            }
        }
        KeyCode::Char('d') => {
            app.delete_task(app.selected_task)
        }
        KeyCode::Char(c) => {
            if app.input_mode {
                app.input_buffer.push(c);
            }
        }
        KeyCode::Backspace => {
            if app.input_mode {
                app.input_buffer.pop();
            }
        }
        _ => {}
    }
}
