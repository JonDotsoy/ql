mod bind_options;
mod direction;
mod lexer_error;
mod lexer_options;
mod source_cursor;
mod to_direction;
pub mod token;
mod value;

use self::bind_options::BindOptions;
use self::direction::Direction;
use self::lexer_error::LexerError;
use self::lexer_options::LexerOptions;
use self::source_cursor::SourceCursor;
use self::token::{Span, Token};

pub struct Tokenizer;

impl Tokenizer {
    pub fn lexer<A: ToString>(payload: A) -> Result<Vec<Token>, LexerError> {
        let ref mut source_cursor = SourceCursor::new(payload);
        let ref mut lexer_options = LexerOptions::default();

        Tokenizer::lexer_w(source_cursor, lexer_options)
    }

    fn lexer_w(
        source_cursor: &mut SourceCursor,
        options: &mut LexerOptions,
    ) -> Result<Vec<Token>, lexer_error::LexerError> {
        let mut tokens = Vec::<Token>::new();

        let ref space_matches_fn =
            |source_cursor: &SourceCursor| matches!(source_cursor.get_current_char(), ' ' | '\t');
        let ref open_keyword_matches_fn = |source_cursor: &SourceCursor| matches!(source_cursor.get_current_char(), '$'|'_'|'a' ..= 'z' | 'A' ..= 'Z' );
        let ref keyword_matches_fn = |source_cursor: &SourceCursor| matches!(source_cursor.get_current_char(), '_'|'$'|'0'..='9'|'a' ..= 'z' | 'A' ..= 'Z' );
        let ref open_numeric_matches_fn =
            |source_cursor: &SourceCursor| matches!(source_cursor.get_current_char(), '0'..='9');
        let ref numeric_matches_fn = |source_cursor: &SourceCursor| {
            matches!(source_cursor.get_current_char(), '0'..='9' | '_')
        };
        let ref dot_matches_fn =
            |source_cursor: &SourceCursor| matches!(source_cursor.get_current_char(), '.');
        let ref operation_matches_fn = |source_cursor: &SourceCursor| {
            matches!(
                source_cursor.get_current_char(),
                '*' | '-' | '/' | '+' | '|' | '&' | '>' | '<'
            )
        };
        let ref string_matches_fn =
            |source_cursor: &SourceCursor| matches!(source_cursor.get_current_char(), '\"');
        let ref colon_matches_fn =
            |source_cursor: &SourceCursor| matches!(source_cursor.get_current_char(), ':');
        let ref open_curly_bracket_matches_fn =
            |source_cursor: &SourceCursor| matches!(source_cursor.get_current_char(), '{');
        let ref close_curly_bracket_matches_fn =
            |source_cursor: &SourceCursor| matches!(source_cursor.get_current_char(), '}');
        let ref open_parenthesis_matches_fn =
            |source_cursor: &SourceCursor| matches!(source_cursor.get_current_char(), '(');
        let ref close_parenthesis_matches_fn =
            |source_cursor: &SourceCursor| matches!(source_cursor.get_current_char(), ')');
        let ref equal_matches_fn =
            |source_cursor: &SourceCursor| matches!(source_cursor.get_current_char(), '=');
        let ref open_square_bracket_matches_fn =
            |source_cursor: &SourceCursor| matches!(source_cursor.get_current_char(), '[');
        let ref close_square_bracket_matches_fn =
            |source_cursor: &SourceCursor| matches!(source_cursor.get_current_char(), ']');
        let ref not_string_matches_fn =
            |source_cursor: &SourceCursor| !matches!(source_cursor.get_current_char(), '\"');
        let ref newline_matches_fn =
            |source_cursor: &SourceCursor| matches!(source_cursor.get_current_char(), '\r' | '\n');
        let ref open_template_matches_fn =
            |source_cursor: &SourceCursor| matches!(source_cursor.get_current_char(), '`');
        let ref template_matches_fn =
            |source_cursor: &SourceCursor| !matches!(source_cursor.get_current_char(), '`');

        while let Some(_) = source_cursor.current() {
            if Tokenizer::lexer_model_by_char_test(source_cursor, space_matches_fn) {
                Tokenizer::lexer_model_by_char_bind(
                    source_cursor,
                    "space",
                    space_matches_fn,
                    None,
                )?;
                continue;
            }

            if Tokenizer::lexer_model_by_char_test(source_cursor, close_curly_bracket_matches_fn) {
                if options.template_break_by_close_curly_bracket {
                    tokens.push(source_cursor.create_token(
                        "template_close_expression",
                        Direction::Current,
                        Direction::Next(1),
                    ));
                    break;
                }
                tokens.push(source_cursor.create_token(
                    "close_curly_bracket",
                    Direction::Current,
                    Direction::Next(1),
                ));
                // let response_tokens = Tokenizer::lexer_model_by_char_bind(
                //     source_cursor,
                //     "close_curly_bracket",
                //     close_curly_bracket_matches_fn,
                //     None,
                // )?;
                // tokens.extend(response_tokens);
                if options.break_by_close_curly_bracket {
                    break;
                }
                continue;
            }

            if Tokenizer::lexer_model_by_char_test(source_cursor, close_parenthesis_matches_fn) {
                tokens.push(source_cursor.create_token(
                    "close_parenthesis",
                    Direction::Current,
                    Direction::Next(1),
                ));
                // let response_tokens = Tokenizer::lexer_model_by_char_bind(
                //     source_cursor,
                //     "close_parenthesis",
                //     close_parenthesis_matches_fn,
                //     None,
                // )?;
                // tokens.extend(response_tokens);
                if options.break_by_close_parenthesis {
                    break;
                }
                continue;
            }

            if Tokenizer::lexer_model_by_char_test(source_cursor, close_square_bracket_matches_fn) {
                tokens.push(source_cursor.create_token(
                    "close_square_bracket",
                    Direction::Current,
                    Direction::Next(1),
                ));
                // let response_tokens = Tokenizer::lexer_model_by_char_bind(
                //     source_cursor,
                //     "close_square_bracket",
                //     close_square_bracket_matches_fn,
                //     None,
                // )?;
                // tokens.extend(response_tokens);
                if options.break_by_close_square_bracket {
                    break;
                }
                continue;
            }

            if Tokenizer::lexer_model_by_char_test(source_cursor, open_curly_bracket_matches_fn) {
                tokens.push(source_cursor.create_token(
                    "open_curly_bracket",
                    Direction::Current,
                    Direction::Next(1),
                ));
                // let response_tokens = Tokenizer::lexer_model_by_char_bind(
                //     source_cursor,
                //     "open_curly_bracket",
                //     open_curly_bracket_matches_fn,
                //     None,
                // )?;
                // tokens.extend(response_tokens);
                tokens.extend(Tokenizer::lexer_w(
                    source_cursor,
                    &mut LexerOptions::default().set_break_by_close_curly_bracket(true),
                )?);
                continue;
            }

            if Tokenizer::lexer_model_by_char_test(source_cursor, open_parenthesis_matches_fn) {
                tokens.push(source_cursor.create_token(
                    "open_curly_bracket",
                    Direction::Current,
                    Direction::Next(1),
                ));
                // let response_tokens = Tokenizer::lexer_model_by_char_bind(
                //     source_cursor,
                //     "open_parenthesis",
                //     open_parenthesis_matches_fn,
                //     None,
                // )?;
                // tokens.extend(response_tokens);
                tokens.extend(Tokenizer::lexer_w(
                    source_cursor,
                    &mut LexerOptions::default().set_break_by_close_parenthesis(true),
                )?);
                continue;
            }

            if Tokenizer::lexer_model_by_char_test(source_cursor, open_square_bracket_matches_fn) {
                // let response_tokens = Tokenizer::lexer_model_by_char_bind(
                //     source_cursor,
                //     "open_square_bracket",
                //     open_square_bracket_matches_fn,
                //     None,
                // )?;
                // tokens.extend(response_tokens);
                tokens.push(source_cursor.create_token(
                    "open_square_bracket",
                    Direction::Current,
                    Direction::Next(1),
                ));
                tokens.extend(Tokenizer::lexer_w(
                    source_cursor,
                    &mut LexerOptions::default().set_break_by_close_square_bracket(true),
                )?);
                continue;
            }

            if Tokenizer::lexer_model_by_char_test(source_cursor, open_template_matches_fn) {
                let response_tokens = Tokenizer::lexer_model_by_char_bind_template(
                    source_cursor,
                    "template",
                    template_matches_fn,
                    Some(&BindOptions::new().set_scape_char(true)),
                )?;
                tokens.extend(response_tokens);
                continue;
            }

            if Tokenizer::lexer_model_by_char_test(source_cursor, colon_matches_fn) {
                let response_tokens = Tokenizer::lexer_model_by_char_bind(
                    source_cursor,
                    "colon",
                    colon_matches_fn,
                    None,
                )?;
                tokens.extend(response_tokens);
                continue;
            }
            if Tokenizer::lexer_model_by_char_test(source_cursor, equal_matches_fn) {
                let response_tokens = Tokenizer::lexer_model_by_char_bind(
                    source_cursor,
                    "equal",
                    equal_matches_fn,
                    None,
                )?;
                tokens.extend(response_tokens);
                continue;
            }
            if Tokenizer::lexer_model_by_char_test(source_cursor, newline_matches_fn) {
                let response_tokens = Tokenizer::lexer_model_by_char_bind(
                    source_cursor,
                    "newline",
                    newline_matches_fn,
                    None,
                )?;
                tokens.extend(response_tokens);
                continue;
            }

            if Tokenizer::lexer_model_by_char_test(source_cursor, dot_matches_fn) {
                let response_tokens = Tokenizer::lexer_model_by_char_bind(
                    source_cursor,
                    "dot",
                    dot_matches_fn,
                    None,
                )?;
                tokens.extend(response_tokens);
                continue;
            }

            if Tokenizer::lexer_model_by_char_test(source_cursor, open_keyword_matches_fn) {
                let response_tokens = Tokenizer::lexer_model_by_char_bind(
                    source_cursor,
                    "keyword",
                    keyword_matches_fn,
                    None,
                )?;
                tokens.extend(response_tokens);
                continue;
            }

            if Tokenizer::lexer_model_by_char_test(source_cursor, open_numeric_matches_fn) {
                let response_tokens = Tokenizer::lexer_model_by_char_bind(
                    source_cursor,
                    "numeric",
                    numeric_matches_fn,
                    None,
                )?;
                tokens.extend(response_tokens);
                continue;
            }

            if Tokenizer::lexer_model_by_char_test(source_cursor, operation_matches_fn) {
                let response_tokens = Tokenizer::lexer_model_by_char_bind(
                    source_cursor,
                    "operation",
                    operation_matches_fn,
                    None,
                )?;
                tokens.extend(response_tokens);
                continue;
            }

            if Tokenizer::lexer_model_by_char_test(source_cursor, string_matches_fn) {
                source_cursor.forward(1);
                let response_tokens = Tokenizer::lexer_model_by_char_bind(
                    source_cursor,
                    "string",
                    not_string_matches_fn,
                    Some(BindOptions::new().set_scape_char(true)),
                )?;
                source_cursor.forward(1);
                tokens.extend(response_tokens);
                continue;
            }

            return Err(lexer_error::LexerError::SymbolInvalid(
                source_cursor.current(),
            ));

            // let other_symbol = Token {
            //     kind: "symbol".to_string(),
            //     raw: {
            //         let mut s = String::default();
            //         s.push(source_cursor.at_current_char().unwrap());
            //         s
            //     },
            //     span: Span {
            //         start: source_cursor.pos,
            //         end: source_cursor.pos,
            //     },
            // };
            // source_cursor.next();
            // tokens.push(other_symbol);
        }

        // println!("{:#?}", tokens);

        Ok(tokens)
    }

