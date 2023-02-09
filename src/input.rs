use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::{Calculator, Function, Operator, Signal};

impl Calculator {
    pub fn handle_input(&mut self, key: KeyEvent) -> Signal {
        match key.modifiers {
            KeyModifiers::CONTROL => match key.code {
                KeyCode::Char('c') => {
                    return Signal::Exit;
                }
                _ => {}
            },
            KeyModifiers::SHIFT => match key.code {
                KeyCode::Char('D') => {
                    self.input_buffer = String::new();
                    self.stack = Vec::new();
                }
                _ => {}
            },
            KeyModifiers::NONE => match key.code {
                KeyCode::Char('q') => {
                    return Signal::Exit;
                }
                KeyCode::Char('j' | 'k') => self.swap(),
                KeyCode::Char('d') => {
                    self.input_buffer = String::new();
                }
                KeyCode::Char(c) => self.push_to_buffer(c),
                KeyCode::Backspace => {
                    self.input_buffer.pop();
                }
                KeyCode::Enter if self.input_buffer.len() > 0 => {
                    if let Ok(func) = Function::parse(&self.input_buffer) {
                        if let Some(f) = self.stack.pop() {
                            self.stack.push(func.call_on(f));
                        }
                    } else {
                        self.stack.push(self.input_buffer.parse::<f64>().unwrap());
                    }
                    self.input_buffer = String::new();
                }
                _ => {}
            },
            _ => {}
        }
        return Signal::None;
    }

    fn push_to_buffer(&mut self, c: char) {
        if c == '.' && !self.input_buffer.contains('.') {
            if self.input_buffer.len() == 0 {
                self.input_buffer.push('0');
            }
            self.input_buffer.push(c);
        } else if let Ok(op) = Operator::parse(c) {
            self.stack.push(self.input_buffer.parse::<f64>().unwrap());
            self.input_buffer = String::new();
            self.perform_operation(op);
        } else {
            self.input_buffer.push(c);
        }
    }

    fn swap(&mut self) {
        if let Some(f) = self.stack.pop() {
            self.stack.push(self.input_buffer.parse::<f64>().unwrap());
            self.input_buffer = format!("{}", f);
        }
    }
}
