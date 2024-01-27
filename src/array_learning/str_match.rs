// 给你一个字符串 s 和一个字符规律 p，请你来实现一个支持 '.' 和 '*' 的正则表达式匹配。

// '.' 匹配任意单个字符
// '*' 匹配零个或多个前面的那一个元素
// 所谓匹配，是要涵盖 整个 字符串 s的，而不是部分字符串。


fn is_match(s: String, p: String) -> bool {
    // 将字符串转换为字符数组，以便索引访问
    let s_chars: Vec<char> = s.chars().collect();
    let p_chars: Vec<char> = p.chars().collect();
    is_match_helper(&s_chars, 0, &p_chars, 0)
}

fn is_match_helper(s_chars: &[char], s_idx: usize, p_chars: &[char], p_idx: usize) -> bool {
    // 如果模式字符串已经用完，检查待匹配字符串是否也用完
    if p_idx == p_chars.len() {
        return s_idx == s_chars.len();
    }

    // 检查当前字符是否匹配
    let first_match = s_idx < s_chars.len() && 
                      (p_chars[p_idx] == s_chars[s_idx] || p_chars[p_idx] == '.');

    // 如果模式字符串的下一个字符是'*'
    if p_idx + 1 < p_chars.len() && p_chars[p_idx + 1] == '*' {
        // '*'表示前面的字符出现0次或多次
        is_match_helper(s_chars, s_idx, p_chars, p_idx + 2) ||
        (first_match && is_match_helper(s_chars, s_idx + 1, p_chars, p_idx))
    } else {
        first_match && is_match_helper(s_chars, s_idx + 1, p_chars, p_idx + 1)
    }
}
#[test]
fn test(){
    println!("{}",is_match("aaa".to_owned(), "a*".to_owned()));
}