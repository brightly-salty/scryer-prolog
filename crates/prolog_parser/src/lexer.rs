use crate::lexical::parse_lossy;
use crate::macros::{
    alpha_numeric_char, back_quote_char, backslash_char, binary_digit_char, capital_letter_char,
    comment_1_char, comment_2_char, cut_char, decimal_digit_char, decimal_point_char,
    double_quote_char, end_line_comment_char, exponent_char, graphic_char, graphic_token_char,
    hexadecimal_digit_char, layout_char, meta_char, new_line_char, octal_digit_char, prolog_char,
    semicolon_char, sign_char, single_quote_char, small_letter_char, solo_char, space_char,
    symbolic_hexadecimal_char, variable_indicator_char,
};
use crate::ordered_float::OrderedFloat;
use crate::rug::Integer;

use ast::{
    rc_atom, Atom, ClauseName, Constant, DoubleQuotes, MachineFlags, ParserError, ParsingStream,
};
use tabled_rc::{TabledData, TabledRc};

use std::convert::TryFrom;
use std::fmt;
use std::io::Read;
use std::rc::Rc;

macro_rules! is_not_eof {
    ($c:expr) => {
        match $c {
            Ok(c) => c,
            Err(ParserError::UnexpectedEOF) => return Ok(true),
            Err(e) => return Err(e),
        }
    };
}

macro_rules! consume_chars_with {
    ($token:expr, $e:expr) => {
        loop {
            match $e {
                Ok(Some(c)) => $token.push(c),
                Ok(None) => continue,
                Err(ParserError::UnexpectedChar(..)) => break,
                Err(e) => return Err(e),
            }
        }
    };
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Constant(Constant),
    Var(Rc<Atom>),
    Open,              // '('
    OpenCT,            // '('
    Close,             // ')'
    OpenList,          // '['
    CloseList,         // ']'
    OpenCurly,         // '{'
    CloseCurly,        // '}'
    HeadTailSeparator, // '|'
    Comma,             // ','
    End,
}

pub struct Lexer<'a, R: Read> {
    pub(crate) atom_tbl: TabledData<Atom>,
    pub(crate) reader: &'a mut ParsingStream<R>,
    pub(crate) flags: MachineFlags,
    pub(crate) line_num: usize,
    pub(crate) col_num: usize,
}

impl<'a, R: Read + fmt::Debug> fmt::Debug for Lexer<'a, R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Lexer")
            .field("atom_tbl", &self.atom_tbl)
            .field("reader", &"&'a mut ParsingStream<R>") // Hacky solution.
            .field("line_num", &self.line_num)
            .field("col_num", &self.col_num)
            .finish()
    }
}

impl<'a, R: Read> Lexer<'a, R> {
    pub fn new(
        atom_tbl: TabledData<Atom>,
        flags: MachineFlags,
        src: &'a mut ParsingStream<R>,
    ) -> Self {
        Lexer {
            atom_tbl,
            flags,
            reader: src,
            line_num: 0,
            col_num: 0,
        }
    }

    fn return_char(&mut self, c: char) {
        if new_line_char(c) {
            self.line_num -= 1;
            self.col_num = 0;
        }

        self.reader.put_back(Ok(c));
    }

    fn skip_char(&mut self) -> Result<char, ParserError> {
        if let Some(Ok(c)) = self.reader.next() {
            self.col_num += 1;

            if new_line_char(c) {
                self.line_num += 1;
                self.col_num = 0;
            }

            Ok(c)
        } else {
            Err(ParserError::UnexpectedEOF)
        }
    }

    /// # Errors
    ///
    /// Will return `Err` if an EOF was gotten but not expected
    pub fn eof(&mut self) -> Result<bool, ParserError> {
        if self.reader.peek().is_none() {
            return Ok(true);
        }

        let mut c = is_not_eof!(self.lookahead_char());

        while layout_char(c) {
            self.skip_char()?;

            if self.reader.peek().is_none() {
                return Ok(true);
            }

            c = is_not_eof!(self.lookahead_char());
        }

        Ok(false)
    }

    /// # Errors
    ///
    /// Will return `Err` if there was an unexpected EOF
    pub fn lookahead_char(&mut self) -> Result<char, ParserError> {
        match self.reader.peek() {
            Some(&Ok(c)) => Ok(c),
            _ => Err(ParserError::UnexpectedEOF),
        }
    }

