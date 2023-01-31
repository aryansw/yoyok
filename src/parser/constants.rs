/// Constants used by the parser
const DELIMS: [char; 9] = [';', ':', ',', '(', ')', '{', '}', '[', ']'];
const KEYWORDS: [&str; 8] = ["let", "var", "if", "else", "true", "false", "fn", "while"];
const OPERATORS: [char; 6] = ['+', '-', '*', '/', '=', '>'];
const COMMENT: char = '#';

pub fn is_comment(c: char) -> bool {
    c == COMMENT
}

pub fn is_operator(c: char) -> bool {
    OPERATORS.contains(&c)
}

pub fn is_keyword(s: &str) -> bool {
    KEYWORDS.contains(&s)
}

pub fn is_delim(c: char) -> bool {
    DELIMS.contains(&c)
}
