/// Constants used by the parser

pub const DELIMS: [char; 1] = [';'];
pub const KEYWORDS: [&str; 2] = ["let", "var"];
pub const OPERATORS: [char; 5] = ['+', '-', '*', '/', '='];

pub fn is_operator(c: char) -> bool {
    OPERATORS.contains(&c)
}

pub fn is_keyword(s: &str) -> bool {
    KEYWORDS.contains(&s)
}

pub fn is_delim(c: char) -> bool {
    DELIMS.contains(&c)
}
