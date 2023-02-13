// copy from https://github.com/wenyan-lang/wenyan/blob/master/src/converts/hanzi2num.ts
// and rewrite it to Rust version.

use std::collections::HashMap;

#[derive(Clone, Copy)]
enum TT {
    SIGN,     // 負
    DIGIT,    // 一二三...
    DECIMAL,  // ·
    IntMult,  // 十百千萬億...
    FracMult, // 分釐毫...
    DELIM,    // 又
    ZERO,     // 零

    // pseudo tokens
    BEGIN, // <BEGIN>
    END,   // <END>
}

type Digit = char;

#[derive(Clone, Copy)]
struct NumberToken {
    kind: TT,
    digit: Option<Digit>,
    sign: Option<i8>,
    expr: Option<i8>,
}

impl NumberToken {
    pub fn new(kind: TT, digit: Option<Digit>, sign: Option<i8>, expr: Option<i8>) -> Self {
        Self {
            kind,
            digit,
            sign,
            expr,
        }
    }
}

fn get_num_tokens() -> HashMap<char, NumberToken> {
    let map = HashMap::new();
    map.insert('負', NumberToken::new(TT::SIGN, None, Some(-1), None));
    map.insert('·', NumberToken::new(TT::DECIMAL, None, None, Some(0)));
    map.insert('又', NumberToken::new(TT::DELIM, None, None, None));
    map.insert('有', NumberToken::new(TT::DELIM, None, None, None));
    map.insert('零', NumberToken::new(TT::DIGIT, Some('0'), None, None));
    map.insert('一', NumberToken::new(TT::DIGIT, Some('1'), None, None));
    map.insert('二', NumberToken::new(TT::DIGIT, Some('2'), None, None));
    map.insert('三', NumberToken::new(TT::DIGIT, Some('3'), None, None));
    map.insert('四', NumberToken::new(TT::DIGIT, Some('4'), None, None));
    map.insert('五', NumberToken::new(TT::DIGIT, Some('5'), None, None));
    map.insert('六', NumberToken::new(TT::DIGIT, Some('6'), None, None));
    map.insert('七', NumberToken::new(TT::DIGIT, Some('7'), None, None));
    map.insert('八', NumberToken::new(TT::DIGIT, Some('8'), None, None));
    map.insert('九', NumberToken::new(TT::DIGIT, Some('9'), None, None));
    map.insert('十', NumberToken::new(TT::IntMult, None, None, Some(1)));
    map.insert('百', NumberToken::new(TT::IntMult, None, None, Some(2)));
    map.insert('千', NumberToken::new(TT::IntMult, None, None, Some(3)));
    map.insert('萬', NumberToken::new(TT::IntMult, None, None, Some(4)));
    map.insert('億', NumberToken::new(TT::IntMult, None, None, Some(8)));
    map.insert('兆', NumberToken::new(TT::IntMult, None, None, Some(12)));
    map.insert('京', NumberToken::new(TT::IntMult, None, None, Some(16)));
    map.insert('垓', NumberToken::new(TT::IntMult, None, None, Some(20)));
    map.insert('秭', NumberToken::new(TT::IntMult, None, None, Some(24)));
    map.insert('穰', NumberToken::new(TT::IntMult, None, None, Some(28)));
    map.insert('溝', NumberToken::new(TT::IntMult, None, None, Some(32)));
    map.insert('澗', NumberToken::new(TT::IntMult, None, None, Some(36)));
    map.insert('正', NumberToken::new(TT::IntMult, None, None, Some(40)));
    map.insert('載', NumberToken::new(TT::IntMult, None, None, Some(44)));
    map.insert('極', NumberToken::new(TT::IntMult, None, None, Some(48)));
    map.insert('分', NumberToken::new(TT::FracMult, None, None, Some(-1)));
    map.insert('釐', NumberToken::new(TT::FracMult, None, None, Some(-2)));
    map.insert('毫', NumberToken::new(TT::FracMult, None, None, Some(-3)));
    map.insert('絲', NumberToken::new(TT::FracMult, None, None, Some(-4)));
    map.insert('忽', NumberToken::new(TT::FracMult, None, None, Some(-5)));
    map.insert('微', NumberToken::new(TT::FracMult, None, None, Some(-6)));
    map.insert('纖', NumberToken::new(TT::FracMult, None, None, Some(-7)));
    map.insert('沙', NumberToken::new(TT::FracMult, None, None, Some(-8)));
    map.insert('塵', NumberToken::new(TT::FracMult, None, None, Some(-9)));
    map.insert('埃', NumberToken::new(TT::FracMult, None, None, Some(-10)));
    map.insert('渺', NumberToken::new(TT::FracMult, None, None, Some(-11)));
    map.insert('漠', NumberToken::new(TT::FracMult, None, None, Some(-12)));
    map.insert('〇', NumberToken::new(TT::DIGIT, Some('0'), None, None));

    map
}

