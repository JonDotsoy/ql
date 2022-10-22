mod statement;

use statement::Statement;

use crate::tokenizer::token::{Span, Token};

#[derive(Debug)]
pub struct AST(Statement);

impl AST {
    pub fn parse(_tokens: Vec<Token>) -> Self {
        AST(Statement::Expression {
            span: Span { start: 0, end: 1 },
            expresssion: Box::new(Statement::Identifier {
                span: Span { start: 13, end: 14 },
                value: "$".to_string(),
            }),
        })
    }
}
