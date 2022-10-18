mod token;
use self::token::{Span, Token};

pub struct SourceCursor {
    source: String,
    pos: usize,
}

struct BindOptions {
    /// If detect the char [`\`] indicate the next char is literal.
    scape_char: bool,
}

impl BindOptions {
    /// Creates a new [`BindOptions`].
    fn new() -> Self {
        Self { scape_char: false }
    }

    fn set_scape_char(&mut self, scape_char: bool) -> &mut Self {
        self.scape_char = scape_char;
        self
    }
}

impl Default for BindOptions {
    fn default() -> Self {
        Self::new()
    }
}

pub struct LexerOptions {
    pub break_by_close_square_bracket: bool,
    pub break_by_close_parenthesis: bool,
    pub break_by_close_curly_bracket: bool,
}

impl LexerOptions {
    fn set_break_by_close_square_bracket(
        &mut self,
        break_by_close_square_bracket: bool,
    ) -> &mut Self {
        self.break_by_close_square_bracket = break_by_close_square_bracket;
        self
    }
    fn set_break_by_close_parenthesis(&mut self, break_by_close_parenthesis: bool) -> &mut Self {
        self.break_by_close_parenthesis = break_by_close_parenthesis;
        self
    }
    fn set_break_by_close_curly_bracket(
        &mut self,
        break_by_close_curly_bracket: bool,
    ) -> &mut Self {
        self.break_by_close_curly_bracket = break_by_close_curly_bracket;
        self
    }
}

impl Default for LexerOptions {
    fn default() -> Self {
        Self {
            break_by_close_square_bracket: false,
            break_by_close_parenthesis: false,
            break_by_close_curly_bracket: false,
        }
    }
}

impl SourceCursor {
    pub fn new<A: ToString>(payload: A) -> Self {
        Self {
            pos: 0,
            source: payload.to_string(),
        }
    }

    pub fn at_current_char(&self) -> Option<char> {
        self.source.chars().nth(self.pos)
    }

    pub fn current(&self) -> Option<(usize, char)> {
        if let Some(s) = self.at_current_char() {
            Some((self.pos, s))
        } else {
            None
        }
    }

    pub fn next(&mut self) -> Option<(usize, char)> {
        let current = self.current();
        self.pos = self.pos + 1;
        current
    }

    pub fn prev(&mut self) -> Option<(usize, char)> {
        let current = self.current();
        self.pos = self.pos - 1;
        current
    }

    pub fn next_string(&self, chunk_len: usize) -> String {
        self.source.chars().skip(self.pos).take(chunk_len).collect()
    }
}

pub struct Tokenizer;

#[derive(Debug)]
pub enum LexerError {
    SymbolInvalid(Option<(usize, char)>),
}

impl Tokenizer {
    pub fn lexer<A: ToString>(payload: A) -> Result<Vec<Token>, LexerError> {
        let ref mut source_cursor = SourceCursor::new(payload);
        let ref mut lexer_options = LexerOptions::default();

        Tokenizer::lexer_w(source_cursor, lexer_options)
    }

