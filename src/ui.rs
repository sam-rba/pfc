use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout},
    widgets::{List, ListItem, Paragraph, Widget},
    Frame, Terminal,
};

use crate::Calculator;

impl Calculator {
    pub fn draw<B: Backend>(&self, f: &mut Frame<B>) {
        let chunks = layout(self.stack.len()).split(f.size());
        f.render_widget(version_number_widget(), chunks[0]);
        f.render_widget(stack_widget(&self.stack), chunks[2]);
        f.render_widget(Paragraph::new(self.input_buffer.as_str()), chunks[3]);
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

fn layout(stack_size: usize) -> Layout {
    Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(1),
                Constraint::Max(u16::MAX),
                Constraint::Length(stack_size as u16),
                Constraint::Length(1),
            ]
            .as_ref(),
        )
}

fn stack_widget(stack: &Vec<f64>) -> impl Widget {
    List::new(
        stack
            .iter()
            .map(|f| ListItem::new(format!("{}", f)))
            .collect::<Vec<ListItem>>(),
    )
}

fn version_number_widget() -> impl Widget {
    Paragraph::new(format!("pfc-{}", option_env!("CARGO_PKG_VERSION").unwrap()))
        .alignment(Alignment::Center)
}
