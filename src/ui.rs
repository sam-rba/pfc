use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders, List, ListItem, Paragraph, Widget},
    Frame, Terminal,
};

use crate::Calculator;

const WIDTH: u16 = 32;

impl Calculator {
    pub fn draw<B: Backend>(&self, f: &mut Frame<B>) {
        let chunks = layout(self.stack.len(), f.size());
        f.render_widget(version_number_widget(), chunks[0]);
        f.render_widget(stack_widget(&self.stack), chunks[2]);
        f.render_widget(
            Paragraph::new(format!("> {}", self.input_buffer))
                .block(Block::default().borders(Borders::ALL)),
            chunks[3],
        );
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

fn layout(stack_size: usize, frame_size: Rect) -> Vec<Rect> {
    let columns = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(WIDTH), Constraint::Max(u16::MAX)].as_ref())
        .split(frame_size);

    Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(1),
                Constraint::Max(u16::MAX),
                Constraint::Length(stack_size as u16 + 2),
                Constraint::Length(3),
            ]
            .as_ref(),
        )
        .split(columns[0])
}

fn stack_widget(stack: &Vec<f64>) -> impl Widget {
    List::new(
        stack
            .iter()
            .map(|f| ListItem::new(format!("  {}", f)))
            .collect::<Vec<ListItem>>(),
    )
    .block(Block::default().borders(Borders::ALL))
}

fn version_number_widget() -> impl Widget {
    Paragraph::new(format!("pfc-{}", option_env!("CARGO_PKG_VERSION").unwrap()))
        .alignment(Alignment::Center)
}