    fn single_line_comment(&mut self) -> Result<(), ParserError> {
        loop {
            if self.reader.peek().is_none() || new_line_char(self.skip_char()?) {
                break;
            }
        }

        Ok(())
    }

    fn bracketed_comment(&mut self) -> Result<bool, ParserError> {
        // we have already checked that the current lookahead_char is comment_1_char, just skip it
        let c = self.skip_char()?;

        if comment_2_char(self.lookahead_char()?) {
            self.skip_char()?;

            // Keep reading until we find characters '*' and '/'
            // Deliberately skip checks for prolog_char to allow comments to contain any characters,
            // including so-called "extended characters", without having to explicitly add them to a character class.
            let mut c2 = self.lookahead_char()?;
            loop {
                while !comment_2_char(c2) {
                    self.skip_char()?;
                    c2 = self.lookahead_char()?;
                }

                self.skip_char()?;

                c2 = self.lookahead_char()?;
                if comment_1_char(c2) {
                    break;
                }
            }

            if prolog_char(c2) {
                self.skip_char()?;
                Ok(true)
            } else {
                Err(ParserError::NonPrologChar(self.line_num, self.col_num))
            }
        } else {
            self.return_char(c);
            Ok(false)
        }
    }

    fn get_back_quoted_char(&mut self) -> Result<char, ParserError> {
        if back_quote_char(self.lookahead_char()?) {
            let c = self.skip_char()?;

            if back_quote_char(self.lookahead_char()?) {
                self.skip_char()
            } else {
                self.return_char(c);
                Err(ParserError::UnexpectedChar(c, self.line_num, self.col_num))
            }
        } else if single_quote_char(self.lookahead_char()?) {
            self.skip_char()
        } else {
            self.get_non_quote_char()
        }
    }

    fn get_back_quoted_item(&mut self) -> Result<Option<char>, ParserError> {
        if backslash_char(self.lookahead_char()?) {
            let c = self.skip_char()?;

            if new_line_char(self.lookahead_char()?) {
                self.skip_char()?;
                Ok(None)
            } else {
                self.return_char(c);
                Err(ParserError::UnexpectedChar(c, self.line_num, self.col_num))
            }
        } else {
            self.get_back_quoted_char().map(Some)
        }
    }

    fn get_back_quoted_string(&mut self) -> Result<String, ParserError> {
        let c = self.lookahead_char()?;

        if back_quote_char(c) {
            self.skip_char()?;

            let mut token = String::new();
            consume_chars_with!(token, self.get_back_quoted_item());

            if back_quote_char(self.lookahead_char()?) {
                self.skip_char()?;
                Ok(token)
            } else {
                Err(ParserError::MissingQuote(self.line_num, self.col_num))
            }
        } else {
            Err(ParserError::UnexpectedChar(c, self.line_num, self.col_num))
        }
    }

    fn get_single_quoted_item(&mut self) -> Result<Option<char>, ParserError> {
        if backslash_char(self.lookahead_char()?) {
            let c = self.skip_char()?;

            if new_line_char(self.lookahead_char()?) {
                self.skip_char()?;
                return Ok(None);
            }
            self.return_char(c);
        }

        self.get_single_quoted_char().map(Some)
    }

    fn get_single_quoted_char(&mut self) -> Result<char, ParserError> {
        let c = self.lookahead_char()?;

        if single_quote_char(c) {
            self.skip_char()?;

            if single_quote_char(self.lookahead_char()?) {
                self.skip_char()
            } else {
                self.return_char(c);
                Err(ParserError::UnexpectedChar(c, self.line_num, self.col_num))
            }
        } else if double_quote_char(c) || back_quote_char(c) {
            self.skip_char()
        } else {
            self.get_non_quote_char()
        }
    }

    fn get_double_quoted_item(&mut self) -> Result<Option<char>, ParserError> {
        if backslash_char(self.lookahead_char()?) {
            let c = self.skip_char()?;

            if new_line_char(self.lookahead_char()?) {
                self.skip_char()?;
                return Ok(None);
            }
            self.return_char(c);
        }

        self.get_double_quoted_char().map(Some)
    }

    fn get_double_quoted_char(&mut self) -> Result<char, ParserError> {
        if double_quote_char(self.lookahead_char()?) {
            let c = self.skip_char()?;

            if double_quote_char(self.lookahead_char()?) {
                self.skip_char()
            } else {
                self.return_char(c);
                Err(ParserError::UnexpectedChar(c, self.line_num, self.col_num))
            }
        } else if single_quote_char(self.lookahead_char()?)
            || back_quote_char(self.lookahead_char()?)
        {
            self.skip_char()
        } else {
            self.get_non_quote_char()
        }
    }

