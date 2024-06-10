use crate::mode::Mode;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use std::io;

pub fn draw_ui<B: Backend>(
    terminal: &mut Terminal<B>,
    input: &str,
    output: &str,
    command: &str,
    mode: &Mode,
    show_cursor: bool,
) -> io::Result<()> {
    terminal.draw(|f| {
        let size = f.size();
        let total_height = size.height as f32;

        let title_height = 3.0;
        let mode_height = 3.0;

        let available_height = total_height - title_height - mode_height;
        let input_height = (available_height * 4.0 / 7.0).round() as u16;
        let output_height = (available_height * 3.0 / 7.0).round() as u16;

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Length(title_height as u16),
                    Constraint::Length(input_height),
                    Constraint::Length(output_height),
                    Constraint::Length(mode_height as u16),
                ]
                .as_ref(),
            )
            .split(size);

        let title_style = Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD);
        let title_border_style = Style::default().fg(Color::White);
        let title = Paragraph::new(Span::styled("XCalc - A simple calculator for your terminal", title_style))
            .block(Block::default().borders(Borders::ALL).border_style(title_border_style));

        let cursor_position = input.len();
        let mut input_with_cursor = input.to_string();
        if matches!(mode, Mode::Input) && show_cursor {
            // input_with_cursor.insert(cursor_position, '█');
            input_with_cursor.insert(cursor_position, '|');
        }

        let input_style = Style::default().fg(Color::Green);
        let input_border_style = Style::default().fg(Color::White);
        let input_widget = Paragraph::new(Span::styled(input_with_cursor, input_style))
            .block(Block::default().borders(Borders::ALL).border_style(input_border_style).title("Input"));

        let output_style = Style::default().fg(Color::Blue);
        let output_border_style = Style::default().fg(Color::White);
        let output_widget = Paragraph::new(Span::styled(output, output_style))
            .block(Block::default().borders(Borders::ALL).border_style(output_border_style).title("Output"));

        let mode_style = Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD);
        let mode_border_style = Style::default().fg(Color::White);
        let mode_display = Paragraph::new(Span::styled(
            format!("-- {} --", match mode {
                Mode::Normal => "NORMAL",
                Mode::Input => "INPUT",
                Mode::Command => "COMMAND",
            }),
            mode_style,
        ))
        .block(Block::default().borders(Borders::ALL).border_style(mode_border_style));

        f.render_widget(title, chunks[0]);
        f.render_widget(input_widget, chunks[1]);
        f.render_widget(output_widget, chunks[2]);
        f.render_widget(mode_display, chunks[3]);

        if matches!(mode, Mode::Command) {
            let command_style = Style::default().fg(Color::Red);
            let command_border_style = Style::default().fg(Color::White);
            let command_widget = Paragraph::new(Span::styled(format!(":{}", command), command_style))
                .block(Block::default().borders(Borders::ALL).border_style(command_border_style).title("Command"));
            f.render_widget(command_widget, chunks[1]);
        }
    })?;
    Ok(())
}
