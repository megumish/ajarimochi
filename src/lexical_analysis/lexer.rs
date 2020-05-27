use crate::lexical_analysis::token::{Kind, Position, Token};

#[derive(Debug)]
pub struct Lexer {
    pub pattern_dot: usize,
    code_dot: usize,
    pattern_line_count: usize,
    line_count: usize,
    char_count: usize,
    pub tokens: Vec<Token<'static>>,
}

impl Lexer {
    pub fn new() -> Self {
        Self {
            pattern_dot: 0,
            code_dot: 0,
            pattern_line_count: 0,
            char_count: 0,
            line_count: 0,
            tokens: Vec::new(),
        }
    }

    pub fn recognized(self, kind: Kind, repr: &'static [u8]) -> Self {
        let new_token = Token {
            kind: kind,
            repr: repr,
            pos: Position {
                line_count: self.line_count + self.pattern_line_count,
                char_count: self.char_count,
            },
        };
        let mut tokens = self.tokens;
        tokens.push(new_token);
        Self {
            pattern_dot: 0,
            code_dot: self.code_dot,
            pattern_line_count: 0,
            line_count: self.line_count + self.pattern_line_count,
            char_count: self.char_count + repr.len() + 1,
            tokens: tokens,
        }
    }

    pub fn finish(self, kind: Kind, accepted_dot: usize, buffer: &'static [u8]) -> Self {
        if self.pattern_dot + 1 == accepted_dot {
            let repr = &buffer[self.code_dot - self.pattern_dot + 1..self.code_dot];
            return self.recognized(kind, repr);
        }
        return self;
    }

    pub fn update_dots(self, new_pattern_dot: usize) -> Self {
        Self {
            pattern_dot: new_pattern_dot,
            code_dot: self.code_dot + 1,
            pattern_line_count: self.pattern_line_count,
            line_count: self.line_count,
            char_count: self.char_count,
            tokens: self.tokens,
        }
    }

    pub fn reset_pattern_dot(self, new_pattern_dot: usize) -> Self {
        Self {
            pattern_dot: new_pattern_dot,
            code_dot: self.code_dot,
            pattern_line_count: self.pattern_line_count,
            line_count: self.line_count,
            char_count: self.char_count,
            tokens: self.tokens,
        }
    }

    pub fn update_counts_by_line(self) -> Self {
        Self {
            pattern_dot: self.pattern_dot,
            code_dot: self.code_dot,
            pattern_line_count: self.pattern_line_count + 1,
            line_count: self.line_count,
            char_count: 0,
            tokens: self.tokens,
        }
    }
}
