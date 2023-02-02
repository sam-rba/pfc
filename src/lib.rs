mod input;
pub mod ui;

#[derive(Default)]
pub struct Calculator {
    stack: Vec<f64>,
    input_buffer: String,
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

pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Exp,
}

impl Operator {
    pub fn parse(c: char) -> Result<Self, ParseOperatorError> {
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

pub struct ParseOperatorError(char);

pub enum Signal {
    None,
    Exit,
}
