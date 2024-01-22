use forge_lib::{address::AddressMode, expression::ExpressionNode};

use crate::{
    error::ParseError,
    scanner::{Scanner, Token, TokenResult},
};

impl Scanner {
    pub fn immediate_mode(&mut self) -> TokenResult {
        let start_pos = self.cursor;

        // Grab a literal u8 (#$00)
        let address = self.literal_u8()?;

        // If it was successful to parse, then get the address and return the address mode
        match address {
            Some(Token::LiteralU8(val)) => {
                Ok(Some(Token::AddressMode(AddressMode::Immediate(val))))
            }
            Some(_) => Err(ParseError::ExpectedLiteralU8),
            None => {
                // Check if there is an identifier then
                self.cursor = start_pos;

                if !self.consume_char('#') {
                    self.cursor = start_pos;
                    return Ok(None)
                }

                match self.expression()? {
                    Some(ExpressionNode::Identifier(ident)) => {
                        Ok(Some(Token::AddressMode(AddressMode::ImmediateIdent(ident))))
                    }
                    Some(ExpressionNode::ScopedReference(scoped_ref)) => {
                        Ok(Some(Token::AddressMode(AddressMode::ImmediateScopedRef(scoped_ref))))
                    }
                    Some(_) => Err(ParseError::ExpectedLiteralU8),
                    None => {
                        self.cursor = start_pos;
                        return Ok(None);
                    }
                }
            }
        }
    }

    pub fn zero_page_mode(&mut self) -> TokenResult {
        let start_pos = self.cursor;

        // Grab an address u8 ($00)
        let address = self.address_u8()?;

        match address {
            Some(Token::AddressU8(val)) => Ok(Some(Token::AddressMode(AddressMode::ZeroPage(val)))),
            Some(_) => Err(ParseError::ExpectedAddressU8),
            None => match self.expression()? {
                Some(ExpressionNode::Identifier(ident)) => Ok(Some(Token::AddressMode(
                    AddressMode::ZeroPageOrAbsoluteIdent(ident),
                ))),
                Some(ExpressionNode::ScopedReference(scoped_ref)) => {
                    Ok(Some(Token::AddressMode(AddressMode::ZeroPageOrAbsoluteScopedRef(scoped_ref))))
                },
                Some(_) => Err(ParseError::ExpectedAddressU8),
                None => {
                    self.cursor = start_pos;
                    return Ok(None);
                }
            },
        }
    }

    /// Parses the given input into Zero Page X addressing mode if it can. EBNF is defined as
    ///
    /// zero_page_x_mode = address_u8 [whitespace] "," [whitespace] "X";
    pub fn zero_page_x_mode(&mut self) -> TokenResult {
        let start_pos = self.cursor;

        // Grab an address u8 ($00)
        let address = self.address_u8()?;

        // Now grab the value of the address
        let value = match address {
            Some(Token::AddressU8(val)) => AddressMode::ZeroPageX(val),
            Some(_) => return Err(ParseError::ExpectedAddressU8),
            None => match self.expression()? {
                Some(ExpressionNode::Identifier(ident)) => {
                    AddressMode::ZeroPageOrAbsoluteXIdent(ident)
                },
                Some(ExpressionNode::ScopedReference(scoped_ref)) => {
                    AddressMode::ZeroPageOrAbsoluteXScopedRef(scoped_ref)
                },
                Some(_) => return Err(ParseError::ExpectedAddressU8),
                None => {
                    self.cursor = start_pos;
                    return Ok(None);
                }
            },
        };

        // Now consume any number of whitespaces
        self.consume_all_whitespace();

        // The next character is a comma ,
        if !self.consume_char(',') {
            self.cursor = start_pos;
            return Ok(None);
        }

        // Consume any number of whitespaces
        self.consume_all_whitespace();

        // Now consume an upper or lowercase X
        if !self.consume_char('X') && !self.consume_char('x') {
            self.cursor = start_pos;
            return Ok(None);
        }

        Ok(Some(Token::AddressMode(value)))
    }

