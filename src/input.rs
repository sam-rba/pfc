use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::{Calculator, Constant, Function, Operator, Signal};

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
                KeyCode::Char('Q') => {
                    return Signal::Exit;
                }
                KeyCode::Char('J' | 'K') => self.swap(),
                KeyCode::Char('D') => {
                    self.input_buffer = String::new();
                }
                KeyCode::Char('C') => self.clear(),
                KeyCode::Char('A') => {
                    self.angle_mode = self.angle_mode.toggle();
                }
                _ => {}
            },
            KeyModifiers::NONE => match key.code {
                KeyCode::Char(c) => self.push_to_buffer(c),
                KeyCode::Backspace => {
                    self.input_buffer.pop();
                }
                KeyCode::Enter => {
                    if let Ok(func) = Function::parse(&self.input_buffer) {
                        if let Some(val) = self.stack.pop() {
                            self.stack.push(func.call(val, self.angle_mode));
                        }
                    } else if let Ok(c) = Constant::parse(&self.input_buffer) {
                        self.stack.push(c.value());
                    } else if let Ok(bu) = self.input_buffer.parse::<f64>() {
                        self.stack.push(bu);
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
            if let Ok(c) = Constant::parse(&self.input_buffer) {
                self.stack.push(c.value());
            } else if let Ok(f) = self.input_buffer.parse::<f64>() {
                self.stack.push(f);
            }
            self.input_buffer = String::new();
            self.perform_operation(op);
        } else {
            self.input_buffer.push(c);
        }
    }

    fn swap(&mut self) {
        if let Some(st) = self.stack.pop() {
            if let Ok(bu) = self.input_buffer.parse::<f64>() {
                self.stack.push(bu);
            }
            self.input_buffer = format!("{}", st);
        }
    }

    // Clear stack and input buffer.
    fn clear(&mut self) {
        self.input_buffer = String::new();
        self.stack = Vec::new();
    }
}
