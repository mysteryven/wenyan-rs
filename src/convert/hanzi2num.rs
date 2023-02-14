// copy from https://github.com/wenyan-lang/wenyan/blob/master/src/converts/hanzi2num.ts
// and rewrite it to Rust version.

use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq)]
enum TT {
    SIGN,      // 負
    DIGIT,     // 一二三...
    DECIMAL,   // ·
    INT_MULT,  // 十百千萬億...
    FRAC_MULT, // 分釐毫...
    DELIM,     // 又
    ZERO,      // 零

    // pseudo tokens
    BEGIN, // <BEGIN>
    END,   // <END>
}

type Digit = char;

#[derive(Clone, Copy)]
struct NumberToken {
    pub kind: TT,
    pub digit: Option<Digit>,
    pub sign: Option<i8>,
    pub expr: Option<Exp>,
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
    pub fn kind(&self) -> &TT {
        &self.kind
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
    map.insert('十', NumberToken::new(TT::INT_MULT, None, None, Some(1)));
    map.insert('百', NumberToken::new(TT::INT_MULT, None, None, Some(2)));
    map.insert('千', NumberToken::new(TT::INT_MULT, None, None, Some(3)));
    map.insert('萬', NumberToken::new(TT::INT_MULT, None, None, Some(4)));
    map.insert('億', NumberToken::new(TT::INT_MULT, None, None, Some(8)));
    map.insert('兆', NumberToken::new(TT::INT_MULT, None, None, Some(12)));
    map.insert('京', NumberToken::new(TT::INT_MULT, None, None, Some(16)));
    map.insert('垓', NumberToken::new(TT::INT_MULT, None, None, Some(20)));
    map.insert('秭', NumberToken::new(TT::INT_MULT, None, None, Some(24)));
    map.insert('穰', NumberToken::new(TT::INT_MULT, None, None, Some(28)));
    map.insert('溝', NumberToken::new(TT::INT_MULT, None, None, Some(32)));
    map.insert('澗', NumberToken::new(TT::INT_MULT, None, None, Some(36)));
    map.insert('正', NumberToken::new(TT::INT_MULT, None, None, Some(40)));
    map.insert('載', NumberToken::new(TT::INT_MULT, None, None, Some(44)));
    map.insert('極', NumberToken::new(TT::INT_MULT, None, None, Some(48)));
    map.insert('分', NumberToken::new(TT::FRAC_MULT, None, None, Some(-1)));
    map.insert('釐', NumberToken::new(TT::FRAC_MULT, None, None, Some(-2)));
    map.insert('毫', NumberToken::new(TT::FRAC_MULT, None, None, Some(-3)));
    map.insert('絲', NumberToken::new(TT::FRAC_MULT, None, None, Some(-4)));
    map.insert('忽', NumberToken::new(TT::FRAC_MULT, None, None, Some(-5)));
    map.insert('微', NumberToken::new(TT::FRAC_MULT, None, None, Some(-6)));
    map.insert('纖', NumberToken::new(TT::FRAC_MULT, None, None, Some(-7)));
    map.insert('沙', NumberToken::new(TT::FRAC_MULT, None, None, Some(-8)));
    map.insert('塵', NumberToken::new(TT::FRAC_MULT, None, None, Some(-9)));
    map.insert('埃', NumberToken::new(TT::FRAC_MULT, None, None, Some(-10)));
    map.insert('渺', NumberToken::new(TT::FRAC_MULT, None, None, Some(-11)));
    map.insert('漠', NumberToken::new(TT::FRAC_MULT, None, None, Some(-12)));
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
    NONE,            // <END>, ·
    MULT,            // 微
    MULT_AMBIG,      // 十 (ambiguous: ...十 or 一十)
    DIGIT,           // 一
    DIGIT_WITH_ZERO, // 一...零, 零零， 零一...零,
    DELIM,           // 又
    ZERO,            // 零<END>, 零·, 零又, 零微, 零一
    Sign,            // 負

    ZERO_MULT_AMBIG, // 零十 (ambiguous: 零一十 or 零十 or 〇十)
}

#[derive(PartialEq)]
enum EMultState {
    NONE, // <END>, 一 (ambiguous: 一萬一 or 一十一 or 一·一 or 一絲一)
    FRAC, // ...微
    INT,  // ...萬, ...·,
    DONE, // 負一,
    SIGN,
}

struct MutlStack {
    exps: Vec<Exp>,
    exp_add: Exp,
}

type Exp = isize;

impl MutlStack {
    pub fn new() -> Self {
        Self {
            exps: vec![],
            exp_add: 0,
        }
    }
    pub fn is_empty(&self) -> bool {
        self.exps.len() == 0
    }
    pub fn total(&self) -> Exp {
        self.exp_add
    }
    pub fn top(&self) -> Exp {
        self.exps[self.exps.len() - 1]
    }
    pub fn state(&self) -> EMultState {
        if self.is_empty() {
            return EMultState::NONE;
        } else if self.exps[0] < 0 {
            return EMultState::FRAC;
        } else if self.exps[0] < isize::MAX {
            return EMultState::INT;
        } else {
            return EMultState::DONE;
        }
    }