    fn get_control_escape_sequence(&mut self) -> Result<char, ParserError> {
        let escaped = match self.lookahead_char()? {
            'a' => '\u{07}', // UTF-8 alert
            'b' => '\u{08}', // UTF-8 backspace
            'v' => '\u{0b}', // UTF-8 vertical tab
            'f' => '\u{0c}', // UTF-8 form feed
            't' => '\t',
            'n' => '\n',
            'r' => '\r',
            c => return Err(ParserError::UnexpectedChar(c, self.line_num, self.col_num)),
        };

        self.skip_char()?;
        Ok(escaped)
    }

    fn get_octal_escape_sequence(&mut self) -> Result<char, ParserError> {
        self.escape_sequence_to_char(octal_digit_char, 8)
    }

    fn get_hexadecimal_escape_sequence(&mut self) -> Result<char, ParserError> {
        self.skip_char()?;
        let c = self.lookahead_char()?;

        if hexadecimal_digit_char(c) {
            self.escape_sequence_to_char(hexadecimal_digit_char, 16)
        } else {
            Err(ParserError::UnexpectedChar(c, self.line_num, self.col_num))
        }
    }

    fn escape_sequence_to_char(
        &mut self,
        accept_char: impl Fn(char) -> bool,
        radix: u32,
    ) -> Result<char, ParserError> {
        let mut c = self.lookahead_char()?;
        let mut token = String::new();

        loop {
            token.push(c);

            self.skip_char()?;
            c = self.lookahead_char()?;

            if !accept_char(c) {
                break;
            }
        }

        if backslash_char(c) {
            self.skip_char()?;
            u32::from_str_radix(&token, radix).map_or_else(
                |_| Err(ParserError::ParseBigInt(self.line_num, self.col_num)),
                |n| {
                    char::try_from(n)
                        .map_err(|_| ParserError::Utf8Error(self.line_num, self.col_num))
                },
            )
        } else {
            // on failure, restore the token characters and backslash.
            self.reader.put_back_all(token.chars().map(Ok));
            self.reader.put_back(Ok('\\'));

            Err(ParserError::UnexpectedChar(c, self.line_num, self.col_num))
        }
    }

    fn get_non_quote_char(&mut self) -> Result<char, ParserError> {
        let c = self.lookahead_char()?;

        if graphic_char(c) || alpha_numeric_char(c) || solo_char(c) || space_char(c) {
            self.skip_char()
        } else {
            if !backslash_char(c) {
                return Err(ParserError::UnexpectedChar(c, self.line_num, self.col_num));
            }

            self.skip_char()?;

            let c2 = self.lookahead_char()?;

            if meta_char(c2) {
                self.skip_char()
            } else if octal_digit_char(c2) {
                self.get_octal_escape_sequence()
            } else if symbolic_hexadecimal_char(c2) {
                self.get_hexadecimal_escape_sequence()
            } else {
                self.get_control_escape_sequence()
            }
        }
    }

    fn char_code_list_token(&mut self) -> Result<String, ParserError> {
        let mut token = String::new();

        self.skip_char()?;
        consume_chars_with!(token, self.get_double_quoted_item());

        if double_quote_char(self.lookahead_char()?) {
            self.skip_char()?;
            Ok(token)
        } else {
            Err(ParserError::MissingQuote(self.line_num, self.col_num))
        }
    }

    fn hexadecimal_constant(&mut self) -> Result<Token, ParserError> {
        self.skip_char()?;

        if hexadecimal_digit_char(self.lookahead_char()?) {
            let mut token = String::new();

            while hexadecimal_digit_char(self.lookahead_char()?) {
                token.push(self.skip_char()?);
            }

            isize::from_str_radix(&token, 16)
                .map(|n| Token::Constant(Constant::Fixnum(n)))
                .or_else(|_| {
                    Integer::from_str_radix(&token, 16)
                        .map(|n| Token::Constant(Constant::Integer(Rc::new(n))))
                        .map_err(|_| ParserError::ParseBigInt(self.line_num, self.col_num))
                })
        } else {
            self.return_char('x');
            Err(ParserError::ParseBigInt(self.line_num, self.col_num))
        }
    }