    fn lexer_model_by_char_test<F: Fn(&SourceCursor) -> bool>(
        source_cursor: &mut SourceCursor,
        matches_fn: F,
    ) -> bool {
        if let Some(_) = source_cursor.at_current_char() {
            matches_fn(&source_cursor)
        } else {
            false
        }
    }

    fn lexer_model_by_char_bind<A: ToString, F: Fn(&SourceCursor) -> bool>(
        source_cursor: &mut SourceCursor,
        kind: A,
        matches_fn: F,
        options: Option<&BindOptions>,
    ) -> Result<Vec<Token>, lexer_error::LexerError> {
        let scape_char = match options {
            Some(bind_options) => bind_options.scape_char,
            _ => false,
        };

        let span_start = source_cursor.pos;

        while let Some((_, char)) = source_cursor.current() {
            if scape_char && char == '\\' {
                source_cursor.forward(2);
                continue;
            }
            if matches_fn(source_cursor) {
                source_cursor.next();
                continue;
            }
            break;
        }

        let span_end = source_cursor.pos;

        let value = source_cursor
            .source
            .get(span_start..span_end)
            .unwrap()
            .to_string();

        Ok(vec![Token {
            kind: kind.to_string(),
            raw: value,
            span: Span {
                start: span_start,
                end: span_end,
            },
        }])
    }

