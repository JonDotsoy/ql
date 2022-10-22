use crate::tokenizer::token::Span;

#[derive(Debug)]
pub enum Statement {
    Expression {
        span: Span,
        expresssion: Box<Statement>,
    },
    Identifier {
        span: Span,
        value: String,
    },
    BinaryExpression {
        span: Span,
        left: Box<Statement>,
        operator: String,
        rigth: Box<Statement>,
    },
}