    fn octal_constant(&mut self) -> Result<Token, ParserError> {
        self.skip_char()?;

        if octal_digit_char(self.lookahead_char()?) {
            let mut token = String::new();

            while octal_digit_char(self.lookahead_char()?) {
                token.push(self.skip_char()?);
            }

            isize::from_str_radix(&token, 8)
                .map(|n| Token::Constant(Constant::Fixnum(n)))
                .or_else(|_| {
                    Integer::from_str_radix(&token, 8)
                        .map(|n| Token::Constant(Constant::Integer(Rc::new(n))))
                        .map_err(|_| ParserError::ParseBigInt(self.line_num, self.col_num))
                })
        } else {
            self.return_char('o');
            Err(ParserError::ParseBigInt(self.line_num, self.col_num))
        }
    }

    fn binary_constant(&mut self) -> Result<Token, ParserError> {
        self.skip_char()?;

        if binary_digit_char(self.lookahead_char()?) {
            let mut token = String::new();

            while binary_digit_char(self.lookahead_char()?) {
                token.push(self.skip_char()?);
            }

            isize::from_str_radix(&token, 2)
                .map(|n| Token::Constant(Constant::Fixnum(n)))
                .or_else(|_| {
                    Integer::from_str_radix(&token, 2)
                        .map(|n| Token::Constant(Constant::Integer(Rc::new(n))))
                        .map_err(|_| ParserError::ParseBigInt(self.line_num, self.col_num))
                })
        } else {
            self.return_char('b');
            Err(ParserError::ParseBigInt(self.line_num, self.col_num))
        }
    }

    fn variable_token(&mut self) -> Result<Token, ParserError> {
        let mut s = String::new();
        s.push(self.skip_char()?);

        while alpha_numeric_char(self.lookahead_char()?) {
            s.push(self.skip_char()?);
        }

        Ok(Token::Var(rc_atom(s)))
    }

    fn name_token(&mut self, c: char) -> Result<Token, ParserError> {
        let mut token = String::new();

        if small_letter_char(c) {
            token.push(self.skip_char()?);

            while alpha_numeric_char(self.lookahead_char()?) {
                token.push(self.skip_char()?);
            }
        } else if graphic_token_char(c) {
            token.push(self.skip_char()?);

            while graphic_token_char(self.lookahead_char()?) {
                token.push(self.skip_char()?);
            }
        } else if cut_char(c) || semicolon_char(c) {
            token.push(self.skip_char()?);
        } else if single_quote_char(c) {
            self.skip_char()?;

            consume_chars_with!(token, self.get_single_quoted_item());

            if single_quote_char(self.lookahead_char()?) {
                self.skip_char()?;

                if !token.is_empty() && token.chars().nth(1).is_none() {
                    if let Some(c) = token.chars().next() {
                        return Ok(Token::Constant(Constant::Char(c)));
                    }
                }
            } else {
                return Err(ParserError::InvalidSingleQuotedCharacter(
                    self.lookahead_char()?,
                ));
            }
        } else {
            match self.get_back_quoted_string() {
                Ok(_) => return Err(ParserError::BackQuotedString(self.line_num, self.col_num)),
                Err(e) => return Err(e),
            }
        }

        if token.as_str() == "[]" {
            Ok(Token::Constant(Constant::EmptyList))
        } else {
            Ok(Token::Constant(atom!(token, self.atom_tbl)))
        }
    }

    fn vacate_with_float(&mut self, mut token: String) -> Token {
        self.return_char(token.pop().unwrap());

        let result = OrderedFloat(parse_lossy::<f64, _>(token.as_bytes()));
        Token::Constant(Constant::Float(result))
    }

