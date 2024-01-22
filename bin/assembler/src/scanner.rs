use std::fmt;

use forge_lib::{address::AddressMode, mnemonic::Mnemonic, operand::Operand, instruction::Instruction, directive::{DirectiveName, Directive}};

use crate::error::ParseError;

pub mod address;
pub mod directive;
pub mod expression;
pub mod instruction;
pub mod line;
pub mod mnemonic;

type TokenResult = Result<Option<Token>, ParseError>;

pub struct Scanner {
    input: Vec<char>,
    cursor: usize,
    pub lines: u32,
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Comment(String),
    Mnemonic(Mnemonic),
    Newline,
    Whitespace,
    AddressU16(u16),
    AddressU8(u8),
    LiteralU8(u8),
    AddressMode(AddressMode),
    Operand(Operand),
    Instruction(Instruction),
    Identifier(String),
    Label(String),
    LocalLabel(String),
    Constant(String, u16),
    DirectiveName(DirectiveName),
    Directive(Directive),
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Comment(string) => {
                write!(f, "Comment: {}", string)
            }
            Token::Mnemonic(mnemonic) => {
                write!(f, "Mnemonic: {}", mnemonic.to_string())
            }
            Token::Newline => {
                write!(f, "Newline token")
            }
            Token::Whitespace => {
                write!(f, "Whitespace token")
            }
            Token::AddressU16(val) => {
                write!(f, "U16 Address: ${:04X}", val)
            }
            Token::AddressU8(val) => {
                write!(f, "U8 Address: ${:02X}", val)
            }
            Token::LiteralU8(val) => {
                write!(f, "U8 Literal: #${:02X}", val)
            }
            Token::AddressMode(address_mode) => {
                write!(f, "Address Mode: {}", address_mode)
            }
            _ => {
                write!(f, "Display not implemented for given token")
            }
        }
    }
}

impl Token {
    pub fn to_generic(&self) -> &'static str {
        match self {
            Token::Mnemonic(_) => "Mnemonic",
            Token::AddressMode(_) => "Address Mode",
            Token::AddressU16(_) => "U16 Address",
            Token::AddressU8(_) => "U8 Address",
            Token::LiteralU8(_) => "Literal U8",
            Token::Comment(_) => "Comment",
            Token::Whitespace => "Whitespace",
            Token::Newline => "Newline",
            Token::Operand(_) => "Operand",
            Token::Instruction(_) => "Instruction",
            Token::Identifier(_) => "Identifier",
            Token::Label(_) => "Label",
            Token::Constant(_, _) => "Constant",
            Token::DirectiveName(_) => "Directive Name",
            Token::Directive(_) => "Directive",
            Token::LocalLabel(_) => "Local Label",
        }
    }
}

