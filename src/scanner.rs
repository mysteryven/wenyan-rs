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

        loop {
            let token = self.scan_token(&sorted_keywords);
            let is_end = match token.get_value() {
                Token::Eof => true,
                _ => false,
            };
            list.push(token);

            if is_end {
                break;
            }
        }

        return list;
    }

    pub fn scan_token(&mut self, sorted_keywords: &[(Vec<char>, Token)]) -> WithSpan<Token> {
        self.start = self.current;

        if self.is_at_end() {
            return self.make_token(Token::Eof);
        }

        self.try_match_keyword(sorted_keywords)
    }

    fn advance(&mut self) {
        self.current += 1
    }

    fn step_by(&mut self, step: usize) {
        self.current += step
    }

    fn check_keyword(&self, keyword: &[char]) -> bool {
        let len = keyword.len();
        if self.current + len > self.chars.len() {
            return false;
        }

        for idx in 0..len {
            if keyword.get(idx) != self.chars.get(self.current + idx) {
                return false;
            }
        }

        true
    }

    fn try_match_keyword(&mut self, sorted_keywords: &[(Vec<char>, Token)]) -> WithSpan<Token> {
        let keyword = sorted_keywords
            .iter()
            .find(|(keyword, _)| self.check_keyword(keyword));

        if keyword.is_none() {
            self.advance();
            return self.error_token("Unexpected character.");
        }

        let (str, token) = keyword.unwrap();
        let first_ch = str.get(0).expect("keyword at least has one char").clone();
        if self.is_numeric(Some(first_ch)) {
            return self.number();
        }
        match first_ch {
            '「' => {
                self.advance();
                if self.peek() == Some('「') {
                    return self.string();
                }

                return self.identifier();
            }
            _ => {
                self.step_by(str.len());
                self.make_token(token.clone())
            }
        }
    }

    fn peek(&self) -> Option<char> {
        match self.chars.get(self.current) {
            Some(c) => Some(c.clone()),
            None => None,
        }
    }

    fn peek_next(&self) -> Option<char> {
        match self.chars.get(self.current + 1) {
            Some(c) => Some(c.clone()),
            None => None,
        }
    }

    fn string(&mut self) -> WithSpan<Token> {
        self.advance();
        self.consume_while(|ch| ch != Some('」'));

        if self.peek() == Some('」') && self.peek_next() == Some('」') {
            self.step_by(2);
            return self.make_token(Token::String);
        }

        return self.error_token("unterminated string.");
    }

    fn identifier(&mut self) -> WithSpan<Token> {
        self.consume_while(|ch| ch != Some('」'));

        if self.consume('」') {
            return self.make_token(Token::Identifier);
        }

        return self.error_token("unterminated identifier.");
    }

    fn consume(&mut self, ch: char) -> bool {
        match self.peek() {
            Some(ch) => {
                self.advance();
                true
            }
            None => false,
        }
    }

    fn number(&mut self) -> WithSpan<Token> {
        let is_numeric = |ch| get_number_keywords().iter().any(|key| Some(*key) == ch);
        self.consume_while(is_numeric);

        if Some('·') == self.peek() && is_numeric(self.peek_next()) {
            self.advance();
            self.consume_while(is_numeric);
        }

        self.make_token(Token::Number)
    }

    fn consume_while<F>(&mut self, predict: F)
    where
        F: Fn(Option<char>) -> bool,
    {
        while let Some(ch) = self.peek() {
            if predict(Some(ch)) {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn is_numeric(&self, ch: Option<char>) -> bool {
        match ch {
            Some(c) => get_number_keywords().iter().any(|keyword| *keyword == c),
            None => false,
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.chars.len()
    }

    pub fn error_token(&self, msg: &str) -> WithSpan<Token> {
        self.make_token(Token::Error(String::from(msg)))
    }

    pub fn make_token(&self, token: Token) -> WithSpan<Token> {
        WithSpan::new(token, Span::new(self.start, self.current))
    }
}

fn get_sorted_keywords() -> Vec<(Vec<char>, Token)> {
    let mut key_defines = Vec::new();

    key_defines.push(("有", Token::Decl));
    key_defines.push(("吾有", Token::Decl));
    key_defines.push(("數", Token::Type));
    key_defines.push(("言", Token::Type));

    let mut final_keywords: Vec<(Vec<char>, Token)> = key_defines
        .iter()
        .map(|(str, token)| (str.chars().collect::<Vec<char>>(), token.clone()))
        .collect();

    for ch in get_number_keywords() {
        final_keywords.push((vec![ch], Token::Number));
    }

    final_keywords.push((vec!['「'], Token::Empty));
    final_keywords.push((vec!['『'], Token::Empty));

    final_keywords
}

fn get_number_keywords() -> [char; 40] {
    [
        '負', '又', '零', '〇', '一', '二', '三', '四', '五', '六', '七', '八', '九', '十', '百',
        '千', '萬', '億', '兆', '京', '垓', '秭', '穰', '溝', '澗', '正', '載', '極', '分', '釐',
        '毫', '絲', '忽', '微', '纖', '沙', '塵', '埃', '渺', '漠',
    ]
}

pub fn wenyan2token(buf: &str) -> WithSpan<Token> {
    todo!()
}

#[cfg(test)]
mod test {
    use super::{get_sorted_keywords, Scanner};

    #[test]
    fn init_scanner() {
        let scanner = Scanner::new("Hello");
        assert_eq!(scanner.chars, vec!['H', 'e', 'l', 'l', 'o']);
    }

    #[test]
    fn test_ordered_keywords() {
        let keywords = get_sorted_keywords();
        let keys: Vec<Vec<char>> = keywords.into_iter().map(|(key, _)| key).collect();
        println!("{:?}", keys)
    }

    #[test]
    fn test_scan_tokens() {
        let mut scanner = Scanner::new("吾有一言");
        let tokens = scanner.scan_tokens();

        for token in &tokens {
            println!("{:?}", token);
        }
    }

    #[test]
    fn test_scan_tokens_with_invalid_end() {
        let mut scanner = Scanner::new("吾有一数");
        let tokens = scanner.scan_tokens();

        for token in &tokens {
            println!("{:?}", token);
        }
    }

    #[test]
    fn test_scan_tokens_with_invalid_mid() {
        let mut scanner = Scanner::new("有树有");
        let tokens = scanner.scan_tokens();

        for token in &tokens {
            println!("{:?}", token);
        }
    }

    #[test]
    fn test_scan_number_token() {
        let mut scanner = Scanner::new("一百三十五");
        let tokens = scanner.scan_tokens();

        for token in &tokens {
            println!("{:?}", token);
        }
    }

    #[test]
    fn test_scan_float_number_token() {
        let mut scanner = Scanner::new("一·三五");
        let tokens = scanner.scan_tokens();

        for token in &tokens {
            println!("{:?}", token);
        }
    }

    #[test]
    fn test_scan_string_token() {
        let mut scanner = Scanner::new("「「一·三五」」");
        let tokens = scanner.scan_tokens();

        for token in &tokens {
            println!("{:?}", token);
        }
    }

    #[test]
    fn test_scan_unterminated_string_token1() {
        let mut scanner = Scanner::new("「「一·三五」");
        let tokens = scanner.scan_tokens();

        for token in &tokens {
            println!("{:?}", token);
        }
    }

    #[test]
    fn test_scan_unterminated_string_token2() {
        let mut scanner = Scanner::new("「「一·三五");
        let tokens = scanner.scan_tokens();

        for token in &tokens {
            println!("{:?}", token);
        }
    }

    #[test]
    fn test_scan_identifier() {
        let mut scanner = Scanner::new("「甲」");
        let tokens = scanner.scan_tokens();

        for token in &tokens {
            println!("{:?}", token);
        }
    }

    #[test]
    fn test_scan_unterminated_identifier() {
        let mut scanner = Scanner::new("「甲");
        let tokens = scanner.scan_tokens();

        for token in &tokens {
            println!("{:?}", token);
        }
    }
}
