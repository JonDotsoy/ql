pub(crate) struct BindOptions {
    /// If detect the char [`\`] indicate the next char is literal.
    pub(crate) scape_char: bool,
}

impl BindOptions {
    /// Creates a new [`BindOptions`].
    pub(crate) fn new() -> Self {
        Self { scape_char: false }
    }

    pub(crate) fn set_scape_char(&mut self, scape_char: bool) -> &mut Self {
        self.scape_char = scape_char;
        self
    }
}

impl Default for BindOptions {
    fn default() -> Self {
        Self::new()
    }
}
