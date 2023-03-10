use super::{
    keywords::{get_number_keywords, get_sorted_keywords},
    position::{BytePos, Span, WithSpan},
    token::Token,
};

pub struct Scanner {
    chars: Vec<char>,
    current: usize,
    start: usize,
    current_pos: BytePos,
    start_pos: BytePos,
    line: usize,
    sorted_keywords: Vec<(Vec<char>, Token)>,
}

impl Scanner {
    pub fn new(buf: &str) -> Self {
        Self {
            chars: buf.chars().collect(),
            current: 0,
            start: 0,
            current_pos: BytePos::default(),
            start_pos: BytePos::default(),
            line: 1,
            sorted_keywords: get_sorted_keywords(),
        }
    }

    fn scan_tokens(&mut self) -> Vec<WithSpan<Token>> {
        let mut list = vec![];

        loop {
            let token = self.scan_token();
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

    pub fn scan_token(&mut self) -> WithSpan<Token> {
        self.skip_whitespace();
        self.start = self.current;
        self.start_pos = self.current_pos;

        if self.is_at_end() {
            return self.make_token(Token::Eof);
        }

        self.try_match_keyword()
    }

    fn try_match_keyword(&mut self) -> WithSpan<Token> {
        let ch = self.peek().expect("has char to consume").clone();
        if self.is_numeric(Some(ch)) {
            return self.number();
        }
        match ch {
            '「' => {
                self.advance();
                if self.peek() == Some('「') {
                    return self.string_single_quota();
                }

                return self.identifier();
            }
            '『' => return self.string_double_quota(),
            _ => {
                let keyword = &self
                    .sorted_keywords
                    .iter()
                    .find(|(keyword, _)| self.check_keyword(keyword));

                let data;

                if let Some((chs, token)) = keyword {
                    data = (Some(chs.len()), Some(token.clone()));
                } else {
                    self.advance();
                    return self.error_token(
                        "Unexpected character, maybe you need use 「」 or 『』 to wrap it.",
                    );
                }

                self.step_by(data.0.unwrap());
                return self.make_token(data.1.unwrap());
            }
        }
    }

    pub fn skip_whitespace(&mut self) {
        loop {
            match self.peek() {
                Some(' ') | Some('\r') | Some('\t') | Some('。') | Some('、') | Some('也') => {
                    self.advance();
                }
                Some('\n') => {
                    self.line += 1;
                    self.advance();
                }
                _ => return,
            }
        }
    }

    pub fn error_token(&self, msg: &str) -> WithSpan<Token> {
        self.make_token(Token::Error(String::from(msg)))
    }

    pub fn make_token_in<const N: usize>(
        &self,
        token: Token,
        left: [char; N],
        right: [char; N],
    ) -> WithSpan<Token> {
        let mut start_pos = self.start_pos;
        let mut end_pos = self.current_pos;
        for ch in left {
            start_pos.shift(ch);
        }
        for ch in right {
            end_pos.backwards(ch);
        }

        WithSpan::new(token, Span::from(start_pos, end_pos), self.line)
    }

    pub fn make_token(&self, token: Token) -> WithSpan<Token> {
        WithSpan::new(
            token,
            Span::from(self.start_pos, self.current_pos),
            self.line,
        )
    }

    fn advance(&mut self) {
        let next_c = self.chars.get(self.current);

        if let Some(c) = next_c {
            self.current_pos.shift(*c)
        }
        self.current += 1
    }

    fn step_by(&mut self, step: usize) {
        for _ in 0..step {
            self.advance()
        }
    }

    fn peek(&self) -> Option<char> {
        self.peek_any_num(0)
    }

    fn peek_next(&self) -> Option<char> {
        self.peek_any_num(1)
    }
    fn peek_any_num(&self, index: usize) -> Option<char> {
        match self.chars.get(self.current + index) {
            Some(c) => Some(c.clone()),
            None => None,
        }
    }

    fn consume(&mut self, _ch: char) -> bool {
        match self.peek() {
            Some(_ch) => {
                self.advance();
                true
            }
            None => false,
        }
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

    fn is_numeric(&self, ch: Option<char>) -> bool {
        match ch {
            Some(c) => get_number_keywords().iter().any(|keyword| *keyword == c),
            None => false,
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.chars.len()
    }

    fn string_single_quota(&mut self) -> WithSpan<Token> {
        self.advance();
        self.consume_while(|ch| ch != Some('」'));

        if self.peek() == Some('」') && self.peek_next() == Some('」') {
            self.step_by(2);
            return self.make_token_in(Token::String, ['「', '「'], ['」', '」']);
        }

        return self.error_token("unterminated string.");
    }

    fn string_double_quota(&mut self) -> WithSpan<Token> {
        self.advance();
        self.consume_while(|ch| ch != Some('』'));

        if self.consume('』') {
            return self.make_token_in(Token::String, ['『'], ['』']);
        }

        return self.error_token("unterminated string.");
    }

    fn identifier(&mut self) -> WithSpan<Token> {
        self.consume_while(|ch| ch != Some('」'));

        if self.consume('」') {
            return self.make_token_in(Token::Identifier, ['「'], ['」']);
        }

        return self.error_token("unterminated identifier.");
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
}

#[cfg(test)]
mod test {

    use super::Scanner;

    #[test]
    fn init_scanner() {
        let scanner = Scanner::new("Hello");
        assert_eq!(scanner.chars, vec!['H', 'e', 'l', 'l', 'o']);
    }

    fn generate_tokens_snapshot(str: &str) {
        let mut scanner = Scanner::new(str);
        let tokens = scanner.scan_tokens();

        insta::assert_yaml_snapshot!(str, tokens);
    }

    #[test]
    fn test_normal_scan_tokens() {
        generate_tokens_snapshot("吾有一言");
    }

    #[test]
    fn test_scan_tokens_with_invalid_end() {
        generate_tokens_snapshot("吾有一数");
    }

    #[test]
    fn test_scan_tokens_with_invalid_mid() {
        generate_tokens_snapshot("吾有一数");
    }

    #[test]
    fn test_scan_number_token() {
        generate_tokens_snapshot("一百三十五");
    }

    #[test]
    fn test_scan_float_number_token() {
        generate_tokens_snapshot("一·三五");
    }

    #[test]
    fn test_scan_string_token() {
        generate_tokens_snapshot("「「一·三五」」");
    }

    #[test]
    fn test_scan_double_string_token() {
        generate_tokens_snapshot("『一·三五』");
    }

    #[test]
    fn test_scan_unterminated_string_token1() {
        generate_tokens_snapshot("「「一·三五」");
    }

    #[test]
    fn test_scan_unterminated_string_token2() {
        generate_tokens_snapshot("「「一·三五");
    }

    #[test]
    fn test_scan_identifier() {
        generate_tokens_snapshot("「甲」");
    }

    #[test]
    fn test_scan_unterminated_identifier() {
        generate_tokens_snapshot("「甲");
    }

    #[test]
    fn test_scan_binary() {
        generate_tokens_snapshot("加一以二");
    }

    #[test]
    fn test_log() {
        generate_tokens_snapshot("書之");
    }

    #[test]
    fn test_assign() {
        generate_tokens_snapshot("昔之「甲」者今一是矣")
    }

    #[test]
    fn test_if() {
        generate_tokens_snapshot("若二等於二者加一以五書之云云")
    }

    #[test]
    fn test_define() {
        generate_tokens_snapshot("有數一名之曰「丙」")
    }
}
