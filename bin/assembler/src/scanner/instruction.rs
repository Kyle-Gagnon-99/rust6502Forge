use forge_lib::{address::AddressMode, mnemonic::Mnemonic, operand::Operand, instruction::Instruction, expression::ExpressionNode};

use crate::error::ParseError;

use super::{
    Scanner, Token,
    TokenResult,
};

impl Scanner {
    pub fn instruction(&mut self) -> TokenResult {
        let start_pos = self.cursor;

        // Get the mneomnic, if there isn't one then return Ok(None)
        let mnemonic = match self.mnemonic()? {
            Some(Token::Mnemonic(mneomnic)) => mneomnic,
            Some(token) => {
                self.cursor = start_pos;
                return Err(ParseError::UnexpectedToken {
                    expected: Token::Mnemonic(Mnemonic::ADC),
                    received: token,
                    position: self.cursor,
                });
            }
            None => {
                self.cursor = start_pos;
                return Ok(None);
            }
        };

        // Consume whitespaces
        self.consume_all_whitespace();

        // Get operand
        let operand = match self.operand()? {
            Some(Token::Operand(operand)) => Some(operand),
            Some(token) => {
                self.cursor = start_pos;
                return Err(ParseError::UnexpectedToken {
                    expected: Token::Operand(Operand::AddressMode(AddressMode::Accumulator)),
                    received: token,
                    position: self.cursor,
                });
            }
            None => None,
        };

        // Construct the instruction
        let instruction = Instruction { mnemonic, operand };

        Ok(Some(Token::Instruction(instruction)))
    }

    pub fn operand(&mut self) -> TokenResult {
        let start_pos = self.cursor;
        let mut error: Option<ParseError> = None;

        match self.indirect_index_y_mode() {
            Ok(value) => match value {
                Some(Token::AddressMode(addr_mode)) => {
                    return Ok(Some(Token::Operand(Operand::AddressMode(addr_mode))))
                },
                _ => {}
            }
            _ => {}
        }

        // Try the parser for an expression
        match self.expression() {
            Ok(result) => match result {
                Some(ExpressionNode::Identifier(_)) => {},
                Some(ExpressionNode::Number(_)) => {},
                Some(ExpressionNode::ScopedReference(_)) => {},
                Some(expression) => {
                    return Ok(Some(Token::Operand(Operand::Expression(expression))))
                }
                None => {},
            }
            Err(err) => match err {
                ParseError::MissingClosingParenthesis => {
                    error = Some(err);
                }
                _ => {
                    return Err(err);
                }
            }
        }

        // Reset the cursor
        self.cursor = start_pos;
    
        // Try the parser on an address mode first
        if let Some(token) = self.attempt_parser(Self::address_modes)? {
            match token {
                Token::AddressMode(address_mode) => {
                    return Ok(Some(Token::Operand(Operand::AddressMode(address_mode))))
                }
                _ => {
                    return Err(ParseError::UnexpectedToken {
                        expected: Token::AddressMode(AddressMode::Accumulator),
                        received: token,
                        position: self.cursor,
                    })
                }
            }
        }

        // Reset the cursor
        self.cursor = start_pos;

        if self.consume_char('@') {
            if let Some(token) = self.attempt_parser(Self::identifier)? {
                match token {
                    Token::Identifier(ident) => {
                        return Ok(Some(Token::Operand(Operand::LocalLabel(ident))))
                    }
                    _ => {
                        return Err(ParseError::UnexpectedToken {
                            expected: Token::AddressMode(AddressMode::Accumulator),
                            received: token,
                            position: self.cursor,
                        })                        
                    }
                }
            }
        }

        self.cursor = start_pos;

        // If we encountered an error earlier and we are done attempting all parsers,
        // then return the error
        if error.is_some() {
            return Err(error.unwrap())
        }

        Ok(None)
    }
}

#[cfg(test)]
mod instruction_tests {
    use forge_lib::{address::AddressMode, mnemonic::Mnemonic, expression::{ExpressionNode, BinaryOp}};

    use crate::scanner::{
        instruction::{Instruction, Operand},
        Scanner, Token,
    };

    #[test]
    fn test_parse_operand_address_mode() {
        let mut scanner = Scanner::new("$44");
        let result = scanner.operand();

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            Some(Token::Operand(Operand::AddressMode(AddressMode::ZeroPage(
                0x44
            ))))
        )
    }

    #[test]
    fn test_parse_operand_address_mode_local_label() {
        let mut scanner = Scanner::new("@loop");
        let result = scanner.operand();

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            Some(Token::Operand(Operand::LocalLabel(String::from("loop"))))
        )
    }

    #[test]
    fn test_parse_instruction_operand_address_mode() {
        let mut scanner = Scanner::new("sta ($44),y");
        let result = scanner.instruction();

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            Some(Token::Instruction(Instruction {
                mnemonic: Mnemonic::STA,
                operand: Some(Operand::AddressMode(AddressMode::IndirectIndexY(0x44)))
            }))
        )
    }

    #[test]
    fn test_parse_instruction_operand_identifier() {
        let mut scanner = Scanner::new("LDA PPUCONSTANT");
        let result = scanner.instruction();

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            Some(Token::Instruction(Instruction {
                mnemonic: Mnemonic::LDA,
                operand: Some(Operand::AddressMode(AddressMode::ZeroPageOrAbsoluteIdent(String::from("PPUCONSTANT"))))
            }))
        );
    }

    #[test]
    fn test_parse_instruction_operand_expression() {
        let mut scanner = Scanner::new("LDA PPUCONSTANT + 1");
        let result = scanner.instruction();

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            Some(Token::Instruction(Instruction {
                mnemonic: Mnemonic::LDA,
                operand: Some(Operand::Expression(ExpressionNode::BinOp(
                    BinaryOp::Add,
                    Box::new(ExpressionNode::Identifier(String::from("PPUCONSTANT"))),
                    Box::new(ExpressionNode::Number(1))
                )))
            }))
        )
    }

    #[test]
    fn test_parse_instruction_no_operand() {
        let mut scanner = Scanner::new("TAX");
        let result = scanner.instruction();

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            Some(Token::Instruction(Instruction {
                mnemonic: Mnemonic::TAX,
                operand: None
            }))
        );
    }
}
