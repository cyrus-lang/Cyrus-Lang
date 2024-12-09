use crate::token::*;

#[derive(Debug)]
pub enum Node {
    Program(Program),
    Statement(Statement),
    Expression(Expression),
}

#[derive(Debug)]
pub struct Program {
    pub body: Vec<Statement>,
    pub span: Span,
}

impl Default for Program {
    fn default() -> Self {
        Self::new()
    }
}

impl Program {
    pub fn new() -> Self {
        Self { body: vec![], span: Span::new_empty_span() }
    }
}

#[derive(Debug, Clone)]
pub enum Expression {
    Identifier(Identifier),
    Literal(Literal),
    Prefix(UnaryExpression),
    Infix(BinaryExpression),
    FunctionCall(FunctionCall),
    UnaryOperator(UnaryOperator),
}

#[derive(Debug, Clone)]
pub enum UnaryOperatorType {
    PreIncrement,
    PreDecrement,
    PostIncrement,
    PostDecrement,
}

#[derive(Debug, Clone)]
pub struct UnaryOperator {
    pub identifer: Identifier,
    pub ty: UnaryOperatorType,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct FunctionCall {
    pub call: Box<Expression>,
    pub arguments: Vec<Expression>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct Identifier {
    pub name: String,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum Literal {
    Integer(Integer),
    Float(Float),
    Boolean(Boolean),
    String(StringType),
}

#[derive(Debug, Clone)]
pub struct UnaryExpression {
    pub operator: Token,
    pub operand: Box<Expression>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct BinaryExpression {
    pub operator: Token,
    pub left: Box<Expression>,
    pub right: Box<Expression>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct Integer {
    pub raw: i64,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct Float {
    pub raw: f64,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct Boolean {
    pub raw: bool,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct StringType {
    pub raw: String,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct Array {
    pub elements: Vec<Expression>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct Hash {
    pub pairs: Vec<(Expression, Expression)>,
    pub span: Span,
}

pub fn format_expressions(exprs: &Vec<Expression>) -> String {
    exprs.iter().map(|expr| expr.to_string()).collect()
}



#[derive(Debug, Clone)]
pub enum Statement {
    VariableDeclaration(Variable),
    Expression(Expression),
    If(If),
    Return(Return),
    Function(Function),
    For(For),
    Match(Match),
    Struct(Struct),
    Package(Package),
    ImportPackage(ImportPackage),
}

pub fn format_statements(stmts: &Vec<Statement>) -> String {
    stmts.iter().map(|stmt| stmt.to_string()).collect()
}

#[derive(Debug, Clone)]
pub struct Return {
    pub argument: Expression,
    pub span: Span,
}


#[derive(Debug, Clone)]
pub struct Package {
    pub name: Identifier,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct ImportPackage {
    pub name: Identifier,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct Struct {
    pub name: Identifier,
    pub fields: Option<Field>,
    pub extends: Option<Box<Struct>>,
    pub methods: Option<Function>,
}

#[derive(Debug, Clone)]
pub struct Field {
    pub name: Identifier,
    pub ty: Literal,
    pub default_value: Literal,
}

#[derive(Debug, Clone)]
pub struct For {
    pub initializer: Option<Variable>,
    pub condition: Option<Expression>,
    pub increment: Option<Expression>,
    pub body: Box<BlockStatement>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct Match {
    pub value: Expression,
    pub sections: Option<Vec<MatchPattern>>,
    pub default: BlockStatement,
    pub body: Box<BlockStatement>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct MatchPattern {
    pub raw: Expression,
    pub span: Span
}

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub params: Vec<Identifier>,
    pub body: Box<BlockStatement>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct BlockStatement {
    pub body: Vec<Statement>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct Variable {
    pub identifier: Token,
    pub expr: Expression,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct If {
    pub condition: Expression,
    pub consequent: Box<BlockStatement>,
    pub branches: Vec<If>,
    pub alternate: Option<Box<BlockStatement>>,
    pub span: Span,
}
