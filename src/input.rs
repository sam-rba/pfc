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
                KeyCode::Char('D') => self.clear(),
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
                KeyCode::Enter => {
                    if let Ok(func) = Function::parse(&self.input_buffer) {
                        if let Some(st) = self.stack.pop() {
                            self.stack.push(func.call_on(st));
                        }
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

    // Push a character into the input buffer. If the character is an operator,
    // the particular operation is performed.
    fn push_to_buffer(&mut self, c: char) {
        if c == '.' && !self.input_buffer.contains('.') {
            if self.input_buffer.len() == 0 {
                self.input_buffer.push('0');
            }
            self.input_buffer.push(c);
        } else if let Ok(op) = Operator::parse(c) {
            if self.input_buffer.len() > 0 {
                self.stack.push(self.input_buffer.parse::<f64>().unwrap());
            }
            self.input_buffer = String::new();
            self.perform_operation(op);
        } else {
            self.input_buffer.push(c);
        }
    }

    // Swap the bottom of the stack and the input buffer. If the input buffer
    // is empty, this simply pops from the stack into the input buffer.
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
