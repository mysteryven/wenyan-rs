use super::token::Token;

fn get_keywords() -> Vec<(Vec<char>, Token)> {
    let mut key_defines = Vec::new();

    key_defines.push(("吾有", Token::Decl));
    key_defines.push(("今有", Token::Decl));
    key_defines.push(("有", Token::DeclShort));
    key_defines.push(("數", Token::Type));
    key_defines.push(("言", Token::Type));
    key_defines.push(("爻", Token::Type));
    key_defines.push(("書之", Token::Print));
    key_defines.push(("名之曰", Token::NameIs));
    key_defines.push(("曰", Token::Is));

    key_defines.push(("陰", Token::False));
    key_defines.push(("陽", Token::True));

    key_defines.push(("加", Token::Plus));
    key_defines.push(("減", Token::Minus));
    key_defines.push(("乘", Token::Star));
    key_defines.push(("於", Token::PrepositionLeft));
    key_defines.push(("以", Token::PrepositionRight));

    key_defines.push(("等於", Token::EqualEqual));
    key_defines.push(("不等於", Token::BangEqual));
    key_defines.push(("不大於", Token::BangGreater));
    key_defines.push(("不小於", Token::BangLess));
    key_defines.push(("大於", Token::Greater));
    key_defines.push(("小於", Token::Less));

    key_defines.push(("昔之", Token::AssignFrom));
    key_defines.push(("今", Token::AssignTo));
    key_defines.push(("者", Token::Conjunction));
    key_defines.push(("其", Token::Prev));
    key_defines.push(("是矣", Token::Sure));

    key_defines.push(("若", Token::If));
    key_defines.push(("若非", Token::Else));
    key_defines.push(("云云", Token::YunYun));
    key_defines.push(("也", Token::Ye));

    key_defines.push(("變", Token::Invert));

    key_defines.push(("夫", Token::Fu));
    key_defines.push(("中無陰乎", Token::And));
    key_defines.push(("中有陽乎", Token::Or));

    key_defines.push(("恆為是", Token::Loop));
    key_defines.push(("乃止", Token::Break));

    let keywords: Vec<(Vec<char>, Token)> = key_defines
        .iter()
        .map(|(str, token)| (str.chars().collect::<Vec<char>>(), token.clone()))
        .collect();

    keywords
}

pub fn get_sorted_keywords() -> Vec<(Vec<char>, Token)> {
    let mut final_keywords = get_keywords();
    for ch in get_number_keywords() {
        final_keywords.push((vec![ch], Token::Number));
    }

    final_keywords.sort_by(|a, b| {
        let (a_chars, _) = a;
        let (b_chars, _) = b;

        b_chars.len().cmp(&a_chars.len())
    });

    final_keywords
}

pub fn get_number_keywords() -> [char; 40] {
    [
        '負', '又', '零', '〇', '一', '二', '三', '四', '五', '六', '七', '八', '九', '十', '百',
        '千', '萬', '億', '兆', '京', '垓', '秭', '穰', '溝', '澗', '正', '載', '極', '分', '釐',
        '毫', '絲', '忽', '微', '纖', '沙', '塵', '埃', '渺', '漠',
    ]
}

#[cfg(test)]
mod test {
    use insta::assert_debug_snapshot;

    use crate::tokenize::keywords::get_sorted_keywords;

    #[test]
    fn test_ordered_keywords() {
        let keywords = get_sorted_keywords();
        let keys: Vec<Vec<char>> = keywords.into_iter().map(|(key, _)| key).collect();
        assert_debug_snapshot!(&keys);
    }
}
