mod statement;
mod value;

use crate::tokenizer::token::{Span, Token};
use statement::Statement;
use std::{ops::Add, vec};

use self::value::Value;

#[derive(Debug)]
struct IterToken<'a> {
    index: usize,
    tokens: &'a Vec<Token>,
}

impl<'a> IterToken<'a> {
    fn current(&self) -> Option<&'a Token> {
        self.tokens.get(self.index)
    }
    fn current_kind(&self) -> Option<&str> {
        if let Some(k) = self.current() {
            Some(k.kind.as_str())
        } else {
            None
        }
    }
    fn current_with_kind<A: ToString>(&self, compare: A) -> Option<&Token> {
        if let Some(token) = self.current() {
            if compare.to_string() == token.kind {
                return Some(token);
            }
        }

        None
    }
    fn current_is_kind<A: ToString>(&self, compare: A) -> bool {
        if let Some(k) = self.current() {
            compare.to_string() == k.kind
        } else {
            false
        }
    }
    fn next_is_kind<A: ToString>(&self, compare: A) -> bool {
        if let Some(k) = self.tokens.get(self.index + 1) {
            compare.to_string() == k.kind
        } else {
            false
        }
    }
    fn forward(&mut self) {
        self.index = self.index.add(1usize);
    }
}

#[derive(Debug)]
pub struct AST(Statement);

impl AST {
    pub fn parse(tokens: Vec<Token>) -> Self {
        let first_span: usize = if let Some(token) = tokens.first() {
            token.span.start
        } else {
            0
        };
        let last_span: usize = if let Some(token) = tokens.last() {
            token.span.end
        } else {
            0
        };

        let ref mut iter = IterToken {
            index: 0,
            tokens: &tokens,
        };
        let main_statements: Vec<Box<Statement>> = Self::parse_main(iter);

        AST(Statement::Expression {
            span: Span {
                start: first_span,
                end: last_span,
            },
            expresssion: main_statements,
        })
    }

    fn parse_main(iter_tokens: &mut IterToken) -> Vec<Box<Statement>> {
        let mut statemens: Vec<Box<Statement>> = vec![];

        loop {
            if let Some(token_kind) = iter_tokens.current_kind() {
                match token_kind {
                    "keyword" => {
                        statemens.push(Box::new(Self::parse_indetifier(iter_tokens)));
                    }
                    _ => {
                        panic!("kind not supported {:?}", iter_tokens.current_kind());
                    }
                }
            } else {
                break;
            }
        }

        return statemens;
    }

    fn parse_indetifier(iter_tokens: &mut IterToken) -> Statement {
        if let Some(token) = iter_tokens.current_with_kind("keyword") {
            let identifier = Statement::Identifier {
                span: token.span.clone(),
                value: Value::from_str(token.raw.clone()),
            };
            iter_tokens.forward();
            if iter_tokens.current_is_kind("dot") {
                iter_tokens.forward();
                return Self::parse_member_expression(iter_tokens, identifier.clone());
            }
            if iter_tokens.current_is_kind("open_square_bracket") {
                return Self::parse_square_bracket_computer_expression(
                    iter_tokens,
                    identifier.clone(),
                );
            }
            // panic!("kind not supported {:?}", iter_tokens.current_kind());
            // if iter_tokens.current_is_kind("compare")
            return identifier;
        }

        panic!("kind not supported {:?}", iter_tokens.current_kind());
    }

    fn parse_compute(iter_tokens: &mut IterToken) -> Statement {
        if iter_tokens.current_is_kind("string") {
            return Statement::Identifier {
                span: iter_tokens.current().unwrap().span.clone(),
                value: Value::from_str(decode_helpper(iter_tokens.current().unwrap().raw.clone())),
            };
        }
        panic!("kind not supported {:?}", iter_tokens.current_kind());
    }

    fn parse_member_expression(iter_tokens: &mut IterToken, object: Statement) -> Statement {
        let p = Self::parse_indetifier(iter_tokens);
        let m = Statement::MemberExpression {
            span: Span {
                start: object.clone().to_span().start.clone(),
                end: p.clone().to_span().end.clone(),
            },
            object: Box::new(object.clone()),
            property: Box::new(p),
        };
        iter_tokens.forward();
        return m;
    }

    fn parse_square_bracket_computer_expression(
        iter_tokens: &mut IterToken,
        object: Statement,
    ) -> Statement {
        iter_tokens.forward();
        let p = Self::parse_compute(iter_tokens);
        iter_tokens.forward();
        let m = Statement::MemberExpression {
            span: Span {
                start: object.clone().to_span().start.clone(),
                end: p.clone().to_span().end.clone(),
            },
            object: Box::new(object.clone()),
            property: Box::new(p),
        };
        iter_tokens.forward();
        return m;
    }
}

fn decode_helpper(str: String) -> String {
    let mut d = "".to_string();
    let mut cursor = str.as_bytes().iter();
    loop {
        let char_cursor = cursor.next();
        if let Some(c) = char_cursor {
            if c == &92 {
                d.push(cursor.next().unwrap().clone() as char);
                continue;
            }
            d.push(*c as char);
        } else {
            break;
        }
    }

    return d;
}
