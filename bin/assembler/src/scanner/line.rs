use forge_lib::line::{Line, MainComponent, Labels};

use crate::error::ParseError;

use super::{
    Scanner, Token, TokenResult,
};

impl Scanner {
    pub fn line(&mut self) -> Result<Line, ParseError> {
        let _start_pos = self.cursor;

        // Consume any whitespace
        self.consume_all_whitespace();

        // Try to parse a comment first. Anything after a comment is not needed
        let comment = match self.attempt_parser(Self::comment)? {
            Some(Token::Comment(comment)) => Some(comment),
            _ => None,
        };

        if comment.is_some() {
            // Consume any whitespace
            self.consume_all_whitespace();

            // Try to consume 1 or more newlines if possible
            let mut newline_count: u32 = 0;
            while self.consume_newline() {
                newline_count += 1;
            }

            // If there wasn't at least 1 newline and this is not the end of the file this is an error
            // If there wasn't a newline but we are at the end of input then we are all set! (This means end of file)
            if newline_count == 0 && !self.is_done() {
                return Err(ParseError::ExpectedNewline);
            }

            // Add the number of newlines to the line tracker
            self.lines += newline_count;
            return Ok(Line {
                comment,
                constant: None,
                label: None,
                main_component: None,
                newlines: newline_count,
            });
        }

        let constant = match self.attempt_parser(Self::constant)? {
            Some(Token::Constant(ident, value)) => Some((ident, value)),
            Some(token) => return Err(ParseError::UnexpectedToken { expected: Token::Constant("".to_string(), 0), received: token, position: self.cursor }),
            None => None
        };

        if constant.is_some() {
            // Consume any whitespace
            self.consume_all_whitespace();

            let comment = match self.attempt_parser(Self::comment)? {
                Some(Token::Comment(comment)) => Some(comment),
                _ => None,
            };

            // Try to consume 1 or more newlines if possible
            let mut newline_count: u32 = 0;
            while self.consume_newline() {
                newline_count += 1;
            }

            // If there wasn't at least 1 newline and this is not the end of the file this is an error
            // If there wasn't a newline but we are at the end of input then we are all set! (This means end of file)
            if newline_count == 0 && !self.is_done() {
                return Err(ParseError::ExpectedNewline);
            }

            // Add the number of newlines to the line tracker
            self.lines += newline_count;

            return Ok(
                Line {
                    comment,
                    constant,
                    label: None,
                    main_component: None,
                    newlines: newline_count
                }
            )
        }

        // Check for a label
        let label = match self.attempt_parser(Self::label)? {
            Some(Token::Label(label)) => Some(Labels::Label(label)),
            Some(Token::LocalLabel(label)) => Some(Labels::LocalLabel(label)),
            _ => None,
        };

        // Consume any whitespace
        self.consume_all_whitespace();

        // Check for either a directive or insturction
        let main_component = match self.attempt_parser(Self::instruction)? {
            Some(Token::Instruction(instruction)) => Some(MainComponent::Instruction(instruction)),
            _ => match self.attempt_parser(Self::directive)? {
                Some(Token::Directive(directive)) => Some(MainComponent::Directive(directive)),
                _ => None
            }
        };

        // Consume any whitespace
        self.consume_all_whitespace();

        // Now check for a comment
        let comment = match self.attempt_parser(Self::comment)? {
            Some(Token::Comment(comment)) => Some(comment),
            _ => None,
        };

        // Try to consume 1 or more newlines if possible
        let mut newline_count: u32 = 0;
        while self.consume_newline() {
            newline_count += 1;
        }

        // If there wasn't at least 1 newline and this is not the end of the file this is an error
        // If there wasn't a newline but we are at the end of input then we are all set! (This means end of file)
        if newline_count == 0 && !self.is_done() {
            return Err(ParseError::ExpectedNewline);
        }

        // Add the number of newlines to the line tracker
        self.lines += newline_count;

        return Ok(Line {
            comment,
            constant: None,
            label,
            main_component,
            newlines: newline_count,
        });
    }

    pub fn constant(&mut self) -> TokenResult {
        let start_pos = self.cursor;
        
        // First try to get an identifier
        let ident = match self.attempt_parser(Self::identifier)? {
            Some(Token::Identifier(ident)) => ident,
            _ => {
                self.cursor = start_pos;
                return Ok(None)
            }
        };

        // Next consume any whitespace
        self.consume_all_whitespace();

        // Now get an = sign
        if !self.consume_char('=') {
            self.cursor = start_pos;
            return Ok(None)
        }

        // Next consume any whitespace
        self.consume_all_whitespace();

        // Now get the number
        let number = match self.number()? {
            Some(number) => number,
            _ => {
                self.cursor = start_pos;
                return Ok(None)
            }
        };

        Ok(Some(Token::Constant(ident, number)))
    }
}

