mod input;
pub mod ui;

#[derive(Default)]
pub struct Calculator {
    stack: Vec<f64>,
    input_buffer: String,
}

impl Calculator {
    fn op(&mut self, op: Operator) {
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

    fn func(&self) -> impl Fn(f64) -> f64 {
        match self {
            Self::Sin => |f: f64| f.sin(),
            Self::Cos => |f: f64| f.cos(),
            Self::Tan => |f: f64| f.tan(),
        }
    }
}

struct ParseFunctionError(String);

pub enum Signal {
    None,
    Exit,
}