    /// Parses the given input into Zero Page Y addressing mode if it can. EBNF is defined as
    ///
    /// zero_page_y_mode = address_u8 [whitespace] "," [whitespace] "Y";
    pub fn zero_page_y_mode(&mut self) -> TokenResult {
        let start_pos = self.cursor;

        // Grab an address u8 ($00)
        let address = self.address_u8()?;

        // Now grab the value of the address
        let value = match address {
            Some(Token::AddressU8(val)) => AddressMode::ZeroPageY(val),
            Some(_) => return Err(ParseError::ExpectedAddressU8),
            None => match self.expression()? {
                Some(ExpressionNode::Identifier(ident)) => {
                    AddressMode::ZeroPageOrAbsoluteYIdent(ident)
                },
                Some(ExpressionNode::ScopedReference(scoped_ref)) => {
                    AddressMode::ZeroPageOrAbsoluteYScopedRef(scoped_ref)
                },
                Some(_) => return Err(ParseError::ExpectedAddressU8),
                None => {
                    self.cursor = start_pos;
                    return Ok(None);
                }
            },
        };

        // Now consume any number of whitespaces
        self.consume_all_whitespace();

        // The next character is a comma ,
        if !self.consume_char(',') {
            self.cursor = start_pos;
            return Ok(None);
        }

        // Consume any number of whitespaces
        self.consume_all_whitespace();

        // Now consume an upper or lowercase X
        if !self.consume_char('Y') && !self.consume_char('y') {
            self.cursor = start_pos;
            return Ok(None);
        }

        Ok(Some(Token::AddressMode(value)))
    }

    pub fn absolute_mode(&mut self) -> TokenResult {
        let start_pos = self.cursor;

        // Grab an address u16 $0000
        let address = self.address_u16()?;

        match address {
            Some(Token::AddressU16(val)) => {
                Ok(Some(Token::AddressMode(AddressMode::Absolute(val))))
            }
            Some(_) => Err(ParseError::ExpectedAddressU8),
            None => match self.expression()? {
                Some(ExpressionNode::Identifier(ident)) =>
                    return Ok(Some(Token::AddressMode(AddressMode::ZeroPageOrAbsoluteIdent(ident)))),
                Some(ExpressionNode::ScopedReference(scoped_ref)) => {
                    return Ok(Some(Token::AddressMode(AddressMode::ZeroPageOrAbsoluteScopedRef(scoped_ref))))
                },
                Some(_) => return Err(ParseError::ExpectedAddressU8),
                None => {
                    self.cursor = start_pos;
                    return Ok(None);
                }
            },
        }
    }

    /// Parses the given input into Absolute X addressing mode if it can. EBNF is defined as
    ///
    /// absolute_x_mode = address_u16 [whitespace] "," [whitespace] "X";
    pub fn absolute_x_mode(&mut self) -> TokenResult {
        let start_pos = self.cursor;

        // Grab an address u8 ($00)
        let address = self.address_u16()?;

        // Now grab the value of the address
        let value = match address {
            Some(Token::AddressU16(val)) => AddressMode::AbsoluteX(val),
            Some(_) => return Err(ParseError::ExpectedAddressU16),
            None => match self.expression()? {
                Some(ExpressionNode::Identifier(ident)) => {
                    AddressMode::ZeroPageOrAbsoluteXIdent(ident)
                },
                Some(ExpressionNode::ScopedReference(scoped_ref)) => {
                    AddressMode::ZeroPageOrAbsoluteXScopedRef(scoped_ref)
                },
                Some(_) => return Err(ParseError::ExpectedAddressU8),
                None => {
                    self.cursor = start_pos;
                    return Ok(None);
                }
            },
        };

        // Now consume any number of whitespaces
        self.consume_all_whitespace();

        // The next character is a comma ,
        if !self.consume_char(',') {
            self.cursor = start_pos;
            return Ok(None);
        }

        // Consume any number of whitespaces
        self.consume_all_whitespace();

        // Now consume an upper or lowercase X
        if !self.consume_char('X') && !self.consume_char('x') {
            self.cursor = start_pos;
            return Ok(None);
        }

        Ok(Some(Token::AddressMode(value)))
    }

