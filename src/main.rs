use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use std::io;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    widgets::Paragraph,
    Frame, Terminal,
};

use pfc::ui;

enum Signal {
    None,
    Exit,
}

#[derive(Default)]
struct App {}

impl App {
    fn handle_input(&mut self, key: KeyEvent) -> Signal {
        match key.modifiers {
            KeyModifiers::CONTROL => match key.code {
                KeyCode::Char('c') => {
                    return Signal::Exit;
                }
                _ => {}
            },
            KeyModifiers::NONE => match key.code {
                KeyCode::Char('q') => {
                    return Signal::Exit;
                }
                _ => {}
            },
            _ => {}
        }
        return Signal::None;
    }

    fn draw<B: Backend>(&self, f: &mut Frame<B>) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(100)].as_ref())
            .split(f.size());
        f.render_widget(Paragraph::new("test"), chunks[0]);
    }
}

fn main() -> io::Result<()> {
    let app = App::default();
    let mut terminal = ui::init_terminal()?;
    let result = run(app, &mut terminal);
    ui::cleanup_terminal(terminal)?;
    result
}

fn run<B: Backend>(mut app: App, terminal: &mut Terminal<B>) -> io::Result<()> {
    loop {
        terminal.draw(|f| app.draw(f))?;

        if let Event::Key(key) = event::read()? {
            if let Signal::Exit = app.handle_input(key) {
                return Ok(());
            }
        }
    }
}
