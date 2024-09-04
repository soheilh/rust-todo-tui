use std::io;
mod tui;
mod app;
mod ui;
mod events;

fn main() -> io::Result<()> {
    let mut app = app::App::default();
    app.tasks.push(app::Task::new("First Task".into()));
    app.tasks.push(app::Task::new("Second Task".into()));
    app.tasks.push(app::Task::new("Third Task".into()));

    let mut terminal = tui::init()?;
    let app_result = app.run(&mut terminal);
    tui::restore()?;
    app_result
}