    /// Parses the given input into Absolute Y addressing mode if it can. EBNF is defined as
    ///
    /// absolute_y_mode = address_u16 [whitespace] "," [whitespace] "Y";
    pub fn absolute_y_mode(&mut self) -> TokenResult {
        let start_pos = self.cursor;

        // Grab an address u8 ($00)
        let address = self.address_u16()?;

        // Now grab the value of the address
        let value = match address {
            Some(Token::AddressU16(val)) => AddressMode::AbsoluteY(val),
            Some(_) => return Err(ParseError::ExpectedAddressU16),
            None => match self.expression()? {
                Some(ExpressionNode::Identifier(ident)) => {
                    AddressMode::ZeroPageOrAbsoluteYIdent(ident)
                },
                Some(ExpressionNode::ScopedReference(scoped_ref)) => {
                    AddressMode::ZeroPageOrAbsoluteYScopedRef(scoped_ref)
                },
                Some(_) => return Err(ParseError::ExpectedAddressU8),
                None => {
                    self.cursor = start_pos;
                    return Ok(None);
                }
            },
        };

        // Now consume any number of whitespaces
        self.consume_all_whitespace();

        // The next character is a comma ,
        if !self.consume_char(',') {
            self.cursor = start_pos;
            return Ok(None);
        }

        // Consume any number of whitespaces
        self.consume_all_whitespace();

        // Now consume an upper or lowercase Y
        if !self.consume_char('Y') && !self.consume_char('y') {
            self.cursor = start_pos;
            return Ok(None);
        }

        Ok(Some(Token::AddressMode(value)))
    }

    /// Parses into Indirect Indexed X addressing mode. The EBNF defintion is
    ///
    /// indexed_indirect_x_mode = "(" [whitespace] address_u8 [whitespace] "," [whitespace] "X" [whitespace] ")";
    pub fn indexed_indirect_x_mode(&mut self) -> TokenResult {
        let start_pos = self.cursor;

        // Check to see if we have a (
        if !self.consume_char('(') {
            return Ok(None);
        }

        // Consume any whitespaces
        self.consume_all_whitespace();

        // Grab an address u8 ($00)
        let address = self.address_u8()?;

        // Now consume a u8 address
        let value = match address {
            Some(Token::AddressU8(val)) => AddressMode::IndexedIndirectX(val),
            Some(_) => return Err(ParseError::ExpectedAddressU8),
            None => match self.expression()? {
                Some(ExpressionNode::Identifier(ident)) => {
                    AddressMode::IndexedIndirectXIdent(ident)
                },
                Some(ExpressionNode::ScopedReference(scoped_ref)) => {
                    AddressMode::IndexedIndirectXScopedRef(scoped_ref)
                }
                _ => {
                    self.cursor = start_pos;
                    return Ok(None)
                }
            }
        };

        // Consume any number of whitespaces
        self.consume_all_whitespace();

        // Consume a comma
        if !self.consume_char(',') {
            self.cursor = start_pos;
            return Ok(None);
        }

        // Consume any number of whitespaces
        self.consume_all_whitespace();

        // Consume a lower case or upper case X
        if !self.consume_char('X') && !self.consume_char('x') {
            self.cursor = start_pos;
            return Ok(None);
        }

        // Consume any number of whitespaces
        self.consume_all_whitespace();

        // Consume a )
        if !self.consume_char(')') {
            self.cursor = start_pos;
            return Ok(None);
        }

        Ok(Some(Token::AddressMode(value)))
    }

