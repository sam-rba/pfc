use std::{
    f64::consts::{E, PI},
    fmt::{self, Display, Formatter},
};

mod function;
mod input;
pub mod ui;

pub use function::Function;

#[derive(Default)]
pub struct Calculator {
    stack: Vec<f64>,
    input_buffer: String,
    angle_mode: AngleMode,
}

impl Calculator {
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
        self.stack.push(match op {
            Operator::Add => lhs + rhs,
            Operator::Sub => lhs - rhs,
            Operator::Mul => lhs * rhs,
            Operator::Div => lhs / rhs,
            Operator::Exp => lhs.powf(rhs),
        });
    }
}

#[derive(Default, Copy, Clone, PartialEq)]
pub enum AngleMode {
    #[default]
    Degrees,
    Radians,
}

impl AngleMode {
    fn toggle(&self) -> Self {
        match self {
            Self::Degrees => Self::Radians,
            Self::Radians => Self::Degrees,
        }
    }
}

impl Display for AngleMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Degrees => "deg",
                Self::Radians => "rad",
            }
        )
    }
}

enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Exp,
}

impl Operator {
    fn parse(c: char) -> Result<Self, ParseOperatorError> {
        match c {
            '+' => Ok(Self::Add),
            '-' => Ok(Self::Sub),
            '*' => Ok(Self::Mul),
            '/' => Ok(Self::Div),
            '^' => Ok(Self::Exp),
            _ => Err(ParseOperatorError(c)),
        }
    }
}

struct ParseOperatorError(char);

enum Constant {
    Pi, // Archimedes’ constant (π)
    E,  // Euler's number (e)
}

impl Constant {
    fn parse(s: &str) -> Result<Self, ParseConstantError> {
        match s {
            "pi" => Ok(Self::Pi),
            "e" => Ok(Self::E),
            _ => Err(ParseConstantError(s.to_string())),
        }
    }

    fn value(&self) -> f64 {
        match self {
            Self::Pi => PI,
            Self::E => E,
        }
    }
}

struct ParseConstantError(String);

pub enum Signal {
    None,
    Exit,
}
