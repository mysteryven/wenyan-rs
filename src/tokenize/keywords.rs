use insta::assert_debug_snapshot;

use super::token::Token;

pub fn get_sorted_keywords() -> Vec<(Vec<char>, Token)> {
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

    final_keywords
}

pub fn get_number_keywords() -> [char; 40] {
    [
        '負', '又', '零', '〇', '一', '二', '三', '四', '五', '六', '七', '八', '九', '十', '百',
        '千', '萬', '億', '兆', '京', '垓', '秭', '穰', '溝', '澗', '正', '載', '極', '分', '釐',
        '毫', '絲', '忽', '微', '纖', '沙', '塵', '埃', '渺', '漠',
    ]
}

#[test]
fn test_ordered_keywords() {
    let keywords = get_sorted_keywords();
    let keys: Vec<Vec<char>> = keywords.into_iter().map(|(key, _)| key).collect();
    assert_debug_snapshot!(&keys);
}
