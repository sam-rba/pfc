use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    widgets::{List, ListItem, Paragraph},
    Frame, Terminal,
};

use crate::Calculator;

impl Calculator {
    pub fn draw<B: Backend>(&self, f: &mut Frame<B>) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Max(u16::MAX),
                    Constraint::Length(self.stack.len() as u16),
                    Constraint::Length(1),
                ]
                .as_ref(),
            )
            .split(f.size());

        let items: Vec<ListItem> = (self.stack)
            .iter()
            .map(|f| ListItem::new(format!("{}", f)))
            .collect();
        f.render_widget(List::new(items), chunks[1]);

        f.render_widget(Paragraph::new(self.input_buffer.as_str()), chunks[2]);
    }
}

pub fn init_terminal() -> Result<Terminal<CrosstermBackend<io::Stdout>>, io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    Terminal::new(backend)
}

pub fn cleanup_terminal<B>(mut terminal: Terminal<B>) -> Result<(), io::Error>
where
    B: Backend + io::Write,
{
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture,
    )?;
    terminal.show_cursor()?;
    Ok(())
}
