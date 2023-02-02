use crossterm::event::{self, Event};
use std::io;

use pfc::{ui, Calculator, Signal};

fn main() -> io::Result<()> {
    let mut terminal = ui::init_terminal()?;
    let mut calculator = Calculator::default();

    let result = || -> io::Result<()> {
        loop {
            terminal.draw(|f| calculator.draw(f))?;

            if let Event::Key(key) = event::read()? {
                if let Signal::Exit = calculator.handle_input(key) {
                    return Ok(());
                }
            }
        }
    }();

    ui::cleanup_terminal(terminal)?;
    result
}