    /// Parses into indirect index y address mode. The grammar is defined as
    ///
    /// indirect_index_y_mode = "(" [whitespace] address_u8 [whitespace] ")" [whitespace] "," [whitespace] "Y"
    pub fn indirect_index_y_mode(&mut self) -> TokenResult {
        let start_pos = self.cursor;

        // Check to see if we have a (
        if !self.consume_char('(') {
            return Ok(None);
        }

        // Consume any whitespaces
        self.consume_all_whitespace();

        // Grab an address u8 ($00)
        let address = self.address_u8()?;

        // Now consume a u8 address
        let value = match address {
            Some(Token::AddressU8(val)) => AddressMode::IndirectIndexY(val),
            Some(_) => return Err(ParseError::ExpectedAddressU8),
            None => match self.expression()? {
                Some(ExpressionNode::Identifier(ident)) => {
                    AddressMode::IndirectIndexYIdent(ident)
                },
                Some(ExpressionNode::ScopedReference(scoped_ref)) => {
                    AddressMode::IndirectIndexYScopedRef(scoped_ref)
                }
                _ => {
                    self.cursor = start_pos;
                    return Ok(None)
                }
            }
        };

        // Consume any number of whitespaces
        self.consume_all_whitespace();

        // Consume a )
        if !self.consume_char(')') {
            self.cursor = start_pos;
            return Ok(None);
        }

        // Consume any number of whitespaces
        self.consume_all_whitespace();

        // Consume a comma
        if !self.consume_char(',') {
            self.cursor = start_pos;
            return Ok(None);
        }

        // Consume any number of whitespaces
        self.consume_all_whitespace();

        // Consume a lower case or upper case X
        if !self.consume_char('Y') && !self.consume_char('y') {
            self.cursor = start_pos;
            return Ok(None);
        }

        Ok(Some(Token::AddressMode(value)))
    }

    /// Parses into accumulator mode. The EBNF is defined as
    ///
    /// accumalator_mode = "A";
    fn _accumulator_mode(&mut self) -> TokenResult {
        if !self.consume_char('A') {
            return Ok(None);
        }

        Ok(Some(Token::AddressMode(AddressMode::Accumulator)))
    }
}

#[cfg(test)]
pub mod address_modes_tests {
    use forge_lib::address::AddressMode;

    use crate::scanner::{Scanner, Token};

    #[test]
    fn test_parse_immediate_addressing_success() {
        let mut scanner = Scanner::new("#$44");
        let result = scanner.immediate_mode();

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            Some(Token::AddressMode(AddressMode::Immediate(0x44)))
        );

