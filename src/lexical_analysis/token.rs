#[derive(Debug, PartialEq)]
pub struct Token<'a> {
    pub kind: Kind,
    pub repr: &'a [u8],
    pub pos: Position,
}

#[derive(Debug, PartialEq)]
pub struct Position {
    pub line_count: usize,
    pub char_count: usize,
}

#[derive(Debug, PartialEq)]
pub enum Kind {
    Let,
    Unknown,
}

mod token_let;

pub(in crate::lexical_analysis) fn is_whitespace(c: &u8) -> bool {
    *c == ' ' as u8 || *c == '\t' as u8 || *c == '\n' as u8
}

pub(in crate::lexical_analysis) fn is_line(c: &u8) -> bool {
    *c == '\n' as u8
}
