use std::collections::HashSet;

use forge_lib::{
    directive::{ByteArgs, Directive, DirectiveName, WordArgs},
    expression::ExpressionNode,
};

use crate::error::ParseError;

use super::{Scanner, Token, TokenResult};

impl Scanner {
    pub fn directive_list(&mut self) -> TokenResult {
        let directives: HashSet<&str> = [
            "WORD", "ORG", "BYTE", "SEGMENT", "INCLUDE", "PROC", "ENDPROC", "ENUM", "ENDENUM",
            "MACRO", "ENDMACRO", "SCOPE", "ENDSCOPE", "ADDR", "CODE"
        ]
        .iter()
        .cloned()
        .collect();

        let start_pos = self.cursor;

        while let Some(c) = self.peek() {
            if !(c.is_ascii_uppercase() || c.is_ascii_lowercase()) {
                break;
            }
            self.next();
        }

        let directive: String = self.input[start_pos..self.cursor].iter().collect();
        let directive = directive.to_ascii_uppercase();

        if directives.contains(directive.as_str()) {
            Ok(Some(Token::DirectiveName(directive.into())))
        } else {
            self.cursor = start_pos;
            Ok(None)
        }
    }

    pub fn directive_args_word(&mut self) -> Result<Option<WordArgs>, ParseError> {
        let start_pos = self.cursor;

        // Try to get a number
        if let Some(number) = self.number()? {
            return Ok(Some(WordArgs::Value(number)));
        }

        // Reset the cursor back
        self.cursor = start_pos;

        // First try to parse an expression
        if let Some(expression) = self.expression()? {
            match expression {
                // If the expression is JUST an identifier, then it is likely just an ident anyways so skip
                ExpressionNode::Identifier(_) => {}
                _ => return Ok(Some(WordArgs::Expression(expression))),
            }
        }

        // Reset the cursor back
        self.cursor = start_pos;

        // Try an identifier
        if let Some(token) = self.identifier()? {
            match token {
                Token::Identifier(ident) => return Ok(Some(WordArgs::Identifier(ident))),
                _ => {}
            }
        }

        // Reset the cursor back
        self.cursor = start_pos;

        Err(ParseError::ValidArgNotFound)
    }

    pub fn directive_args_byte(&mut self) -> Result<Option<ByteArgs>, ParseError> {
        let start_pos = self.cursor;

        // Get a number but make sure that it is the size of a byte
        if let Some(number) = self.number()? {
            if number <= 0xFF {
                return Ok(Some(ByteArgs::Value(number as u8)));
            } else {
                return Err(ParseError::ValueTooLarge);
            }
        }

        // Reset the cursor back
        self.cursor = start_pos;

        // First try to parse an expression
        if let Some(expression) = self.expression()? {
            match expression {
                // If the expression is JUST an identifier, then it is likely just an ident anyways so skip
                ExpressionNode::Identifier(_) => {}
                _ => return Ok(Some(ByteArgs::Expression(expression))),
            }
        }

        // Reset the cursor back
        self.cursor = start_pos;

        // Try an identifier
        if let Some(token) = self.identifier()? {
            match token {
                Token::Identifier(ident) => return Ok(Some(ByteArgs::Identifier(ident))),
                _ => {}
            }
        }

        // Reset the cursor back
        self.cursor = start_pos;

        Err(ParseError::ValidArgNotFound)
    }

    pub fn directive_args_include(&mut self) -> Result<Option<String>, ParseError> {
        let start_pos = self.cursor;

        // Now go through and consume until we don't hit a letter, number,
        while let Some(c) = self.peek() {
            if !(c.is_alphanumeric() || c == '_' || c == '.' || c == '/' || c == '\\') {
                break;
            }

            // Move the cursor forward
            self.next();
        }

        Ok(Some(self.input[start_pos..self.cursor].iter().collect()))
    }

    pub fn directive_args_org(&mut self) -> Result<Option<u16>, ParseError> {
        let start_pos = self.cursor;

        // Reset the cursor back
        if let Some(number) = self.number()? {
            return Ok(Some(number));
        }

        // Reset the cursor
        self.cursor = start_pos;

        Err(ParseError::ValidArgNotFound)
    }

