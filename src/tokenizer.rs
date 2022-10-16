#[derive(Debug)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug)]
pub struct Token {
    pub kind: String,
    pub raw: String,
    pub span: Span,
}

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
}

pub struct Tokenizer {}

#[derive(Debug)]
pub enum LexerError {
    SymbolInvalid(usize),
}

impl Tokenizer {
    pub fn lexer<A: ToString>(payload: A) -> Result<Vec<Token>, LexerError> {
        let source_cursor = &mut SourceCursor::new(payload);
        let mut tokens = Vec::<Token>::new();

        let ref trim_matches_fn = |char| matches!(char, ' ' | '\t');
        let ref open_keyword_matches_fn = |char| matches!(char, '$'|'_'|'a' ..= 'z' | 'A' ..= 'Z' );
        let ref keyword_matches_fn =
            |char| matches!(char, '_'|'$'|'0'..='9'|'a' ..= 'z' | 'A' ..= 'Z' );
        let ref open_numeric_matches_fn = |char| matches!(char, '0'..='9');
        let ref numeric_matches_fn = |char| matches!(char, '0'..='9' | '_');
        let ref dot_matches_fn = |char| matches!(char, '.');
        let ref operation_matches_fn =
            |char| matches!(char, '*' | '-' | '/' | '+' | '|' | '>' | '<');
        let ref string_matches_fn = |char| matches!(char, '\"');
        let ref colon_matches_fn = |char| matches!(char, ':');
        let ref open_parenthesis_matches_fn = |char| matches!(char, '(');
        let ref close_parenthesis_matches_fn = |char| matches!(char, ')');
        let ref equal_matches_fn = |char| matches!(char, '=');
        let ref open_bracket_matches_fn = |char| matches!(char, '[');
        let ref close_bracket_matches_fn = |char| matches!(char, ']');
        let ref not_string_matches_fn = |char| !matches!(char, '\"');
        let ref newline_matches_fn = |char| matches!(char, '\r' | '\n');

        while let Some(_) = source_cursor.current() {
            if Tokenizer::lexer_model_by_char_test(source_cursor, trim_matches_fn) {
                Tokenizer::lexer_model_by_char_bind(source_cursor, "space", trim_matches_fn, None)?;
                continue;
            }

            if Tokenizer::lexer_model_by_char_test(source_cursor, colon_matches_fn) {
                let token = Tokenizer::lexer_model_by_char_bind(
                    source_cursor,
                    "colon",
                    colon_matches_fn,
                    None,
                )?;
                tokens.push(token);
                continue;
            }
            if Tokenizer::lexer_model_by_char_test(source_cursor, equal_matches_fn) {
                let token = Tokenizer::lexer_model_by_char_bind(
                    source_cursor,
                    "equal",
                    equal_matches_fn,
                    None,
                )?;
                tokens.push(token);
                continue;
            }
            if Tokenizer::lexer_model_by_char_test(source_cursor, newline_matches_fn) {
                let token = Tokenizer::lexer_model_by_char_bind(
                    source_cursor,
                    "newline",
                    newline_matches_fn,
                    None,
                )?;
                tokens.push(token);
                continue;
            }

            if Tokenizer::lexer_model_by_char_test(source_cursor, open_parenthesis_matches_fn) {
                let token = Tokenizer::lexer_model_by_char_bind(
                    source_cursor,
                    "open_parenthesis",
                    open_parenthesis_matches_fn,
                    None,
                )?;
                tokens.push(token);
                continue;
            }

            if Tokenizer::lexer_model_by_char_test(source_cursor, close_parenthesis_matches_fn) {
                let token = Tokenizer::lexer_model_by_char_bind(
                    source_cursor,
                    "close_parenthesis",
                    close_parenthesis_matches_fn,
                    None,
                )?;
                tokens.push(token);
                continue;
            }

            if Tokenizer::lexer_model_by_char_test(source_cursor, open_bracket_matches_fn) {
                let token = Tokenizer::lexer_model_by_char_bind(
                    source_cursor,
                    "open_bracket",
                    open_bracket_matches_fn,
                    None,
                )?;
                tokens.push(token);
                continue;
            }

            if Tokenizer::lexer_model_by_char_test(source_cursor, close_bracket_matches_fn) {
                let token = Tokenizer::lexer_model_by_char_bind(
                    source_cursor,
                    "close_bracket",
                    close_bracket_matches_fn,
                    None,
                )?;
                tokens.push(token);
                continue;
            }

            if Tokenizer::lexer_model_by_char_test(source_cursor, dot_matches_fn) {
                let token = Tokenizer::lexer_model_by_char_bind(
                    source_cursor,
                    "dot",
                    dot_matches_fn,
                    None,
                )?;
                tokens.push(token);
                continue;
            }

            if Tokenizer::lexer_model_by_char_test(source_cursor, open_keyword_matches_fn) {
                let token = Tokenizer::lexer_model_by_char_bind(
                    source_cursor,
                    "keyword",
                    keyword_matches_fn,
                    None,
                )?;
                tokens.push(token);
                continue;
            }

            if Tokenizer::lexer_model_by_char_test(source_cursor, open_numeric_matches_fn) {
                let token = Tokenizer::lexer_model_by_char_bind(
                    source_cursor,
                    "numeric",
                    numeric_matches_fn,
                    None,
                )?;
                tokens.push(token);
                continue;
            }

            if Tokenizer::lexer_model_by_char_test(source_cursor, operation_matches_fn) {
                let token = Tokenizer::lexer_model_by_char_bind(
                    source_cursor,
                    "operation",
                    operation_matches_fn,
                    None,
                )?;
                tokens.push(token);
                continue;
            }

            if Tokenizer::lexer_model_by_char_test(source_cursor, string_matches_fn) {
                source_cursor.next();
                let token = Tokenizer::lexer_model_by_char_bind(
                    source_cursor,
                    "string",
                    not_string_matches_fn,
                    Some(BindOptions::new().set_scape_char(true)),
                )?;
                source_cursor.next();
                tokens.push(token);
                continue;
            }

            return Err(LexerError::SymbolInvalid(source_cursor.pos));
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
    ) -> Result<Token, LexerError> {
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

        Ok(Token {
            kind: kind.to_string(),
            raw: value,
            span: Span {
                start: span_start,
                end: span_end,
            },
        })
    }
}