const NEG_WORD: &str = "負";
const INF_WORD: &str = "無限大數";
const NAN_WORD: &str = "不可算數";

fn tokenize(s: &str) -> Vec<NumberToken> {
    let num_tokens_map = get_num_tokens();
    let mut ret = vec![];
    let iterator = s.chars().into_iter();
    for ch in iterator {
        match num_tokens_map.get(&ch) {
            Some(value) => ret.push(value.clone()),
            None => {}
        }
    }

    ret.push(NumberToken {
        kind: TT::END,
        digit: None,
        sign: None,
        expr: None,
    });

    return ret;
}

enum EDigitState {
    None,          // <END>, ·
    Mult,          // 微
    MultAmbig,     // 十 (ambiguous: ...十 or 一十)
    DIGIT,         // 一
    DigitWithZero, // 一...零, 零零， 零一...零,
    Delim,         // 又
    Zero,          // 零<END>, 零·, 零又, 零微, 零一
    Sign,          // 負
    ZeroMultAmbig, // 零十 (ambiguous: 零一十 or 零十 or 〇十)
}

enum EMultState {
    NONE, // <END>, 一 (ambiguous: 一萬一 or 一十一 or 一·一 or 一絲一)
    FRAC, // ...微
    INT,  // ...萬, ...·,
    DONE, // 負一,
    SIGN,
}

struct MutlStack {
    exps: Vec<f64>,
    exp_add: f64,
}

impl MutlStack {
    pub fn new() -> Self {
        Self {
            exps: vec![],
            exp_add: 0.0,
        }
    }
    pub fn is_empty(&self) -> bool {
        self.exps.len() == 0
    }
    pub fn total(&self) -> f64 {
        self.exp_add
    }
    pub fn top(&self) -> f64 {
        self.exps[self.exps.len() - 1]
    }
    pub fn state(&self) -> EMultState {
        if self.is_empty() {
            return EMultState::NONE;
        } else if self.exps[0] < 0.0 {
            return EMultState::FRAC;
        } else if self.exps[0] < f64::MAX {
            return EMultState::INT;
        } else {
            return EMultState::DONE;
        }
    }

    pub fn push(&mut self, exp: f64) {
        self.exp_add += exp;
        self.exps.push(exp);
    }
    pub fn pop(&mut self) {
        self.exp_add -= self.top();
        self.exps.pop();
    }
    pub fn clear(&mut self) {
        self.exp_add = 0.0;
        self.exps.clear();
    }
    pub fn mark_done(&mut self) {
        self.clear();
        self.push(f64::MAX);
    }
}

pub fn parser() {
    let digit_state = EDigitState::None;

    let mult_stack = MutlStack::new();
}

struct ParserResult {
    sign: i8, // +1/-1
    exp: i8,  // one plus exponent of the highest digit
    digits: Vec<Digit>,
}

impl ParserResult {
    pub fn sign(&self) -> i8 {
        self.sign
    }
    pub fn exp(&self) -> i8 {
        self.exp
    }
    pub fn digits(&self) -> Vec<Digit> {
        self.digits
    }

    pub fn apply_sign(&mut self, new_sign: i8) {
        self.sign *= new_sign;
    }
    // digit: number or array of numbers
    pub fn push(&mut self, digits: Vec<Digit>) {
        for digit in digits {
            self.digits.push(digit);
            self.exp += 1;
        }
    }
    pub fn fill_zeros(&self, new_exp: i8) {
        let arr = vec![];
        for _ in 0..(new_exp - self.exp) {
            arr.push('0');
        }
        self.push(arr)
    }
    pub fn reset_exp(&self, new_exp: i8) {
        self.exp = new_exp;
    }
}

pub fn hanzi2num(s: &str) -> f64 {}

#[cfg(test)]
mod test {}
