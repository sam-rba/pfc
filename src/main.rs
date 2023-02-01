use crossterm::{
    event::{
        self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, KeyModifiers,
    },
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    widgets::Paragraph,
    Frame, Terminal,
};

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
    let mut terminal = init_terminal()?;
    let result = run(app, &mut terminal);
    cleanup_terminal(terminal)?;
    result
}

fn init_terminal() -> Result<Terminal<CrosstermBackend<io::Stdout>>, io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    Terminal::new(backend)
}

fn cleanup_terminal<B>(mut terminal: Terminal<B>) -> Result<(), io::Error>
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
