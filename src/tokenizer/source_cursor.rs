use super::to_direction::ToDirection;
use super::token::Span;
use super::token::Token;

pub struct SourceCursor {
    pub(crate) source: String,
    pub(crate) pos: usize,
}

impl SourceCursor {
    pub fn new<A: ToString>(payload: A) -> Self {
        Self {
            pos: 0,
            source: payload.to_string(),
        }
    }

    pub fn get<A>(&self, position_from: A, position_at: A) -> &str
    where
        A: ToDirection,
    {
        let from = position_from.resolve(self.pos);
        let at = position_at.resolve(self.pos);
        let len = self.source.len();
        if at < from {
            return "";
        }
        let real_from = if from < len { from } else { len };
        let real_at = if at < len { at } else { len };

        unsafe { self.source.get_unchecked(real_from..real_at) }
    }

    pub fn get_old(&self, from: usize, at: usize) -> &str {
        let len = self.source.len();
        if at < from {
            return "";
        }
        let real_from = if from < len { from } else { len };
        let real_at = if at < len { at } else { len };

        unsafe { self.source.get_unchecked(real_from..real_at) }
    }

    pub fn _get_current_plus_at(&self, plus: usize) -> &str {
        self.get_old(self.pos, self.pos + plus)
    }

    pub fn get_at_char(&self, option_from: Option<usize>) -> char {
        let from = if let Some(from) = option_from {
            from
        } else {
            self.pos
        };
        if let Some(c) = self.source.chars().nth(from) {
            c
        } else {
            '\0'
        }
    }

    pub fn get_current_char(&self) -> char {
        self.get_at_char(Some(self.pos))
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

    // pub fn next_steps(&mut self, steps: usize) -> Option<(usize, char)> {
    //     let mut last_current = self.current();
    //     for _i in 0..steps {
    //         last_current = self.next();
    //     }
    //     last_current
    // }

    pub fn _prev(&mut self) -> Option<(usize, char)> {
        let current = self.current();
        self.pos = self.pos - 1;
        current
    }

    pub fn _next_string(&self, chunk_len: usize) -> String {
        self.source.chars().skip(self.pos).take(chunk_len).collect()
    }

    pub fn forward<A: ToDirection>(&mut self, direction: A) {
        self.pos = direction.resolve(self.pos);
    }

    pub fn create_token<A, B>(
        &mut self,
        token_kind: &str,
        direction_from: A,
        direction_at: B,
    ) -> Token
    where
        A: ToDirection,
        B: ToDirection,
    {
        let from = direction_from.resolve(self.pos);
        let at = direction_at.resolve(self.pos);
        self.pos = at;
        Token {
            kind: token_kind.to_string(),
            raw: self.get_old(from, at).to_string(),
            span: Span {
                start: from,
                end: at,
            },
        }
    }

    // pub fn _create_token_at(&mut self, token_kind: &str, steps: usize) -> Token {
    //     let from = self.pos;
    //     let at = from + steps;
    //     self.next_steps(steps);
    //     Token {
    //         kind: token_kind.to_string(),
    //         raw: self.get_old(from, at).to_string(),
    //         span: Span {
    //             start: from,
    //             end: at,
    //         },
    //     }
    // }
}