    fn lexer_w(
        source_cursor: &mut SourceCursor,
        options: &mut LexerOptions,
    ) -> Result<Vec<Token>, LexerError> {
        let mut tokens = Vec::<Token>::new();

        let ref space_matches_fn = |char| matches!(char, ' ' | '\t');
        let ref open_keyword_matches_fn = |char| matches!(char, '$'|'_'|'a' ..= 'z' | 'A' ..= 'Z' );
        let ref keyword_matches_fn =
            |char| matches!(char, '_'|'$'|'0'..='9'|'a' ..= 'z' | 'A' ..= 'Z' );
        let ref open_numeric_matches_fn = |char| matches!(char, '0'..='9');
        let ref numeric_matches_fn = |char| matches!(char, '0'..='9' | '_');
        let ref dot_matches_fn = |char| matches!(char, '.');
        let ref operation_matches_fn =
            |char| matches!(char, '*' | '-' | '/' | '+' | '|' | '&' | '>' | '<');
        let ref string_matches_fn = |char| matches!(char, '\"');
        let ref colon_matches_fn = |char| matches!(char, ':');
        let ref open_curly_bracket_matches_fn = |char| matches!(char, '{');
        let ref close_curly_bracket_matches_fn = |char| matches!(char, '}');
        let ref open_parenthesis_matches_fn = |char| matches!(char, '(');
        let ref close_parenthesis_matches_fn = |char| matches!(char, ')');
        let ref equal_matches_fn = |char| matches!(char, '=');
        let ref open_square_bracket_matches_fn = |char| matches!(char, '[');
        let ref close_square_bracket_matches_fn = |char| matches!(char, ']');
        let ref not_string_matches_fn = |char| !matches!(char, '\"');
        let ref newline_matches_fn = |char| matches!(char, '\r' | '\n');
        let ref open_template_matches_fn = |char| matches!(char, '`');
        let ref template_matches_fn = |char| !matches!(char, '`');

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

            if Tokenizer::lexer_model_by_char_test(source_cursor, open_template_matches_fn) {
                source_cursor.next();
                let response_tokens = Tokenizer::lexer_model_by_char_bind_template(
                    source_cursor,
                    "template",
                    template_matches_fn,
                    Some(&BindOptions::new().set_scape_char(true)),
                )?;
                source_cursor.next();
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

            if Tokenizer::lexer_model_by_char_test(source_cursor, open_curly_bracket_matches_fn) {
                let response_tokens = Tokenizer::lexer_model_by_char_bind(
                    source_cursor,
                    "open_curly_bracket",
                    open_curly_bracket_matches_fn,
                    None,
                )?;
                tokens.extend(response_tokens);
                tokens.extend(Tokenizer::lexer_w(
                    source_cursor,
                    &mut LexerOptions::default(),
                )?);
                continue;
            }

            if Tokenizer::lexer_model_by_char_test(source_cursor, close_curly_bracket_matches_fn) {
                let response_tokens = Tokenizer::lexer_model_by_char_bind(
                    source_cursor,
                    "close_curly_bracket",
                    close_curly_bracket_matches_fn,
                    None,
                )?;
                tokens.extend(response_tokens);
                if options.break_by_close_curly_bracket {
                    break;
                }
                continue;
            }

            if Tokenizer::lexer_model_by_char_test(source_cursor, open_parenthesis_matches_fn) {
                let response_tokens = Tokenizer::lexer_model_by_char_bind(
                    source_cursor,
                    "open_parenthesis",
                    open_parenthesis_matches_fn,
                    None,
                )?;
                tokens.extend(response_tokens);
                tokens.extend(Tokenizer::lexer_w(
                    source_cursor,
                    &mut LexerOptions::default(),
                )?);
                continue;
            }

            if Tokenizer::lexer_model_by_char_test(source_cursor, close_parenthesis_matches_fn) {
                let response_tokens = Tokenizer::lexer_model_by_char_bind(
                    source_cursor,
                    "close_parenthesis",
                    close_parenthesis_matches_fn,
                    None,
                )?;
                tokens.extend(response_tokens);
                if options.break_by_close_parenthesis {
                    break;
                }
                continue;
            }

            if Tokenizer::lexer_model_by_char_test(source_cursor, open_square_bracket_matches_fn) {
                let response_tokens = Tokenizer::lexer_model_by_char_bind(
                    source_cursor,
                    "open_square_bracket",
                    open_square_bracket_matches_fn,
                    None,
                )?;
                tokens.extend(response_tokens);
                tokens.extend(Tokenizer::lexer_w(
                    source_cursor,
                    &mut LexerOptions::default(),
                )?);
                continue;
            }

            if Tokenizer::lexer_model_by_char_test(source_cursor, close_square_bracket_matches_fn) {
                let response_tokens = Tokenizer::lexer_model_by_char_bind(
                    source_cursor,
                    "close_square_bracket",
                    close_square_bracket_matches_fn,
                    None,
                )?;
                tokens.extend(response_tokens);
                if options.break_by_close_square_bracket {
                    break;
                }
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
                source_cursor.next();
                let response_tokens = Tokenizer::lexer_model_by_char_bind(
                    source_cursor,
                    "string",
                    not_string_matches_fn,
                    Some(BindOptions::new().set_scape_char(true)),
                )?;
                source_cursor.next();
                tokens.extend(response_tokens);
                continue;
            }

            return Err(LexerError::SymbolInvalid(source_cursor.current()));

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

    fn lexer_model_by_char_test<F: Fn(char) -> bool>(
        source_cursor: &mut SourceCursor,
        matches_fn: F,
    ) -> bool {
        if let Some(c) = source_cursor.at_current_char() {
            matches_fn(c)
        } else {
            false
        }
    }

    fn lexer_model_by_char_bind<A: ToString, F: Fn(char) -> bool>(
        source_cursor: &mut SourceCursor,
        kind: A,
        matches_fn: F,
        options: Option<&BindOptions>,
    ) -> Result<Vec<Token>, LexerError> {
        let scape_char = match options {
            Some(bind_options) => bind_options.scape_char,
            _ => false,
        };

        let span_start = source_cursor.pos;

        while let Some((_, char)) = source_cursor.current() {
            if scape_char && char == '\\' {
                source_cursor.next();
                source_cursor.next();
                continue;
            }
            if matches_fn(char) {
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

    fn lexer_model_by_char_bind_template<A: ToString, F: Fn(char) -> bool>(
        source_cursor: &mut SourceCursor,
        kind: A,
        matches_fn: F,
        options: Option<&BindOptions>,
    ) -> Result<Vec<Token>, LexerError> {
        let mut tokens = Vec::<Token>::default();
        let scape_char = match options {
            Some(bind_options) => bind_options.scape_char,
            _ => false,
        };

        let mut span_start = source_cursor.pos;

        while let Some((_, char)) = source_cursor.current() {
            if scape_char && char == '\\' {
                source_cursor.next();
                source_cursor.next();
                continue;
            }

            if source_cursor.next_string(2) == "${".to_string() {
                let value = source_cursor
                    .source
                    .get(span_start..source_cursor.pos)
                    .unwrap()
                    .to_string();
                tokens.push(Token {
                    kind: kind.to_string(),
                    raw: value,
                    span: Span {
                        start: span_start,
                        end: source_cursor.pos,
                    },
                });
                tokens.extend(Tokenizer::lexer_w(
                    source_cursor,
                    LexerOptions::default().set_break_by_close_curly_bracket(true),
                )?);
                span_start = source_cursor.pos;
                continue;
            }

            if matches_fn(char) {
                source_cursor.next();
                continue;
            }

            break;
        }

        let span_end = source_cursor.pos;

        let value: String = source_cursor
            .source
            .get(span_start..span_end)
            .unwrap_or("")
            .to_string();

        tokens.push(Token {
            kind: kind.to_string(),
            raw: value,
            span: Span {
                start: span_start,
                end: span_end,
            },
        });

        Ok(tokens)
    }
}