impl Scanner {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            cursor: 0,
            lines: 0,
        }
    }

    /// Attempts a parser. Returns Some or None if the result was Some, None, or a non-fatal error
    /// or returns an error if it was a fatal error
    pub fn attempt_parser<F>(&mut self, parser: F) -> TokenResult
    where
        F: Fn(&mut Self) -> TokenResult,
    {
        match parser(self) {
            Ok(Some(token)) => Ok(Some(token)),
            Ok(None) => Ok(None),
            Err(e) if !e.is_fatal() => Ok(None),
            Err(e) => Err(e),
        }
    }

    /// Returns the current cursor. Useful for reporting errors.
    pub fn _cursor(&self) -> usize {
        self.cursor
    }

    /// Returns the next character without advancing the cursor
    /// AKA "Lookahead"
    pub fn peek(&self) -> Option<char> {
        self.input.get(self.cursor).cloned()
    }

    /// Returns true if further progress is not possible.
    pub fn is_done(&self) -> bool {
        self.cursor == self.input.len()
    }

    /// Moves the cursor to the next position
    pub fn next(&mut self) {
        self.cursor += 1;
    }

    /// Checks if the next character is the given character. If so it will consume
    /// the character and return true. Otherwise return false
    pub fn consume_char(&mut self, c: char) -> bool {
        if self.peek() == Some(c) {
            self.next();
            true
        } else {
            false
        }
    }

    pub fn peek_chars(&self, s: &str) -> bool {
        let end_pos = self.cursor + s.len();

        // First check if the end position is in bounds
        if end_pos > self.input.len() {
            return false;
        }

        // Check if the substring of the input from cursor to end_pos
        // matches the string s
        let upcoming_chars: String = self.input[self.cursor..end_pos].iter().collect();

        &upcoming_chars == s
    }

    pub fn consume_chars(&mut self, num: usize) -> bool {
        let end_pos = self.cursor + num;

        // First check if the end position is in bounds
        if end_pos > self.input.len() {
            return false;
        }

        // Now just move the cursor however many
        for _ in 0..num {
            self.next();
        }

        true
    }

    /// Parses a comment if applicable. EBNF is defined as
    ///
    /// comment = ";" any_char*;
    ///
    /// See assembler.ebnf line 36
    fn comment(&mut self) -> TokenResult {
        if self.peek() == Some(';') {
            let start_pos = self.cursor;
            while let Some(c) = self.peek() {
                if c == '\n' {
                    break;
                }
                self.next();
            }
            Ok(Some(Token::Comment(
                self.input[start_pos..self.cursor].iter().collect(),
            )))
        } else {
            Ok(None)
        }
    }

    /// Parses a newline character if applicable. EBNF is defined as
    ///
    /// newline = ? newline character(s) ?
    ///
    /// For now we are only accepting \n. See assembler.ebnf line 42
    fn newline(&mut self) -> TokenResult {
        match self.peek() {
            Some('\n') => {
                self.next();
                Ok(Some(Token::Newline))
            }
            Some(_) => Ok(None),
            None => Err(ParseError::UnexpectedEndOfInput),
        }
    }

    /// Attempts to consume a newline character. If it was successful then return true
    /// and if it didn't then return false
    pub fn consume_newline(&mut self) -> bool {
        match self.newline() {
            Ok(token) => match token {
                Some(Token::Newline) => true,
                _ => false,
            },
            Err(_) => false,
        }
    }

    /// Parses a whitespace character (space or a tab) if applicable. EBNF is defined as
    ///
    /// whitespace = { ? space or tab character ? }
    ///
    /// See assembler.ebnf line 44
    fn whitespace(&mut self) -> TokenResult {
        match self.peek() {
            Some(' ') | Some('\t') => {
                self.next();
                Ok(Some(Token::Whitespace))
            }
            Some(_) => Ok(None),
            None => Err(ParseError::UnexpectedEndOfInput),
        }
    }

    /// Consumes 0 or more whitespace
    pub fn consume_all_whitespace(&mut self) {
        while let Ok(Some(_)) = self.whitespace() {}
    }

    /// Consumes 1 or more whitespaces
    /// Returns true if it consumed at least 1 whitespace and false otherwise
    pub fn _consume_all_whitespace1(&mut self) -> bool {
        let mut count = 0;
        while let Ok(Some(_)) = self.whitespace() {
            count += 1;
        }

        if count > 0 {
            true
        } else {
            false
        }
    }

    /// Attempts to parse a constant (or identifier). The grammar is defined as
    ///
    /// identifier = letter {letter | digit | "_"}
    pub fn identifier(&mut self) -> TokenResult {
        let start_pos = self.cursor;

        // First we need to check that we have a letter
        match self.peek() {
            Some(c) => {
                if !c.is_alphabetic() {
                    return Ok(None);
                }
            }
            None => return Ok(None),
        }

        // Move the cursor
        self.next();

        // Now go through and consume until we don't hit a letter, number, or _
        while let Some(c) = self.peek() {
            if !(c.is_alphanumeric() || c == '_') {
                break;
            }

            // Move the cursor forward
            self.next();
        }

        // Convert the characters to a String
        Ok(Some(Token::Identifier(
            self.input[start_pos..self.cursor].iter().collect(),
        )))
    }

    /// Attempts to parse a label. The grammar is defined as
    ///
    /// label = identifier ":"
    pub fn label(&mut self) -> TokenResult {
        let start_pos = self.cursor;
        let mut is_local = false;

        // Check if it begins with an @
        if self.consume_char('@') {
            is_local = true
        }

        // Get the label name
        let identifier = match self.identifier()? {
            Some(Token::Identifier(value)) => value,
            _ => {
                self.cursor = start_pos;
                return Ok(None);
            }
        };

        // Now consume the :
        if !self.consume_char(':') {
            self.cursor = start_pos;
            return Ok(None);
        }

        if !is_local {
            Ok(Some(Token::Label(identifier)))
        } else {
            Ok(Some(Token::LocalLabel(identifier)))
        }
    }
}