    pub fn directive(&mut self) -> TokenResult {
        let start_pos = self.cursor;

        // First try to parse out a dot
        if !self.consume_char('.') {
            self.cursor = start_pos;
            return Ok(None);
        }

        // Now try to get a directive name
        let directive_name = match self.attempt_parser(Self::directive_list)? {
            Some(token) => match token {
                Token::DirectiveName(name) => name,
                _ => {
                    self.cursor = start_pos;
                    return Ok(None);
                }
            },
            None => {
                self.cursor = start_pos;
                return Ok(None);
            }
        };

        // Consume any whitespace
        self.consume_all_whitespace();

        // Now depending on the name parse out the instructions
        let directive = match directive_name {
            DirectiveName::ORG => match self.directive_args_org()? {
                Some(value) => Directive::ORG(value),
                None => {
                    self.cursor = start_pos;
                    return Ok(None);
                }
            },
            DirectiveName::BYTE => {
                // The bytes could be in a list. First attempt to get something
                let mut byte_args = Vec::new();

                // Need at least one arg
                let arg = match self.directive_args_byte()? {
                    Some(arg) => arg,
                    None => {
                        return Err(ParseError::DirectiveWithNoArg {
                            directive: String::from("BYTE"),
                        })
                    }
                };
                byte_args.push(arg);
                loop {
                    // Now consume any whitespace
                    self.consume_all_whitespace();

                    // Consume a ,
                    if !self.consume_char(',') {
                        break;
                    }

                    // Now consume any whitespace
                    self.consume_all_whitespace();

                    let arg = match self.directive_args_byte()? {
                        Some(arg) => arg,
                        None => break,
                    };

                    byte_args.push(arg);
                }

                Directive::BYTE(byte_args)
            }
            DirectiveName::WORD => {
                // The bytes could be in a list. First attempt to get something
                let mut word_args = Vec::new();

                // Need at least one arg
                let arg = match self.directive_args_word()? {
                    Some(arg) => arg,
                    None => {
                        return Err(ParseError::DirectiveWithNoArg {
                            directive: String::from("WORD"),
                        })
                    }
                };
                word_args.push(arg);
                loop {
                    // Now consume any whitespace
                    self.consume_all_whitespace();

                    // Consume a ,
                    if !self.consume_char(',') {
                        break;
                    }

                    // Consume any whitespace after the comma
                    self.consume_all_whitespace();

                    let arg = match self.directive_args_word()? {
                        Some(arg) => arg,
                        None => break,
                    };

                    word_args.push(arg);
                }

                Directive::WORD(word_args)
            }
            DirectiveName::SEGMENT => {
                // Consume any whitespace
                self.consume_all_whitespace();

                // Consume a "
                if !self.consume_char('"') {
                    self.cursor = start_pos;
                    return Ok(None);
                }

                // Get an identifier
                let ident = match self.identifier()? {
                    Some(Token::Identifier(ident)) => ident,
                    _ => {
                        self.cursor = start_pos;
                        return Ok(None);
                    }
                };

                // Consume a "
                if !self.consume_char('"') {
                    self.cursor = start_pos;
                    return Ok(None);
                }

                Directive::SEGMENT(ident)
            }
            DirectiveName::INCLUDE => {
                self.consume_all_whitespace();

                // Consume a "
                if !self.consume_char('"') {
                    self.cursor = start_pos;
                    return Ok(None);
                }

                // Get an identifier
                let inc_file = match self.directive_args_include()? {
                    Some(file) => file,
                    None => return Ok(None),
                };

                // Consume a "
                if !self.consume_char('"') {
                    self.cursor = start_pos;
                    return Ok(None);
                }

                Directive::INCLUDE(inc_file)
            }
            DirectiveName::PROC => {
                // Consume any whitespace
                self.consume_all_whitespace();

                // Get an identifier
                let ident = match self.identifier()? {
                    Some(Token::Identifier(ident)) => ident,
                    _ => {
                        self.cursor = start_pos;
                        return Ok(None);
                    }
                };

                Directive::PROC(ident)
            }
            DirectiveName::ENDPROC => Directive::ENDPROC,
            DirectiveName::ENUM => {
                // Consume any whitespace
                self.consume_all_whitespace();

                // Get an identifier
                let ident = match self.identifier()? {
                    Some(Token::Identifier(ident)) => ident,
                    _ => {
                        self.cursor = start_pos;
                        return Ok(None);
                    }
                };

                Directive::ENUM(ident)
            }
            DirectiveName::SCOPE => {
                // Consume any whitespace
                self.consume_all_whitespace();

                // Get an identifier
                let ident = match self.identifier()? {
                    Some(Token::Identifier(ident)) => ident,
                    _ => {
                        self.cursor = start_pos;
                        return Ok(None);
                    }
                };

                Directive::SCOPE(ident)
            }
            DirectiveName::ENDSCOPE => Directive::ENDSCOPE,
            DirectiveName::ENDENUM => Directive::ENDENUM,
            DirectiveName::MACRO => {
                // Consume any whitespace
                self.consume_all_whitespace();

                // Get an identifier
                let ident = match self.identifier()? {
                    Some(Token::Identifier(ident)) => ident,
                    _ => {
                        self.cursor = start_pos;
                        return Ok(None);
                    }
                };

                Directive::MACRO(ident)
            }
            DirectiveName::ENDMACRO => Directive::ENDMACRO,
            DirectiveName::CODE => {
                Directive::SEGMENT(String::from("CODE"))
            },
            DirectiveName::ADDR => {
                // The bytes could be in a list. First attempt to get something
                let mut word_args = Vec::new();

                // Need at least one arg
                let arg = match self.directive_args_word()? {
                    Some(arg) => arg,
                    None => {
                        return Err(ParseError::DirectiveWithNoArg {
                            directive: String::from("WORD"),
                        })
                    }
                };
                word_args.push(arg);
                loop {
                    // Now consume any whitespace
                    self.consume_all_whitespace();

                    // Consume a ,
                    if !self.consume_char(',') {
                        break;
                    }

                    // Consume any whitespace after the comma
                    self.consume_all_whitespace();

                    let arg = match self.directive_args_word()? {
                        Some(arg) => arg,
                        None => break,
                    };

                    word_args.push(arg);
                }

                Directive::WORD(word_args)
            }
        };

        Ok(Some(Token::Directive(directive)))
    }
}

