use super::{is_line, is_whitespace, Kind, Token};
use crate::lexical_analysis::lexer::Lexer;

const ACCEPTED_DOT: usize = 5;

pub fn recognize(buf: &'static [u8]) -> Vec<Token<'static>> {
    buf.iter()
        .fold(
            {
                let lexer = Lexer::new();
                lexer.reset_pattern_dot(1)
            },
            |lexer, c| {
                let new_lexer;
                if is_whitespace(c) && lexer.pattern_dot == 0 {
                    new_lexer = lexer.update_dots(1);
                } else if *c == 'l' as u8 {
                    new_lexer = lexer.update_dots(2);
                } else if *c == 'e' as u8 && lexer.pattern_dot == 2 {
                    new_lexer = lexer.update_dots(3);
                } else if *c == 't' as u8 && lexer.pattern_dot == 3 {
                    new_lexer = lexer.update_dots(4);
                } else if is_whitespace(c) && lexer.pattern_dot == 4 {
                    new_lexer = lexer.update_dots(5);
                } else {
                    new_lexer = lexer.update_dots(0);
                }

                let accepted_lexer = if new_lexer.pattern_dot == ACCEPTED_DOT {
                    let accepted_lexer = new_lexer.recognized(Kind::Let, "let".as_bytes());
                    accepted_lexer.reset_pattern_dot(1)
                } else {
                    new_lexer
                };

                if is_line(c) {
                    accepted_lexer.update_counts_by_line()
                } else {
                    accepted_lexer
                }
            },
        )
        .finish(Kind::Let, ACCEPTED_DOT, buf)
        .tokens
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexical_analysis::token::Position;
    #[test]
    fn test_recognize() {
        let s = "let let let let\n let";
        assert_eq!(
            recognize(s.as_bytes()),
            [
                Token {
                    kind: Kind::Let,
                    repr: &[108, 101, 116,],
                    pos: Position {
                        line_count: 0,
                        char_count: 0,
                    },
                },
                Token {
                    kind: Kind::Let,
                    repr: &[108, 101, 116,],
                    pos: Position {
                        line_count: 0,
                        char_count: 4,
                    },
                },
                Token {
                    kind: Kind::Let,
                    repr: &[108, 101, 116,],
                    pos: Position {
                        line_count: 0,
                        char_count: 8,
                    },
                },
                Token {
                    kind: Kind::Let,
                    repr: &[108, 101, 116,],
                    pos: Position {
                        line_count: 0,
                        char_count: 12,
                    },
                },
                Token {
                    kind: Kind::Let,
                    repr: &[108, 101, 116,],
                    pos: Position {
                        line_count: 1,
                        char_count: 0,
                    },
                },
            ]
        );
    }
}
