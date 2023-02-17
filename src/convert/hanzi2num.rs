// copy from https://github.com/wenyan-lang/wenyan/blob/master/src/converts/hanzi2num.ts
// and rewrite it to Rust version.
// Note: I finally not use it.
use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq)]
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
    pub kind: TT,
    pub digit: Option<Digit>,
    pub sign: Option<i8>,
    pub expr: Option<Exp>,
}

impl NumberToken {
    pub fn new(kind: TT, digit: Option<Digit>, sign: Option<i8>, expr: Option<Exp>) -> Self {
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
    let mut map = HashMap::new();
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

fn tokenize(s: &str) -> Option<Vec<NumberToken>> {
    let num_tokens_map = get_num_tokens();
    let mut ret = vec![];
    ret.push(NumberToken {
        kind: TT::BEGIN,
        digit: None,
        sign: None,
        expr: None,
    });
    let iterator = s.chars().into_iter();
    for ch in iterator {
        match num_tokens_map.get(&ch) {
            Some(value) => ret.push(value.clone()),
            None => return None,
        }
    }

    ret.push(NumberToken {
        kind: TT::END,
        digit: None,
        sign: None,
        expr: None,
    });

    return Some(ret);
}

#[derive(PartialEq, Eq)]
enum EDigitState {
    NONE,          // <END>, ·
    MULT,          // 微
    MultAmbit,     // 十 (ambiguous: ...十 or 一十)
    DIGIT,         // 一
    DigitWithZero, // 一...零, 零零， 零一...零,
    DELIM,         // 又
    ZERO,          // 零<END>, 零·, 零又, 零微, 零一
    SIGN,          // 負

    ZeroMultAmbig, // 零十 (ambiguous: 零一十 or 零十 or 〇十)
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

struct ParseResult {
    pub sign: i8, // +1/-1
    pub exp: Exp, // one plus exponent of the highest digit
    pub digits: Vec<Digit>,
}

impl ParseResult {
    pub fn new() -> Self {
        Self {
            sign: 1,
            exp: 0,
            digits: vec![],
        }
    }
    pub fn sign(&self) -> i8 {
        self.sign
    }
    pub fn exp(&self) -> Exp {
        self.exp
    }
    pub fn digits(&self) -> &Vec<Digit> {
        &self.digits
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
        let mut arr = vec![];
        for _ in 0..(new_exp - self.exp) {
            arr.push('0');
        }
        self.push(arr)
    }
    pub fn reset_exp(&mut self, new_exp: Exp) {
        self.exp = new_exp;
    }
}

fn parse(tokens: Vec<NumberToken>) -> Option<ParseResult> {
    let mut digit_state = EDigitState::NONE;

    let mut mult_stack = MutlStack::new();
    let mut result = ParseResult::new();

    // parses the number string backwards, discarding <END>
    for token in tokens.iter().rev() {
        if token.kind == TT::END {
            continue;
        }
        if mult_stack.state() == EMultState::SIGN && token.kind() == &TT::BEGIN {
            return None;
        }

        let exp = match token.expr {
            Some(value) => value,
            None => 1,
        };

        match digit_state {
            EDigitState::MultAmbit => {
                match token.kind {
                    // <BEGIN>(一?)十 -> <BEGIN>一十
                    // 負(一?)十 -> 負一十
                    // 又(一?)十 -> 又一十
                    // ·(一?)十 -> ·一十
                    // 分(一?)十絲 -> 分一十絲
                    TT::BEGIN | TT::SIGN | TT::DELIM | TT::DECIMAL | TT::FracMult => {
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
                    TT::IntMult => {
                        if mult_stack.top() < exp {
                            result.push_char('1');
                            digit_state = EDigitState::DIGIT;
                        } else {
                            digit_state = EDigitState::MULT;
                        }
                    }

                    // 零(一?)十 -> 零(一?)十
                    TT::ZERO => {
                        digit_state = EDigitState::MultAmbit;
                    }
                    _ => {}
                }
            }
            EDigitState::ZeroMultAmbig => {
                match token.kind {
                    // <BEGIN>(零一|零|〇)十 -> <BEGIN>〇十
                    // 負(零一|零|〇)十 -> 負〇十
                    // 一(零一|零|〇)十 -> 一〇十
                    // 又(零一|零|〇)十 -> 又〇十
                    // 零(零一|零|〇)十 -> 〇〇十
                    TT::BEGIN | TT::SIGN | TT::DIGIT | TT::DELIM | TT::ZERO => {
                        result.push_char('0');
                        digit_state = EDigitState::DigitWithZero;
                    }

                    // ·(零一|零|〇)十絲 -> ·零一十絲
                    // ·(零一|零|〇)十釐 -> ·〇十釐
                    // ·(零一|零|〇)十分 -> error
                    // ·(零一|零|〇)百分 -> error
                    // 分(零一|零|〇)十絲 -> 分零一十絲
                    // 分(零一|零|〇)十毫 -> 分〇十絲
                    // 分(零一|零|〇)十釐 -> error
                    // 分(零一|零|〇)十分 -> error
                    TT::DECIMAL | TT::FracMult => {
                        if mult_stack.total() + 1 < exp {
                            result.push_char('1');
                            result.push_char('0');
                            digit_state = EDigitState::ZERO;
                        } else if mult_stack.total() + 1 == exp {
                            result.push_char('0');
                            digit_state = EDigitState::DigitWithZero;
                        } else {
                            return None;
                        }
                    }
                    // 千(零一|零|〇)十 -> 千零一十
                    // 百(零一|零|〇)十 -> 百〇十
                    // 萬(零一|零|〇)萬 -> 萬零萬
                    // 百(零一|零|〇)萬 -> 百零萬
                    TT::IntMult => {
                        if token.expr.is_none() {
                            return None;
                        }

                        if mult_stack.top() + 1 < exp {
                            result.push_char('1');
                            result.push_char('0');
                            digit_state = EDigitState::ZERO;
                        } else if mult_stack.top() + 1 == exp {
                            result.push_char('0');
                            digit_state = EDigitState::DigitWithZero;
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
        if mult_stack.state() == EMultState::NONE {
            match token.kind {
                // exponent is correct
                TT::IntMult => {}
                TT::DECIMAL | TT::FracMult => {
                    if !token.expr.is_none() {
                        result.reset_exp(exp);
                    }
                }

                _ => {}
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
            TT::DECIMAL | TT::FracMult => {
                if digit_state == EDigitState::MULT {
                    None
                } else {
                    mult_stack.clear();
                    mult_stack.push(exp);
                    Some(mult_stack.total())
                }
            }

            TT::IntMult => {
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
                    | EDigitState::MultAmbit
                    | EDigitState::DIGIT
                    | EDigitState::DigitWithZero
                    | EDigitState::ZERO
                    | EDigitState::ZeroMultAmbig => {
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

                Some(mult_stack.total())
            }
            _ => None,
        };

        if curr_exp.is_none() {
            return None;
        }

        let cur_exp = curr_exp.unwrap();

        // check for disallowed missing decimal places
        if cur_exp > result.exp() {
            let check = || {
                if token.kind == TT::BEGIN || token.kind == TT::SIGN {
                    return true;
                }

                if digit_state == EDigitState::DELIM || digit_state == EDigitState::ZERO {
                    return true;
                }

                if token.kind == TT::IntMult {
                    return true;
                }

                if token.kind == TT::FracMult || token.kind == TT::DECIMAL {
                    return true;
                }

                return false;
            };

            if !check() {
                return None;
            }

            if mult_stack.state() != EMultState::DONE {
                result.fill_zeros(cur_exp);
            }
        }

        // push the digit, update parser state
        match token.kind {
            TT::BEGIN => {}
            TT::SIGN => {
                result.apply_sign(token.sign.expect("valid sign"));
                digit_state = EDigitState::SIGN;
            }

            TT::DIGIT => {
                result.push_char(token.digit.expect("token digit"));
                if digit_state == EDigitState::ZERO || digit_state == EDigitState::DigitWithZero {
                    digit_state = EDigitState::DigitWithZero;
                } else {
                    digit_state = EDigitState::DIGIT;
                }
            }

            TT::DECIMAL => {
                digit_state = EDigitState::NONE;
            }

            TT::IntMult => {
                digit_state = EDigitState::MultAmbit;
            }

            TT::FracMult => {
                digit_state = EDigitState::MULT;
            }

            TT::DELIM => {
                digit_state = EDigitState::DELIM;
            }

            TT::ZERO => match digit_state {
                EDigitState::NONE | EDigitState::MULT | EDigitState::DIGIT | EDigitState::DELIM => {
                    result.push_char(token.digit.expect("expect digit"));
                    digit_state = EDigitState::ZERO;
                }

                EDigitState::DigitWithZero | EDigitState::ZERO => {
                    result.push_char(token.digit.expect("expect digit"));
                    digit_state = EDigitState::ZERO;
                }

                EDigitState::MultAmbit => {
                    digit_state = EDigitState::ZeroMultAmbig;
                }
                _ => {}
            },
            _ => {}
        }
    }

    if result.digits().len() == 0 {
        return None;
    }

    Some(ParseResult {
        sign: result.sign(),
        exp: result.exp() - result.digits().len() as isize,
        digits: result.digits().to_vec(),
    })
}

fn get_digit(result: &ParseResult, exp: Exp) -> char {
    let idx = exp - result.exp;
    match usize::try_from(idx) {
        Ok(v) => result.digits.get(v).unwrap_or(&'0').clone(),
        Err(_) => '0',
    }
}

fn compare_magnitude(result_a: &ParseResult, result_b: &ParseResult) -> Exp {
    let get_max_exp = |result: &ParseResult| result.exp + (result.digits.len() - 1) as isize;

    let max_exp = get_max_exp(result_a).max(get_max_exp(result_b));
    let min_exp = result_a.exp.min(result_b.exp);

    let mut i = max_exp;

    while i >= min_exp {
        let digit_a = get_digit(result_a, i);
        let digit_b = get_digit(result_b, i);
        if digit_a > digit_b {
            return 1;
        } else if digit_a < digit_b {
            return -1;
        }

        i -= 1;
    }

    return 0;
}

pub fn hanzi2num(s: &str) -> Option<String> {
    let result_2_to_63 = ParseResult {
        sign: 1,
        exp: 0,
        digits: "9223372036854775808".chars().into_iter().rev().collect(),
    };

    let tokens = tokenize(s);
    if tokens.is_none() {
        return None;
    }

    let parse_result = parse(tokens.unwrap());

    if parse_result.is_none() {
        return None;
    }

    let result = parse_result.unwrap();

    let mut str = if result.sign < 0 {
        "-".to_owned()
    } else {
        "".to_owned()
    };

    let print_as_int = match result.exp < 0 {
        false => false,
        true => {
            let c = compare_magnitude(&result, &result_2_to_63);
            if result.sign < 0 {
                c <= 0
            } else {
                c < 0
            }
        }
    };

    if let Some(rend) = result.digits.iter().position(|x| *x != '0') {
        let rend_exp = result.exp + rend as isize;

        let mut rbegin = result.digits.len();
        while result.digits[rbegin - 1] == '0' {
            rbegin -= 1;
        }

        let rbegin_exp = result.exp + rbegin as isize;

        // compute length of fixed and scientific format
        let mut exp_str = String::new();
        let mut print_as_scientific = false;
        if !print_as_int {
            let scientific_exp = result.exp + (rbegin - 1) as isize;
            exp_str += if scientific_exp < 0 { "e-" } else { "e+" };
            exp_str.push_str(scientific_exp.abs().to_string().as_str());

            let fixed_len = if rend_exp < 0 {
                rbegin_exp.max(1) - rend_exp + 1
            } else {
                rbegin_exp
            };
            let scientific_mag_len = if rbegin - rend > 1 {
                rbegin - rend + 1
            } else {
                1
            };
            let temp = (scientific_mag_len + exp_str.len()) as isize;

            if temp < fixed_len {
                print_as_scientific = true;
            }
        }

        if print_as_scientific {
            str.push(result.digits[rbegin - 1]);
            if rbegin - 1 > rend {
                str += ".";
                let mut i = rbegin - 1;
                while i > rend {
                    str.push(result.digits[i - 1]);
                    i -= 1;
                }
            }
            str += exp_str.as_str();
            return Some(str);
        } else {
            let mut i = rbegin_exp.max(1);
            while i > 0 {
                str.push(get_digit(&result, i - 1));
                i -= 1;
            }

            if rend_exp < 0 {
                str += ".";
                let mut i = 0;
                while i > rend_exp {
                    str.push(get_digit(&result, i - 1));
                    i -= 1
                }
            }
            return Some(str);
        }
    } else {
        str.push('0');
        return Some(str);
    }
}

fn str2f64(str: &str) -> Option<f64> {
    match str.parse::<f64>() {
        Ok(value) => Some(value),
        Err(_) => None,
    }
}

#[cfg(test)]
mod test {
    use super::hanzi2num;

    #[test]
    fn test_hanzi2num_一極零二() {
        assert_eq!(
            hanzi2num("一極零二"),
            Some("1000000000000000000000000000000000000000000000002".to_owned())
        )
    }

    #[test]
    fn test_hanzi2num_一極零二又三漠() {
        assert_eq!(
            hanzi2num("一極零二又三漠"),
            Some("1000000000000000000000000000000000000000000000002.000000000003".to_owned())
        )
    }

    #[test]
    fn test_hanzi2num_一極零二京() {
        assert_eq!(
            hanzi2num("一極零二京"),
            Some("1.00000000000000000000000000000002e+48".to_owned())
        )
    }

    #[test]
    fn test_hanzi2num_二十一京二千三百四十五兆六千七百八十億零九百萬零二百五十有一() {
        assert_eq!(
            hanzi2num("二十一京二千三百四十五兆六千七百八十億零九百萬零二百五十有一"),
            Some("212345678009000251".to_owned())
        )
    }
}