    /// # Errors
    ///
    /// Will return `Err` if there were any errors while trying to parse a number token
    pub fn number_token(&mut self) -> Result<Token, ParserError> {
        let mut token = String::new();

        token.push(self.skip_char()?);
        let mut c = self.lookahead_char()?;

        while decimal_digit_char(c) {
            token.push(c);
            self.skip_char()?;
            c = self.lookahead_char()?;
        }

        if decimal_point_char(c) {
            self.skip_char()?;

            if self.reader.peek().is_none() {
                self.return_char('.');

                isize::from_str_radix(&token, 10)
                    .map(|n| Token::Constant(Constant::Fixnum(n)))
                    .or_else(|_| {
                        token
                            .parse::<Integer>()
                            .map(|n| Token::Constant(Constant::Integer(Rc::new(n))))
                            .map_err(|_| ParserError::ParseBigInt(self.line_num, self.col_num))
                    })
            } else if decimal_digit_char(self.lookahead_char()?) {
                token.push('.');
                token.push(self.skip_char()?);

                let mut c2 = self.lookahead_char()?;

                while decimal_digit_char(c2) {
                    token.push(c2);
                    self.skip_char()?;
                    c2 = self.lookahead_char()?;
                }

                if exponent_char(self.lookahead_char()?) {
                    token.push(self.skip_char()?);

                    let c = match self.lookahead_char() {
                        Err(_) => return Ok(self.vacate_with_float(token)),
                        Ok(c) => c,
                    };

                    if !sign_char(c) && !decimal_digit_char(c) {
                        return Ok(self.vacate_with_float(token));
                    }

                    if sign_char(c) {
                        token.push(self.skip_char()?);

                        let c = match self.lookahead_char() {
                            Err(_) => {
                                self.return_char(token.pop().unwrap());
                                return Ok(self.vacate_with_float(token));
                            }
                            Ok(c) => c,
                        };

                        if !decimal_digit_char(c) {
                            self.return_char(token.pop().unwrap());
                            return Ok(self.vacate_with_float(token));
                        }
                    }

                    if decimal_digit_char(self.lookahead_char()?) {
                        token.push(self.skip_char()?);

                        while decimal_digit_char(self.lookahead_char()?) {
                            token.push(self.skip_char()?);
                        }

                        let n = OrderedFloat(parse_lossy::<f64, _>(token.as_bytes()));
                        Ok(Token::Constant(Constant::Float(n)))
                    } else {
                        Ok(self.vacate_with_float(token))
                    }
                } else {
                    let n = OrderedFloat(parse_lossy::<f64, _>(token.as_bytes()));
                    Ok(Token::Constant(Constant::Float(n)))
                }
            } else {
                self.return_char('.');

                isize::from_str_radix(&token, 10)
                    .map(|n| Token::Constant(Constant::Fixnum(n)))
                    .or_else(|_| {
                        token
                            .parse::<Integer>()
                            .map(|n| Token::Constant(Constant::Integer(Rc::new(n))))
                            .map_err(|_| ParserError::ParseBigInt(self.line_num, self.col_num))
                    })
            }
        } else if token.starts_with('0') && token.len() == 1 {
            if c == 'x' {
                self.hexadecimal_constant().or_else(|e| {
                    if let ParserError::ParseBigInt(..) = e {
                        isize::from_str_radix(&token, 10)
                            .map(|n| Token::Constant(Constant::Fixnum(n)))
                            .or_else(|_| {
                                token
                                    .parse::<Integer>()
                                    .map(|n| Token::Constant(Constant::Integer(Rc::new(n))))
                                    .map_err(|_| {
                                        ParserError::ParseBigInt(self.line_num, self.col_num)
                                    })
                            })
                    } else {
                        Err(e)
                    }
                })
            } else if c == 'o' {
                self.octal_constant().or_else(|e| {
                    if let ParserError::ParseBigInt(..) = e {
                        isize::from_str_radix(&token, 10)
                            .map(|n| Token::Constant(Constant::Fixnum(n)))
                            .or_else(|_| {
                                token
                                    .parse::<Integer>()
                                    .map(|n| Token::Constant(Constant::Integer(Rc::new(n))))
                                    .map_err(|_| {
                                        ParserError::ParseBigInt(self.line_num, self.col_num)
                                    })
                            })
                    } else {
                        Err(e)
                    }
                })
            } else if c == 'b' {
                self.binary_constant().or_else(|e| {
                    if let ParserError::ParseBigInt(..) = e {
                        isize::from_str_radix(&token, 10)
                            .map(|n| Token::Constant(Constant::Fixnum(n)))
                            .or_else(|_| {
                                token
                                    .parse::<Integer>()
                                    .map(|n| Token::Constant(Constant::Integer(Rc::new(n))))
                                    .map_err(|_| {
                                        ParserError::ParseBigInt(self.line_num, self.col_num)
                                    })
                            })
                    } else {
                        Err(e)
                    }
                })
            } else if single_quote_char(c) {
                self.skip_char()?;

                if backslash_char(self.lookahead_char()?) {
                    self.skip_char()?;

                    if new_line_char(self.lookahead_char()?) {
                        self.return_char('\\');
                        self.return_char('\'');

                        return Ok(Token::Constant(Constant::Fixnum(0)));
                    }
                    self.return_char('\\');
                }

                self.get_single_quoted_char()
                    .map(|c| Token::Constant(Constant::Fixnum(c as isize)))
                    .or_else(|_| {
                        self.return_char(c);

                        isize::from_str_radix(&token, 10)
                            .map(|n| Token::Constant(Constant::Fixnum(n)))
                            .or_else(|_| {
                                token
                                    .parse::<Integer>()
                                    .map(|n| Token::Constant(Constant::Integer(Rc::new(n))))
                                    .map_err(|_| {
                                        ParserError::ParseBigInt(self.line_num, self.col_num)
                                    })
                            })
                    })
            } else {
                isize::from_str_radix(&token, 10)
                    .map(|n| Token::Constant(Constant::Fixnum(n)))
                    .or_else(|_| {
                        token
                            .parse::<Integer>()
                            .map(|n| Token::Constant(Constant::Integer(Rc::new(n))))
                            .map_err(|_| ParserError::ParseBigInt(self.line_num, self.col_num))
                    })
            }
        } else {
            isize::from_str_radix(&token, 10)
                .map(|n| Token::Constant(Constant::Fixnum(n)))
                .or_else(|_| {
                    token
                        .parse::<Integer>()
                        .map(|n| Token::Constant(Constant::Integer(Rc::new(n))))
                        .map_err(|_| ParserError::ParseBigInt(self.line_num, self.col_num))
                })
        }
    }