#[cfg(test)]
mod scanner_tests {
    use crate::scanner::Token;

    use super::Scanner;

    #[test]
    fn test_parse_comment() {
        let mut scanner = Scanner::new("; This is a comment");
        let result = scanner.comment();

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            Some(Token::Comment(String::from("; This is a comment")))
        );
    }

    #[test]
    fn test_parse_comment_no_semicolon() {
        let mut scanner = Scanner::new("This is a comment");
        let result = scanner.comment();

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), None);
    }

    #[test]
    fn test_parse_newline() {
        let mut scanner = Scanner::new("\n");
        let result = scanner.newline();

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(Token::Newline));
    }

    #[test]
    fn test_parse_newline_no_error_on_non_newline() {
        let mut scanner = Scanner::new(" ");
        let result = scanner.newline();

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), None);
    }

    #[test]
    fn test_parse_newline_unexpected_end_of_input_error() {
        let mut scanner = Scanner::new("\n");
        // Increase the cursor by one since this will be an error
        scanner.cursor += 1;
        let result = scanner.newline();

        assert!(result.is_err());
    }

    #[test]
    fn test_coonsume_newline() {
        let mut scanner = Scanner::new("\n");
        assert_eq!(scanner.consume_newline(), true);

        let mut scanner = Scanner::new("");
        assert_eq!(scanner.consume_newline(), false);
    }

    #[test]
    fn test_parse_whitespace_space() {
        let mut scanner = Scanner::new(" ");
        let result = scanner.whitespace();

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(Token::Whitespace));
    }

    #[test]
    fn test_parse_whitespace_tab() {
        let mut scanner = Scanner::new("\t");
        let result = scanner.whitespace();

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(Token::Whitespace));
    }

    #[test]
    fn test_parse_whitespace_non_whitespace() {
        let mut scanner = Scanner::new("A");
        let result = scanner.whitespace();

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), None);
    }

    #[test]
    fn test_parse_whitespace_end_of_input_error() {
        let mut scanner = Scanner::new(" ");
        scanner.cursor += 1;
        let result = scanner.whitespace();

        assert!(result.is_err());
    }

    #[test]
    fn test_parse_consume_whitespace1_one_space() {
        let mut scanner = Scanner::new(" ");
        let result = scanner._consume_all_whitespace1();

        assert!(result);
    }

    #[test]
    fn test_parse_consume_whitespace1_no_space() {
        let mut scanner = Scanner::new("nospacesarehere");
        let result = scanner._consume_all_whitespace1();

        assert!(!result);
    }

    #[test]
    fn test_parse_identifier_success() {
        let mut scanner = Scanner::new("PPUCONSTANT");
        let result = scanner.identifier();

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            Some(Token::Identifier(String::from("PPUCONSTANT")))
        )
    }

    #[test]
    fn test_parse_label_success() {
        let mut scanner = Scanner::new("START:");
        let result = scanner.label();

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(Token::Label(String::from("START"))));
    }
}
