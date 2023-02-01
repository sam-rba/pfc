use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use std::io;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    widgets::{List, ListItem, Paragraph},
    Frame, Terminal,
};

use pfc::{ui, Operator};

enum Signal {
    None,
    Exit,
}

#[derive(Default)]
struct App {
    stack: Vec<f64>,
    input_buffer: String,
}

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
                KeyCode::Char(c) => {
                    if c.is_ascii_digit() {
                        self.input_buffer.push(c);
                    } else if c == '.' && !self.input_buffer.contains('.') {
                        if self.input_buffer.len() == 0 {
                            self.input_buffer.push('0');
                        }
                        self.input_buffer.push(c);
                    } else if let Ok(op) = Operator::parse(c) {
                        if self.input_buffer.len() > 0 {
                            self.stack.push(self.input_buffer.parse::<f64>().unwrap());
                            self.input_buffer = String::new();
                        }
                        self.perform_operation(op);
                    }
                }
                KeyCode::Enter if self.input_buffer.len() > 0 => {
                    self.stack.push(self.input_buffer.parse::<f64>().unwrap());
                    self.input_buffer = String::new();
                }
                KeyCode::Backspace => {
                    self.input_buffer.pop();
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

    fn perform_operation(&mut self, op: Operator) {
        let rhs = match self.stack.pop() {
            Some(f) => f,
            None => {
                return;
            }
        };
        let lhs = match self.stack.pop() {
            Some(f) => f,
            None => {
                return;
            }
        };
        match op {
            Operator::Add => self.stack.push(lhs + rhs),
            Operator::Sub => self.stack.push(lhs - rhs),
            Operator::Mul => self.stack.push(lhs * rhs),
            Operator::Div => self.stack.push(lhs / rhs),
        }
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
