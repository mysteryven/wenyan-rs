use std::{
    collections::HashMap,
    iter,
    slice::{Iter, IterMut},
    vec::IntoIter,
};

use crate::{
    position::{Span, WithSpan},
    token::Token,
};

struct Scanner {
    chars: Vec<char>,
    current: usize,
    start: usize,
}

impl Scanner {
    pub fn new(buf: &str) -> Self {
        Self {
            chars: buf.chars().collect(),
            current: 0,
            start: 0,
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<WithSpan<Token>> {
        let sorted_keywords = get_sorted_keywords();
        let mut list = vec![];

        let token = self.scan_token(&sorted_keywords);
        list.push(token);

        list
    }

    pub fn scan_token(&mut self, sorted_keywords: &[(String, Token)]) -> WithSpan<Token> {
        self.start = self.current;

        if self.is_at_end() {
            return self.make_token();
        }

        if let Some(token) = self.check_keyword(sorted_keywords) {
            token
        } else {
            self.error_token("unrecoginate")
        }
    }

    fn slice_between(&mut self, from: usize, to: usize) -> &str {
        unimplemented!()
    }

    fn check_keyword(&mut self, sorted_keywords: &[(String, Token)]) -> Option<WithSpan<Token>> {
        let keyword = sorted_keywords
            .iter()
            .find(|(keyword, _)| keyword == self.slice_between(0, keyword.len()));

        match keyword {
            Some((_, token)) => Some(WithSpan {
                value: token.clone(),
                span: Span::new(self.start, self.current),
            }),
            None => None,
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.chars.len()
    }

    pub fn error_token(&self, msg: &str) -> WithSpan<Token> {
        todo!()
    }

    pub fn make_token(&mut self) -> WithSpan<Token> {
        todo!()
    }
}

fn get_sorted_keywords() -> Vec<(String, Token)> {
    let mut list = Vec::new();

    list.push(("有".to_owned(), Token::Decl));
    list.push(("吾有".to_owned(), Token::Decl));
    list.push(("數".to_owned(), Token::Type));
    list.push(("言".to_owned(), Token::Type));

    list.sort_by(|a, b| {
        let a_len = a.0.len();
        let b_len = b.0.len();

        b_len.cmp(&a_len)
    });

    list
}

pub fn wenyan2token(buf: &str) -> WithSpan<Token> {
    todo!()
}

#[cfg(test)]
mod test {
    use std::cmp::Ordering;

    use super::{get_sorted_keywords, Scanner};

    #[test]
    fn init_scanner() {
        let scanner = Scanner::new("Hello");
        assert_eq!(scanner.chars, vec!['H', 'e', 'l', 'l', 'o']);
    }

    #[test]
    fn test_ordered_keywords() {
        let keywords = get_sorted_keywords();
        let keys: Vec<String> = keywords.into_iter().map(|(key, _)| key).collect();
        println!("{:?}", keys)
    }
}
