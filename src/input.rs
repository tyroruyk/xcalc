use crossterm::event::{KeyCode, KeyEvent};
use crate::mode::Mode;
use crate::utils::calculate;

pub fn handle_input(
    key: KeyEvent,
    mode: &mut Mode,
    input: &mut String,
    output: &mut String,
    command: &mut String,
) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
    match mode {
        Mode::Normal => match key.code {
            KeyCode::Char('i') => *mode = Mode::Input,
            KeyCode::Char(':') => {
                *mode = Mode::Command;
                input.clear();
                command.clear();
            }
            // KeyCode::Char('q') => {
            //     return Ok(true);
            // }
            _ => {}
        },
        Mode::Input => match key.code {
            KeyCode::Esc => *mode = Mode::Normal,
            KeyCode::Char(c) => input.push(c),
            KeyCode::Backspace => {
                input.pop();
            }
            KeyCode::Enter => {
                *output = calculate(input);
            }
            _ => {}
        },
        Mode::Command => match key.code {
            KeyCode::Esc => *mode = Mode::Normal,
            KeyCode::Char(c) => command.push(c),
            KeyCode::Backspace => {
                command.pop();
            }
            KeyCode::Enter => {
                if command == "c" {
                    input.clear();
                    output.clear();
                }
                if command == "q" {
                    return Ok(true);
                }
                *mode = Mode::Normal;
            }
            _ => {}
        },
    }
    Ok(false)
}
