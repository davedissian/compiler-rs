pub mod semantic;

#[derive(Debug)]
pub struct Program(pub Vec<Function>);

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub returns: Type,
    pub statements: Vec<Statement>
}

#[derive(Debug)]
pub enum Statement {
    Block(Vec<Statement>),
    Declare(Type, String, Expression),
    Assign(String, Expression),
    Return(Expression),
    Print(Expression)
}

#[derive(Debug, Clone)]
pub enum Expression {
    Int(i32),
    Char(char),
    Bool(bool),
    Str(String),
    Identifier(String),
    FunctionCall(String),
    Unary(UnaryOp, Box<Expression>),
    Binary(BinaryOp, Box<Expression>, Box<Expression>)
}

#[derive(Debug, Clone)]
pub enum UnaryOp {
    Neg
}

#[derive(Debug, Clone)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod
}

#[derive(Debug, Clone)]
pub enum Type {
    Unknown,
    Error,
    Any,
    Void,
    Int,
    Char,
    Bool,
    Str,
    Array(Box<Type>),
    Pair(Box<Type>, Box<Type>)
}

impl PartialEq for Type {
    fn eq(&self, other: &Type) -> bool {
        match *self {
            Type::Unknown => match *other { Type::Unknown => true, _ => false },
            Type::Error => false,
            Type::Any => true,
            Type::Void => match *other { Type::Void => true, _ => false },
            Type::Int => match *other { Type::Int => true, _ => false },
            Type::Char => match *other { Type::Char => true, _ => false },
            Type::Bool => match *other { Type::Bool => true, _ => false },
            Type::Str => match *other { Type::Str => true, _ => false },
            _ => false
        }
    }
}
