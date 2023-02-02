use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::{Calculator, Operator, Signal};

impl Calculator {
    pub fn handle_input(&mut self, key: KeyEvent) -> Signal {
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
                KeyCode::Char('j' | 'k') => self.swap(),
                KeyCode::Char(c) => self.push_to_buffer(c),
                KeyCode::Enter => self.push_buffer_to_stack(),
                KeyCode::Backspace => {
                    self.input_buffer.pop();
                }
                _ => {}
            },
            _ => {}
        }
        return Signal::None;
    }

    fn push_to_buffer(&mut self, c: char) {
        if c.is_ascii_digit() {
            self.input_buffer.push(c);
        } else if c == '.' && !self.input_buffer.contains('.') {
            if self.input_buffer.len() == 0 {
                self.input_buffer.push('0');
            }
            self.input_buffer.push(c);
        } else if let Ok(op) = Operator::parse(c) {
            self.push_buffer_to_stack();
            self.perform_operation(op);
        }
    }

    fn push_buffer_to_stack(&mut self) {
        if self.input_buffer.len() > 0 {
            self.stack.push(self.input_buffer.parse::<f64>().unwrap());
            self.input_buffer = String::new();
        }
    }

    fn swap(&mut self) {
        if let Some(f) = self.stack.pop() {
            self.push_buffer_to_stack();
            self.input_buffer = format!("{}", f);
        }
    }
}