#[cfg(test)]
mod directive_test {
    use forge_lib::{
        directive::{ByteArgs, Directive, DirectiveName, WordArgs},
        expression::{BinaryOp, ExpressionNode},
    };

    use crate::{
        error::ParseError,
        scanner::{Scanner, Token},
    };

    #[test]
    fn test_parse_directive_name() {
        let mut scanner = Scanner::new("ORG");
        let result = scanner.directive_list();

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            Some(Token::DirectiveName(DirectiveName::ORG))
        );

        let mut scanner = Scanner::new("BYTE");
        let result = scanner.directive_list();

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            Some(Token::DirectiveName(DirectiveName::BYTE))
        );

        let mut scanner = Scanner::new("WORD");
        let result = scanner.directive_list();

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            Some(Token::DirectiveName(DirectiveName::WORD))
        );

        let mut scanner = Scanner::new("SEGMENT");
        let result = scanner.directive_list();

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            Some(Token::DirectiveName(DirectiveName::SEGMENT))
        );
    }

    #[test]
    fn test_parse_directive_args_word() {
        let mut scanner = Scanner::new("$2000");
        let result = scanner.directive_args_word();

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(WordArgs::Value(0x2000)));

        let mut scanner = Scanner::new("%1000100010001000");
        let result = scanner.directive_args_word();

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(WordArgs::Value(0b1000100010001000)));

        let mut scanner = Scanner::new("PPUCONSTANT");
        let result = scanner.directive_args_word();

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            Some(WordArgs::Identifier(String::from("PPUCONSTANT")))
        );

        let mut scanner = Scanner::new("((mapper & $0f) << 4) | (mirroring & 1)");
        let result = scanner.directive_args_word();

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            Some(WordArgs::Expression(ExpressionNode::BinOp(
                BinaryOp::Or,
                Box::new(ExpressionNode::Parenthesized(Box::new(
                    ExpressionNode::BinOp(
                        BinaryOp::ShiftLeft,
                        Box::new(ExpressionNode::Parenthesized(Box::new(
                            ExpressionNode::BinOp(
                                BinaryOp::And,
                                Box::new(ExpressionNode::Identifier("mapper".to_string())),
                                Box::new(ExpressionNode::Number(0x0F))
                            )
                        ))),
                        Box::new(ExpressionNode::Number(4))
                    )
                ))),
                Box::new(ExpressionNode::Parenthesized(Box::new(
                    ExpressionNode::BinOp(
                        BinaryOp::And,
                        Box::new(ExpressionNode::Identifier("mirroring".to_string())),
                        Box::new(ExpressionNode::Number(1))
                    )
                )))
            )))
        );
    }

    #[test]
    fn test_parse_directive_args_org() {
        let mut scanner = Scanner::new("$2000");
        let result = scanner.directive_args_org();

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(0x2000));
    }

    #[test]
    fn test_parse_directive_args_byte() {
        let mut scanner = Scanner::new("%1000");
        let result = scanner.directive_args_byte();

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(ByteArgs::Value(0b1000)));

        let mut scanner = Scanner::new("$2F");
        let result = scanner.directive_args_byte();

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(ByteArgs::Value(0x2F)));

        let mut scanner = Scanner::new("PPUCONSTANT");
        let result = scanner.directive_args_byte();

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            Some(ByteArgs::Identifier(String::from("PPUCONSTANT")))
        );

        let mut scanner = Scanner::new("((mapper & $0f) << 4) | (mirroring & 1)");
        let result = scanner.directive_args_byte();

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            Some(ByteArgs::Expression(ExpressionNode::BinOp(
                BinaryOp::Or,
                Box::new(ExpressionNode::Parenthesized(Box::new(
                    ExpressionNode::BinOp(
                        BinaryOp::ShiftLeft,
                        Box::new(ExpressionNode::Parenthesized(Box::new(
                            ExpressionNode::BinOp(
                                BinaryOp::And,
                                Box::new(ExpressionNode::Identifier("mapper".to_string())),
                                Box::new(ExpressionNode::Number(0x0F))
                            )
                        ))),
                        Box::new(ExpressionNode::Number(4))
                    )
                ))),
                Box::new(ExpressionNode::Parenthesized(Box::new(
                    ExpressionNode::BinOp(
                        BinaryOp::And,
                        Box::new(ExpressionNode::Identifier("mirroring".to_string())),
                        Box::new(ExpressionNode::Number(1))
                    )
                )))
            )))
        );

        let mut scanner = Scanner::new("$2000");
        let result = scanner.directive_args_byte();

        assert!(result.is_err());
        assert_eq!(result, Err(ParseError::ValueTooLarge))
    }

    #[test]
    fn test_parse_directive_full() {
        let mut scanner = Scanner::new(".ORG $8000");
        let result = scanner.directive();

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            Some(Token::Directive(Directive::ORG(0x8000)))
        );

        let mut scanner = Scanner::new(".byte PPUCONSTANT");
        let result = scanner.directive();

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            Some(Token::Directive(Directive::BYTE(vec![
                ByteArgs::Identifier(String::from("PPUCONSTANT"))
            ])))
        );

        let mut scanner = Scanner::new(".word $2000, $8000, $4400");
        let result = scanner.directive();

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            Some(Token::Directive(Directive::WORD(vec![
                WordArgs::Value(0x2000),
                WordArgs::Value(0x8000),
                WordArgs::Value(0x4400)
            ])))
        );

        let mut scanner = Scanner::new(".SEGMENT \"INES\"");
        let result = scanner.directive();

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            Some(Token::Directive(Directive::SEGMENT(String::from("INES"))))
        );

        let mut scanner = Scanner::new(".include \"../../resources/test/test_01.asm\"");
        let result = scanner.directive();

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            Some(Token::Directive(Directive::INCLUDE(String::from(
                "../../resources/test/test_01.asm"
            ))))
        );

        let mut scanner = Scanner::new(".scope Player");
        let result = scanner.directive();

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            Some(Token::Directive(Directive::SCOPE(String::from("Player"))))
        );
    }
}
