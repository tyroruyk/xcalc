use crate::input::handle_input;
use crate::mode::Mode;
use crate::ui::draw_ui;
use crossterm::event::{self, Event};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::io;
use std::time::{Duration, Instant};
use tui::backend::{Backend, CrosstermBackend};
use tui::Terminal;

pub fn run_app() -> io::Result<()> {
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    crossterm::execute!(io::stdout(), crossterm::terminal::EnterAlternateScreen)?;
    enable_raw_mode()?;

    let result = run_main_loop(&mut terminal);

    crossterm::execute!(io::stdout(), crossterm::terminal::LeaveAlternateScreen)?;
    disable_raw_mode()?;

    // Converting Box<dyn std::error::Error> to io::Error
    match result {
        Ok(_) => Ok(()),
        Err(err) => Err(io::Error::new(io::ErrorKind::Other, err)),
    }
}

fn run_main_loop<B: Backend>(terminal: &mut Terminal<B>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut input = String::new();
    let mut output = String::new();
    let mut command = String::new();
    let mut mode = Mode::Normal;
    let mut last_tick = Instant::now();
    let tick_rate = Duration::from_millis(500);
    let mut show_cursor = true;

    loop {
        let now = Instant::now();
        if now.duration_since(last_tick) >= tick_rate {
            if matches!(mode, Mode::Input) {
                show_cursor = !show_cursor;
            } else {
                show_cursor = false;
            }
            last_tick = now;
        }

        draw_ui(terminal, &input, &output, &command, &mode, show_cursor)?;

        if crossterm::event::poll(tick_rate)? {
            if let Event::Key(key) = event::read()? {
                if handle_input(key, &mut mode, &mut input, &mut output, &mut command)? {
                    break;
                }
            }
        }
    }
    Ok(())
}
