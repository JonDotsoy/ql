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
