use crossterm::event::{self, Event};
use std::io;
use tui::{backend::Backend, Terminal};

use pfc::{ui, Calculator, Signal};

fn main() -> io::Result<()> {
    let calculator = Calculator::default();
    let mut terminal = ui::init_terminal()?;
    let result = run(calculator, &mut terminal);
    ui::cleanup_terminal(terminal)?;
    result
}

fn run<B: Backend>(mut calculator: Calculator, terminal: &mut Terminal<B>) -> io::Result<()> {
    loop {
        terminal.draw(|f| calculator.draw(f))?;

        if let Event::Key(key) = event::read()? {
            if let Signal::Exit = calculator.handle_input(key) {
                return Ok(());
            }
        }
    }
}
