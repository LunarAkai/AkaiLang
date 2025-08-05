#[derive(Debug, Clone)]
pub enum Op {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl Op {
    pub fn eval(&self) -> String {
        let text: &str = match self {
           Op::Add => "+",
           Op::Subtract => "-",
           Op::Multiply => "*",
           Op::Divide => "/",
        };
        text.to_string()
    }
}