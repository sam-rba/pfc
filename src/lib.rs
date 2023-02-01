pub mod ui;

enum Token {
    Operand(f64),
    Operator(Operator),
}

enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}
