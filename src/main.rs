use std::io;
mod tui;
mod app;
mod ui;
mod events;
mod db;

fn main() -> io::Result<()> {
    let database = db::Database::new("tasks.db").expect("Failed to initialize database");
    database.initialize().expect("Failed to initialize tables");

    let mut app = app::App::default();
    app.db = Some(database);

    let mut terminal = tui::init()?;
    let app_result = app.run(&mut terminal);
    tui::restore()?;
    app_result
}