#[cfg(test)]
mod line_tests {
    use forge_lib::{address::AddressMode, mnemonic::Mnemonic, instruction::Instruction, operand::Operand, line::Labels};

    use crate::scanner::{
        line::{Line, MainComponent},
        Scanner, Token,
    };

    #[test]
    fn test_parse_line_comment_only() {
        let mut scanner = Scanner::new("; This is a comment line with no newline!");
        let result = scanner.line();

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            Line {
                comment: Some(String::from("; This is a comment line with no newline!")),
                constant: None,
                label: None,
                main_component: None,
                newlines: 0
            }
        )
    }

    #[test]
    fn test_parse_line_instruction_only() {
        let mut scanner = Scanner::new("STA ($00),y");
        let result = scanner.line();

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            Line {
                comment: None,
                constant: None,
                label: None,
                main_component: Some(MainComponent::Instruction(Instruction {
                    mnemonic: Mnemonic::STA,
                    operand: Some(Operand::AddressMode(AddressMode::IndirectIndexY(0x00)))
                })),
                newlines: 0
            }
        )
    }

    #[test]
    fn test_parse_line_instruction_and_comment_on_line() {
        let mut scanner = Scanner::new("LDA $4400,X; Hey look! This is a comment");
        let result = scanner.line();

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            Line {
                comment: Some(String::from("; Hey look! This is a comment")),
                constant: None,
                label: None,
                main_component: Some(MainComponent::Instruction(Instruction {
                    mnemonic: Mnemonic::LDA,
                    operand: Some(Operand::AddressMode(AddressMode::AbsoluteX(0x4400)))
                })),
                newlines: 0
            }
        )
    }

    #[test]
    fn test_parse_line_instruction_and_comment_on_line_multiple_newlines() {
        let mut scanner = Scanner::new("LDA $4400,X; Hey look! This is a comment\n\n");
        let result = scanner.line();

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            Line {
                comment: Some(String::from("; Hey look! This is a comment")),
                constant: None,
                label: None,
                main_component: Some(MainComponent::Instruction(Instruction {
                    mnemonic: Mnemonic::LDA,
                    operand: Some(Operand::AddressMode(AddressMode::AbsoluteX(0x4400)))
                })),
                newlines: 2
            }
        );
        assert_eq!(scanner.lines, 2);
    }

    #[test]
    fn test_parse_line_label_only() {
        let mut scanner = Scanner::new("START:  ");
        let result = scanner.line();

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            Line {
                comment: None,
                constant: None,
                label: Some(Labels::Label(String::from("START"))),
                main_component: None,
                newlines: 0
            }
        )
    }

    #[test]
    fn test_parse_line_label_instruction_comment() {
        let mut scanner =
            Scanner::new("START: LDA PPUCONSTANT ; Load the PPU into the accumulator");
        let result = scanner.line();

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            Line {
                comment: Some(String::from("; Load the PPU into the accumulator")),
                constant: None,
                label: Some(Labels::Label(String::from("START"))),
                main_component: Some(MainComponent::Instruction(Instruction {
                    mnemonic: Mnemonic::LDA,
                    operand: Some(Operand::AddressMode(AddressMode::ZeroPageOrAbsoluteIdent(String::from("PPUCONSTANT"))))
                })),
                newlines: 0
            }
        );
    }

    #[test]
    fn test_parse_line_constant_only() {
        let mut scanner = Scanner::new("PPUCONSTANT = $2000");
        let result = scanner.line();

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            Line {
                comment: None,
                constant: Some((String::from("PPUCONSTANT"), 0x2000)),
                label: None,
                main_component: None,
                newlines: 0
            }
        )
    }

    #[test]
    fn test_parse_constant() {
        let mut scanner = Scanner::new("PPUCONSTANT = $2000");
        let result = scanner.constant();

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            Some(Token::Constant(String::from("PPUCONSTANT"), 0x2000))
        );

        let mut scanner = Scanner::new("PPUCONSTANT = %1000");
        let result = scanner.constant();

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            Some(Token::Constant(String::from("PPUCONSTANT"), 0b1000))
        )
    }
}
