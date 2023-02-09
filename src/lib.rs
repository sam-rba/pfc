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

enum Function {
    Sin,
    Cos,
    Tan,
}

impl Function {
    fn parse(s: &str) -> Result<Self, ParseFunctionError> {
        match s {
            "sin" => Ok(Self::Sin),
            "cos" => Ok(Self::Cos),
            "tan" => Ok(Self::Tan),
            _ => Err(ParseFunctionError(s.to_string())),
        }
    }

    fn call_on(&self, f: f64) -> f64 {
        match self {
            Self::Sin => f.sin(),
            Self::Cos => f.cos(),
            Self::Tan => f.tan(),
        }
    }
}

struct ParseFunctionError(String);

pub enum Signal {
    None,
    Exit,
}
