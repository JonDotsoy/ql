pub struct LexerOptions {
    pub break_by_close_square_bracket: bool,
    pub break_by_close_parenthesis: bool,
    pub break_by_close_curly_bracket: bool,
    pub template_break_by_close_curly_bracket: bool,
}

impl LexerOptions {
    pub fn set_break_by_close_square_bracket(
        &mut self,
        break_by_close_square_bracket: bool,
    ) -> &mut Self {
        self.break_by_close_square_bracket = break_by_close_square_bracket;
        self
    }
    pub fn set_break_by_close_parenthesis(
        &mut self,
        break_by_close_parenthesis: bool,
    ) -> &mut Self {
        self.break_by_close_parenthesis = break_by_close_parenthesis;
        self
    }
    pub fn set_break_by_close_curly_bracket(
        &mut self,
        break_by_close_curly_bracket: bool,
    ) -> &mut Self {
        self.break_by_close_curly_bracket = break_by_close_curly_bracket;
        self
    }
    pub fn set_template_break_by_close_curly_bracket(
        &mut self,
        template_break_by_close_curly_bracket: bool,
    ) -> &mut Self {
        self.template_break_by_close_curly_bracket = template_break_by_close_curly_bracket;
        self
    }
}

impl Default for LexerOptions {
    fn default() -> Self {
        Self {
            break_by_close_square_bracket: false,
            break_by_close_parenthesis: false,
            break_by_close_curly_bracket: false,
            template_break_by_close_curly_bracket: false,
        }
    }
}