    /// # Errors
    ///
    /// Will return `Err` if there were any errors while scanning for layout characters
    pub fn scan_for_layout(&mut self) -> Result<bool, ParserError> {
        let mut layout_inserted = false;

        loop {
            match self.lookahead_char() {
                Ok(c) if layout_char(c) || new_line_char(c) => {
                    self.skip_char()?;
                    layout_inserted = true;
                }
                Ok(c) if end_line_comment_char(c) => {
                    self.single_line_comment()?;
                    layout_inserted = true;
                }
                Ok(c) if comment_1_char(c) => {
                    if self.bracketed_comment()? {
                        layout_inserted = true;
                    } else {
                        break;
                    }
                }
                _ => break,
            };
        }

        Ok(layout_inserted)
    }

    /// # Errors
    ///
    /// Will return `Err` if there were any errors while getting the next token
    pub fn next_token(&mut self) -> Result<Token, ParserError> {
        let layout_inserted = self.scan_for_layout()?;
        let cr = self.lookahead_char();

        match cr {
            Ok(c) => {
                if capital_letter_char(c) || variable_indicator_char(c) {
                    return self.variable_token();
                }

                if c == ',' {
                    self.skip_char()?;
                    return Ok(Token::Comma);
                }

                if c == ')' {
                    self.skip_char()?;
                    return Ok(Token::Close);
                }

                if c == '(' {
                    self.skip_char()?;
                    return Ok(if layout_inserted {
                        Token::Open
                    } else {
                        Token::OpenCT
                    });
                }

                if c == '.' {
                    self.skip_char()?;

                    match self.lookahead_char() {
                        Ok(c) if layout_char(c) || c == '%' => {
                            if new_line_char(c) {
                                self.skip_char()?;
                            }

                            return Ok(Token::End);
                        }
                        Err(ParserError::UnexpectedEOF) => {
                            return Ok(Token::End);
                        }
                        _ => {
                            self.return_char('.');
                        }
                    };
                }

                if decimal_digit_char(c) {
                    return self.number_token();
                }

                if c == ']' {
                    self.skip_char()?;
                    return Ok(Token::CloseList);
                }

                if c == '[' {
                    self.skip_char()?;
                    return Ok(Token::OpenList);
                }

                if c == '|' {
                    self.skip_char()?;
                    return Ok(Token::HeadTailSeparator);
                }

                if c == '{' {
                    self.skip_char()?;
                    return Ok(Token::OpenCurly);
                }

                if c == '}' {
                    self.skip_char()?;
                    return Ok(Token::CloseCurly);
                }

                if c == '"' {
                    let s = self.char_code_list_token()?;

                    if let DoubleQuotes::Atom = self.flags.double_quotes {
                        let s = clause_name!(s, self.atom_tbl);
                        return Ok(Token::Constant(Constant::Atom(s, None)));
                    }
                    let s = Rc::new(s);
                    return Ok(Token::Constant(Constant::String(s)));
                }

                self.name_token(c)
            }
            Err(e) => Err(e),
        }
    }
}
