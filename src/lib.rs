pub mod ui;

pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

impl Operator {
    pub fn parse(c: char) -> Result<Self, ParseOperatorError> {
        match c {
            '+' => Ok(Self::Add),
            '-' => Ok(Self::Sub),
            '*' => Ok(Self::Mul),
            '/' => Ok(Self::Div),
            _ => Err(ParseOperatorError(c)),
        }
    }
}

pub struct ParseOperatorError(char);
