struct PatternMatcher<'a> {
    pattern: &'static str,
    whitespaces: &'a [&'static str],
}
use std::collections::LinkedList;

#[derive(Debug)]
struct Token;

impl<'a> PatternMatcher<'a> {
    fn new(pattern: &'static str, whitespaces: &'a [&'static str]) -> Self {
        Self {
            pattern,
            whitespaces,
        }
    }

    fn run(&self, text: impl AsRef<str>) -> LinkedList<Token> {
        text.as_ref()
            .chars()
            .into_iter()
            .fold(LinkedList::new(), |tokens, c| tokens)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let pattern = "ptn";
        let lines = ["\n"];
        let whitespaces = [" ", "\n", "\r", "\t"];
        let text = "ptn pta pte\n pt ptn\n";

        let pattern_matcher = PatternMatcher::new(pattern, &whitespaces);
        let tokens = pattern_matcher.run(text);
        println!("{:#?}", tokens);
        assert!(false);
    }
}
