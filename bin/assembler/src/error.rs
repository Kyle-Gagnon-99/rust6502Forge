use std::fmt;

use crate::scanner::Token;

#[derive(Debug, PartialEq)]
pub enum ParseError {
    UnexpectedEndOfInput,
    UnexpectedToken { expected: Token, received: Token, position: usize },
    ParseIntError { msg: String, position: usize },
    DirectiveWithNoArg { directive: String },
    ExpectedLiteralU8,
    ExpectedAddressU8,
    ExpectedAddressU16,
    ExpectedNewline,
    ExpectedValidMnemonic,
    MissingClosingParenthesis,
    TooManyDigits { msg: String, position: usize },
    ValueTooLarge,
    ValidArgNotFound,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::UnexpectedEndOfInput => {
                write!(f, "Unexpected end of input")
            }
            ParseError::UnexpectedToken { expected, received, position } => {
                write!(f, "Unexpected token at {}: expected {} - received {}", position, expected.to_generic(), received)
            }
            ParseError::ParseIntError { msg, position } => {
                write!(f, "Failed to parse string to integer: {} at {}", msg, position)
            }
            ParseError::DirectiveWithNoArg { directive } => {
                write!(f, "Directive {} found with no argument(s)", directive)
            }
            ParseError::ExpectedLiteralU8 => {
                write!(f, "Expected to parse a literal u8 hex value")
            }
            ParseError::ExpectedAddressU8 => {
                write!(f, "Expected to parse a u8 address hex value")
            }
            ParseError::ExpectedAddressU16 => {
                write!(f, "Expected to parse a u16 address hex value")
            }
            ParseError::ExpectedNewline => {
                write!(f, "Expected a newline")
            }
            ParseError::ExpectedValidMnemonic => {
                write!(f, "Expected a valid operation")
            }
            ParseError::MissingClosingParenthesis => {
                write!(f, "Missing closing parenthesis for expression")
            }
            ParseError::TooManyDigits { msg, position } => {
                write!(f, "Too many digits found in input: {} at {}", msg, position)
            }
            ParseError::ValidArgNotFound => {
                write!(f, "Valid argument not found for directive")
            }
            ParseError::ValueTooLarge => {
                write!(f, "Value too large")
            }
        }
    }
}

impl ParseError {
    /// Returns a boolean value if the current error is a fatal error
    pub fn is_fatal(&self) -> bool {
        match self {
            ParseError::TooManyDigits { msg: _, position: _ } => false,
            ParseError::ExpectedValidMnemonic => false,
            ParseError::ValidArgNotFound => false,
            _ => true
        }
    }
}