    pub fn push(&mut self, exp: Exp) {
        self.exp_add += exp;
        self.exps.push(exp);
    }
    pub fn pop(&mut self) {
        self.exp_add -= self.top();
        self.exps.pop();
    }
    pub fn clear(&mut self) {
        self.exp_add = 0;
        self.exps.clear();
    }
    pub fn mark_done(&mut self) {
        self.clear();
        self.push(isize::MAX);
    }
}

struct ParserResult {
    sign: i8, // +1/-1
    exp: Exp, // one plus exponent of the highest digit
    digits: Vec<Digit>,
}

impl ParserResult {
    pub fn new() -> Self {
        Self {
            sign: 1,
            exp: 1,
            digits: vec![],
        }
    }
    pub fn sign(&self) -> i8 {
        self.sign
    }
    pub fn exp(&self) -> Exp {
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
    pub fn push_char(&mut self, digit: Digit) {
        self.digits.push(digit);
        self.exp += 1;
    }
    pub fn fill_zeros(&mut self, new_exp: Exp) {
        let arr = vec![];
        for _ in 0..(new_exp - self.exp) {
            arr.push('0');
        }
        self.push(arr)
    }
    pub fn reset_exp(&mut self, new_exp: Exp) {
        self.exp = new_exp;
    }
}

pub fn parser(tokens: Vec<NumberToken>) -> Option<ParserResult> {
    let mut digit_state = EDigitState::NONE;

    let mut mult_stack = MutlStack::new();
    let mut result = ParserResult::new();

    for token in tokens.iter().rev() {
        if mult_stack.state() == EMultState::SIGN && token.kind() == &TT::BEGIN {
            return None;
        }

        let exp = match token.expr {
            Some(value) => value,
            None => 1,
        };

        match digit_state {
            EDigitState::MULT_AMBIG => {
                match token.kind {
                    // <BEGIN>(一?)十 -> <BEGIN>一十
                    // 負(一?)十 -> 負一十
                    // 又(一?)十 -> 又一十
                    // ·(一?)十 -> ·一十
                    // 分(一?)十絲 -> 分一十絲
                    TT::BEGIN | TT::SIGN | TT::DELIM | TT::DECIMAL | TT::FRAC_MULT => {
                        result.push_char('1');
                        digit_state = EDigitState::DIGIT;
                    }

                    // 一(一?)十 -> 一十
                    TT::DIGIT => {
                        digit_state = EDigitState::MULT;
                    }

                    // 百(一?)十 -> 百一十
                    // 千(一?)十 -> 千一十
                    // 百(一?)萬 -> 百萬
                    TT::INT_MULT => {
                        if mult_stack.top() < exp {
                            result.push_char('1');
                            digit_state = EDigitState::DIGIT;
                        } else {
                            digit_state = EDigitState::MULT;
                        }
                    }

                    // 零(一?)十 -> 零(一?)十
                    TT::ZERO => {
                        digit_state = EDigitState::MULT_AMBIG;
                    }
                    _ => {}
                }
            }
            EDigitState::ZERO_MULT_AMBIG => {
                match token.kind {
                    // <BEGIN>(零一|零|〇)十 -> <BEGIN>〇十
                    // 負(零一|零|〇)十 -> 負〇十
                    // 一(零一|零|〇)十 -> 一〇十
                    // 又(零一|零|〇)十 -> 又〇十
                    // 零(零一|零|〇)十 -> 〇〇十
                    TT::BEGIN | TT::SIGN | TT::DIGIT | TT::DELIM | TT::ZERO => {
                        result.push_char('0');
                        digit_state = EDigitState::DIGIT_WITH_ZERO;
                    }

                    // ·(零一|零|〇)十絲 -> ·零一十絲
                    // ·(零一|零|〇)十釐 -> ·〇十釐
                    // ·(零一|零|〇)十分 -> error
                    // ·(零一|零|〇)百分 -> error
                    // 分(零一|零|〇)十絲 -> 分零一十絲
                    // 分(零一|零|〇)十毫 -> 分〇十絲
                    // 分(零一|零|〇)十釐 -> error
                    // 分(零一|零|〇)十分 -> error
                    TT::DECIMAL | TT::FRAC_MULT => {
                        if mult_stack.total() + 1 < exp {
                            result.push_char('1');
                            result.push_char('0');
                            digit_state = EDigitState::ZERO;
                        } else if mult_stack.total() + 1 == exp {
                            result.push_char('0');
                            digit_state = EDigitState::DIGIT_WITH_ZERO;
                        } else {
                            return None;
                        }
                    }
                    // 千(零一|零|〇)十 -> 千零一十
                    // 百(零一|零|〇)十 -> 百〇十
                    // 萬(零一|零|〇)萬 -> 萬零萬
                    // 百(零一|零|〇)萬 -> 百零萬
                    TT::INT_MULT => {
                        if token.expr.is_none() {
                            return None;
                        }

                        if mult_stack.top() + 1 < exp {
                            result.push_char('1');
                            result.push_char('0');
                            digit_state = EDigitState::ZERO;
                        } else if mult_stack.top() + 1 == exp {
                            result.push_char('0');
                            digit_state = EDigitState::DIGIT_WITH_ZERO;
                        } else {
                            result.push_char('0');
                            digit_state = EDigitState::ZERO;
                        }
                    }
                    _ => {}
                }
                break;
            }
            _ => {}
        }

        // determine the exponent of tail digits
      if (mult_stack.state() === EMultState.NONE) {
        switch (token.type) {
          case TT::INT_MULT:
            // exponent is correct
            break;

          case TT::DECIMAL:
          case TT::FRAC_MULT:
            if (token.exp != null) {
              result.resetExp(token.exp);
            }
            break;

          default:
            break;
        }
      }

        // determine the current exponent and update exponent stack
        let curr_exp = match token.kind {
            TT::BEGIN | TT::SIGN => {
                match digit_state {
                    // <BEGIN>微 -> error
                    // 負微 -> error
                    EDigitState::MULT => None,
                    _ => {
                        mult_stack.mark_done();
                        Some(mult_stack.total())
                    }
                }
            }

            TT::DIGIT | TT::ZERO => {
                match digit_state {
                    // 一又 -> 一·
                    // 零又 -> 零·
                    EDigitState::DELIM => {
                        mult_stack.clear();
                        mult_stack.push(0);
                        Some(mult_stack.total())
                    }

                    _ => Some(result.exp()),
                }
            }

            TT::DELIM => {
                match digit_state {
                    // 又又 -> error
                    EDigitState::DELIM => None,
                    _ => Some(result.exp()),
                }
            }

            // ·...絲 -> ·
            // 釐...絲 -> 釐
            // ·絲 -> error
            // 釐絲 -> error
            TT::DECIMAL | TT::FRAC_MULT => {
                if digit_state == EDigitState::MULT {
                    return None;
                } else {
                    multStack.clear();
                    multStack.push(token.exp);
                    return multStack.total();
                }
            }

            TT::INT_MULT => {
                match digit_state {
                    // 百又...絲 -> 百
                    // 萬又...百萬億 -> 萬萬億
                    EDigitState::DELIM => {
                        if mult_stack.state() == EMultState::FRAC {
                            mult_stack.clear();
                            mult_stack.push(exp);
                        } else {
                            while !mult_stack.is_empty() && mult_stack.top() < exp {
                                mult_stack.pop();
                            }

                            mult_stack.push(exp);
                        }
                    }

                    // 萬...百萬億 -> 萬萬億
                    // 萬零...百萬億 -> 萬萬億
                    // 百...十絲 -> 百絲
                    // 千零...十絲 -> 千絲
                    EDigitState::NONE
                    | EDigitState::MULT
                    | EDigitState::MULT_AMBIG
                    | EDigitState::DIGIT
                    | EDigitState::DIGIT_WITH_ZERO
                    | EDigitState::ZERO
                    | EDigitState::ZERO_MULT_AMBIG => {
                        while !mult_stack.is_empty()
                            && mult_stack.top() < exp
                            && mult_stack.top() >= 0
                        {
                            mult_stack.pop();
                        }

                        mult_stack.push(exp);
                    }
                    _ => {}
                }
                return mult_stack.total();
            }
            _ => None,
        };

        if curr_exp.is_none() {
            return None;
        }
    }
    None
}

pub fn hanzi2num(s: &str) -> Option<f64> {
    None
}

#[cfg(test)]
mod test {}
