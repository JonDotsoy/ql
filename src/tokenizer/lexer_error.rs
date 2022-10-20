#[derive(Debug)]
pub enum LexerError {
    SymbolInvalid(Option<(usize, char)>),
}