        let mut scanner = Scanner::new("#constant");
        let result = scanner.immediate_mode();

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            Some(Token::AddressMode(AddressMode::ImmediateIdent(String::from("constant"))))
        );
    }

    #[test]
    fn test_parse_immediate_addressing_non_immediate() {
        let mut scanner = Scanner::new("#$4321");
        let result = scanner.immediate_mode();

        assert!(result.is_err());
    }

    #[test]
    fn test_parse_immediate_addressing_fail_invalid_char() {
        let mut scanner = Scanner::new("#$4N");
        let result = scanner.immediate_mode();

        assert!(result.is_err());
    }

    #[test]
    fn test_parse_zero_page_addressing_success() {
        let mut scanner = Scanner::new("$44");
        let result = scanner.zero_page_mode();

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            Some(Token::AddressMode(AddressMode::ZeroPage(0x44)))
        )
    }

    #[test]
    fn test_parse_zero_page_addressing_non_zero_page() {
        let mut scanner = Scanner::new("$444");
        let result = scanner.zero_page_mode();

        assert!(result.is_err());
    }

    #[test]
    fn test_parse_zero_page_addressing_fail_invalid_char() {
        let mut scanner = Scanner::new("$444");
        let result = scanner.zero_page_mode();

        assert!(result.is_err());
    }

    #[test]
    fn test_parse_zero_page_x_addressing_success_no_spaces() {
        let mut scanner = Scanner::new("$44,X");
        let result = scanner.zero_page_x_mode();

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            Some(Token::AddressMode(AddressMode::ZeroPageX(0x44)))
        );
    }

    #[test]
    fn test_parse_zero_page_x_addressing_success_spaces1() {
        let mut scanner = Scanner::new("$44 ,X");
        let result = scanner.zero_page_x_mode();

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            Some(Token::AddressMode(AddressMode::ZeroPageX(0x44)))
        );
    }

    #[test]
    fn test_parse_zero_page_x_addressing_success_spaces2() {
        let mut scanner = Scanner::new("$44 , X");
        let result = scanner.zero_page_x_mode();

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            Some(Token::AddressMode(AddressMode::ZeroPageX(0x44)))
        );
    }

    #[test]
    fn test_parse_zero_page_x_addressing_success_non_zero_page_x() {
        let mut scanner = Scanner::new("$444,X");
        let result = scanner.zero_page_mode();

        assert!(result.is_err());
    }

    #[test]
    fn test_parse_zero_page_y_addressing_success_no_spaces() {
        let mut scanner = Scanner::new("$44,Y");
        let result = scanner.zero_page_y_mode();

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            Some(Token::AddressMode(AddressMode::ZeroPageY(0x44)))
        );
    }

    #[test]
    fn test_parse_zero_page_y_addressing_success_spaces1() {
        let mut scanner = Scanner::new("$44 ,Y");
        let result = scanner.zero_page_y_mode();

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            Some(Token::AddressMode(AddressMode::ZeroPageY(0x44)))
        );
    }

    #[test]
    fn test_parse_zero_page_y_addressing_success_spaces2() {
        let mut scanner = Scanner::new("$44 , Y");
        let result = scanner.zero_page_y_mode();

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            Some(Token::AddressMode(AddressMode::ZeroPageY(0x44)))
        );
    }

    #[test]
    fn test_parse_zero_page_y_addressing_success_non_zero_page_x() {
        let mut scanner = Scanner::new("$444,Y");
        let result = scanner.zero_page_y_mode();

        assert!(result.is_err());
    }

    #[test]
    fn test_parse_absolute_addressing_success() {
        let mut scanner = Scanner::new("$ABCD");
        let result = scanner.absolute_mode();

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            Some(Token::AddressMode(AddressMode::Absolute(0xABCD)))
        );
    }

    #[test]
    fn test_parse_absolute_x_addressing_success() {
        let mut scanner = Scanner::new("$ABCD,X");
        let result = scanner.absolute_x_mode();

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            Some(Token::AddressMode(AddressMode::AbsoluteX(0xABCD)))
        );
    }

    #[test]
    fn test_parse_absolute_y_addressing_success() {
        let mut scanner = Scanner::new("$ABCD,Y");
        let result = scanner.absolute_y_mode();

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            Some(Token::AddressMode(AddressMode::AbsoluteY(0xABCD)))
        );
    }

    #[test]
    fn test_parse_indexed_indirect_x_addressing_success() {
        let mut scanner = Scanner::new("($44,X)");
        let result = scanner.indexed_indirect_x_mode();

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            Some(Token::AddressMode(AddressMode::IndexedIndirectX(0x44)))
        );

        let mut scanner = Scanner::new("(CONSTANT,X)");
        let result = scanner.indexed_indirect_x_mode();

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            Some(Token::AddressMode(AddressMode::IndexedIndirectXIdent(
                String::from("CONSTANT")
            )))
        );
    }

    #[test]
    fn test_parse_indirect_index_y_address_success() {
        let mut scanner = Scanner::new("($44),y");
        let result = scanner.indirect_index_y_mode();

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            Some(Token::AddressMode(AddressMode::IndirectIndexY(0x44)))
        );

        let mut scanner = Scanner::new("(CONSTANT),Y");
        let result = scanner.indirect_index_y_mode();

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            Some(Token::AddressMode(AddressMode::IndirectIndexYIdent(
                String::from("CONSTANT")
            )))
        );
    }

    #[test]
    fn test_parse_accumulator_addressing_success() {
        let mut scanner = Scanner::new("A");
        let result = scanner._accumulator_mode();

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            Some(Token::AddressMode(AddressMode::Accumulator))
        );
    }
}