    fn lexer_model_by_char_bind_template<A: ToString, F: Fn(&SourceCursor) -> bool>(
        source_cursor: &mut SourceCursor,
        _kind: A,
        matches_fn: F,
        options: Option<&BindOptions>,
    ) -> Result<Vec<Token>, LexerError> {
        let mut tokens = Vec::<Token>::default();
        let scape_char = match options {
            Some(bind_options) => bind_options.scape_char,
            _ => false,
        };

        tokens.push(source_cursor.create_token(
            "template_start",
            Direction::Current,
            Direction::Next(1),
        ));

        let mut span_start = source_cursor.pos;

        while let Some(_) = source_cursor.current() {
            if scape_char && source_cursor.get_current_char() == '\\' {
                source_cursor.forward(Direction::Next(2));
                continue;
            }

            if source_cursor.get(0, 2) == "${" {
                tokens.push(source_cursor.create_token("template", Direction::Pos(span_start), 0));
                tokens.push(source_cursor.create_token("template_start_expression", 0, 2));

                tokens.extend(Tokenizer::lexer_w(
                    source_cursor,
                    LexerOptions::default().set_template_break_by_close_curly_bracket(true),
                )?);

                span_start = source_cursor.pos;
                continue;
            }

            if matches_fn(source_cursor) {
                source_cursor.next();
                continue;
            }

            break;
        }

        tokens.push(source_cursor.create_token("template", Direction::Pos(span_start), 0));
        tokens.push(source_cursor.create_token("template_close", 0, 1));

        Ok(tokens)
    }
}
