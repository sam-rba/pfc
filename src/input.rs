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
                            self.push_buffer_to_stack();
                        }
                        self.perform_operation(op);
                    }
                }
                KeyCode::Enter if self.input_buffer.len() > 0 => {
                    self.push_buffer_to_stack();
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
}
