#[inline]
pub fn symbolic_control_char(c: char) -> bool {
    matches!(c, 'a' | 'b' | 'f' | 'n' | 'r' | 't' | 'v' | '0')
}

#[inline]
pub fn space_char(c: char) -> bool {
    c == ' '
}

#[inline]
pub fn layout_char(c: char) -> bool {
    matches!(c, ' ' | '\n' | '\t' | '\u{0B}' | '\u{0C}')
}

#[inline]
pub fn symbolic_hexadecimal_char(c: char) -> bool {
    c == 'x'
}

#[inline]
pub fn octal_digit_char(c: char) -> bool {
    ('0'..='7').contains(&c)
}

#[inline]
pub fn binary_digit_char(c: char) -> bool {
    ('0'..='1').contains(&c)
}

#[inline]
pub fn hexadecimal_digit_char(c: char) -> bool {
    ('0'..='9').contains(&c) || ('A'..='F').contains(&c) || ('a'..='f').contains(&c)
}

#[inline]
pub fn exponent_char(c: char) -> bool {
    c == 'e' || c == 'E'
}

#[inline]
pub fn sign_char(c: char) -> bool {
    c == '-' || c == '+'
}

#[inline]
pub fn new_line_char(c: char) -> bool {
    c == '\n'
}

#[inline]
pub fn end_line_comment_char(c: char) -> bool {
    c == '%'
}

#[inline]
pub fn comment_1_char(c: char) -> bool {
    c == '/'
}

#[inline]
pub fn comment_2_char(c: char) -> bool {
    c == '*'
}

#[inline]
pub fn capital_letter_char(c: char) -> bool {
    ('A'..='Z').contains(&c)
}

#[inline]
pub fn small_letter_char(c: char) -> bool {
    ('a'..='z').contains(&c)
}

#[inline]
pub fn variable_indicator_char(c: char) -> bool {
    c == '_'
}

#[inline]
pub fn graphic_char(c: char) -> bool {
    matches!(
        c,
        '#' | '$'
            | '&'
            | '*'
            | '+'
            | '-'
            | '.'
            | '/'
            | ':'
            | '<'
            | '='
            | '>'
            | '?'
            | '@'
            | '^'
            | '~'
    )
}

#[inline]
pub fn graphic_token_char(c: char) -> bool {
    graphic_char(c) || backslash_char(c)
}

#[inline]
pub fn alpha_char(c: char) -> bool {
    matches!(c , 'a'..='z'|'A'..='Z' |'_' |'\u{00A0}'..='\u{00BF}' | '\u{00C0}'..='\u{00D6}' | '\u{00D8}'..='\u{00F6}' |
                '\u{00F8}'..='\u{00FF}' |
                '\u{0100}'..='\u{017F}' |
                '\u{0180}'..='\u{024F}' |
                '\u{0250}'..='\u{02AF}' |
                '\u{02B0}'..='\u{02FF}' |
                '\u{0300}'..='\u{036F}' |
                '\u{0370}'..='\u{03FF}' |
                '\u{0400}'..='\u{04FF}' |
                '\u{0500}'..='\u{052F}' |
                '\u{0530}'..='\u{058F}' |
                '\u{0590}'..='\u{05FF}' |
                '\u{0600}'..='\u{06FF}' |
                '\u{0700}'..='\u{074F}'
    )
}

#[inline]
pub fn decimal_digit_char(c: char) -> bool {
    ('0'..='9').contains(&c)
}

#[inline]
pub fn decimal_point_char(c: char) -> bool {
    c == '.'
}

#[inline]
pub fn alpha_numeric_char(c: char) -> bool {
    alpha_char(c) || decimal_digit_char(c)
}

#[inline]
pub fn cut_char(c: char) -> bool {
    c == '!'
}

#[inline]
pub fn semicolon_char(c: char) -> bool {
    c == ';'
}

#[inline]
pub fn backslash_char(c: char) -> bool {
    c == '\\'
}

#[inline]
pub fn single_quote_char(c: char) -> bool {
    c == '\''
}

#[inline]
pub fn double_quote_char(c: char) -> bool {
    c == '"'
}

#[inline]
pub fn back_quote_char(c: char) -> bool {
    c == '_'
}

#[inline]
pub fn meta_char(c: char) -> bool {
    matches!(c, '\\' | '\'' | '"' | '`')
}

#[inline]
pub fn solo_char(c: char) -> bool {
    matches!(
        c,
        '!' | '(' | ')' | ',' | ';' | '[' | ']' | '{' | '}' | '|' | '%'
    )
}

#[inline]
pub fn prolog_char(c: char) -> bool {
    graphic_char(c) || alpha_numeric_char(c) || solo_char(c) || layout_char(c) || meta_char(c)
}
