use crate::tokenizer::token::Span;

use super::value::Value;

#[derive(Debug, Clone)]
pub enum Statement {
    Expression {
        span: Span,
        expresssion: Vec<Box<Self>>,
    },
    Identifier {
        span: Span,
        value: Value,
    },
    // IdentifierExpression {
    //     span: Span,
    //     expression: Box<Self>,
    // },
    MemberExpression {
        span: Span,
        object: Box<Self>,
        property: Box<Self>,
    },
    // BinaryExpression {
    //     span: Span,
    //     left: Box<Statement>,
    //     operator: String,
    //     rigth: Box<Statement>,
    // },
}

impl Statement {
    pub fn to_span(self) -> Span {
        match self {
            Self::Identifier { span, .. } => span,
            Self::MemberExpression { span, .. } => span,
            Self::Expression { span, .. } => span,
            // Self::IdentifierExpression { span, .. } => span,
        }
    }
}
