use std::io::{self, stdout, Stdout};
use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    Terminal,
};

/// A type alias for the terminal type used in this application
pub type Tui = Terminal<CrosstermBackend<Stdout>>;

/// Initialize the terminal
/// This function sets up the terminal in raw mode and switches to an alternate screen.
/// It returns a configured Terminal instance.
pub fn init() -> io::Result<Tui> {
    // Enter alternate screen and enable raw mode for the terminal
    execute!(stdout(), EnterAlternateScreen)?; 
    enable_raw_mode()?;

    // Create and return a new terminal instance with CrosstermBackend
    Terminal::new(CrosstermBackend::new(stdout()))
}

/// Restore the terminal to its original state
/// This function exits the alternate screen and disables raw mode.
pub fn restore() -> io::Result<()> {
    // Leave the alternate screen and disable raw mode
    execute!(stdout(), LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
