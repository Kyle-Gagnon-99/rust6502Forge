use crate::error::ParseError;

use super::{Scanner, Token, TokenResult};

pub mod modes;

pub fn parse_hex16_with_position(s: &str, position: usize) -> Result<u16, ParseError> {
    u16::from_str_radix(s, 16).map_err(|_| ParseError::ParseIntError {
        msg: format!("failed to convert {} to a u16", s),
        position,
    })
}

pub fn parse_hex8_with_position(s: &str, position: usize) -> Result<u8, ParseError> {
    u8::from_str_radix(s, 16).map_err(|_| ParseError::ParseIntError {
        msg: format!("failed to convert {} to a u8", s),
        position,
    })
}

pub fn parse_bin16_with_position(s: &str, position: usize) -> Result<u16, ParseError> {
    u16::from_str_radix(s, 2).map_err(|_| ParseError::ParseIntError {
        msg: format!("failed to convert {} to a u16", s),
        position,
    })
}

impl Scanner {
    /// Parses a u16 hex address ($0000) if applicable. EBNF is defined as
    ///
    /// address_u16 = "$" hex_digit hex_digit hex_digit hex_digit
    ///
    /// See assembler.ebnf line 28
    pub fn address_u16(&mut self) -> TokenResult {
        let start_pos = self.cursor;
        let hex_size = 4;

        if !self.consume_char('$') {
            self.cursor = start_pos;
            return Ok(None);
        }

        // Try to collect exactly four hex digits
        let mut count = 0;
        while let Some(c) = self.peek() {
            if !c.is_ascii_hexdigit() {
                // At this point we determined that we should be parsing a u16 and found an invalid digit. If this ok in the future then ignore
                if count != hex_size {
                    return Err(ParseError::ParseIntError {
                        msg: format!("found invalid character: {}", c),
                        position: self.cursor,
                    });
                }
                break;
            }
            self.next();
            count += 1;

            // We can break early if we already collected four hex digits
            if count == hex_size {
                break;
            }
        }

        if let Some(c) = self.peek() {
            if c.is_ascii_hexdigit() {
                return Err(ParseError::TooManyDigits {
                    msg: format!("literal u8 has too many digits"),
                    position: self.cursor,
                });
            }
        }

        // Check to see if received four hex digits
        if count != hex_size {
            self.cursor = start_pos; // Reset the cursor
            return Ok(None);
        }

        // Collet the address into a string
        let address: String = self.input[(start_pos + 1)..self.cursor].iter().collect();
        match parse_hex16_with_position(&address, self.cursor) {
            Ok(val) => Ok(Some(Token::AddressU16(val))),
            Err(e) => Err(e),
        }
    }

    /// Parses a u8 hex address ($00) if applicable. EBNF is defined as
    ///
    /// address_u8 = "$" hex_digit hex_digit;
    ///
    /// See assembler.ebnf line 30
    pub fn address_u8(&mut self) -> TokenResult {
        let start_pos = self.cursor;
        let hex_size = 2;

        if !self.consume_char('$') {
            self.cursor = start_pos;
            return Ok(None);
        }

        // Try to collect exactly two hex digits
        let mut count = 0;
        while let Some(c) = self.peek() {
            if !c.is_ascii_hexdigit() {
                // At this point we determined that we should be parsing a u16 and found an invalid digit. If this ok in the future then ignore
                if count != hex_size {
                    return Err(ParseError::ParseIntError {
                        msg: format!("found invalid character: {}", c),
                        position: self.cursor,
                    });
                }
                break;
            }
            self.next();
            count += 1;

            // We can break early if we already collected two hex digits
            if count == hex_size {
                break;
            }
        }

        if let Some(c) = self.peek() {
            if c.is_ascii_hexdigit() {
                return Err(ParseError::TooManyDigits {
                    msg: format!("literal u8 has too many digits"),
                    position: self.cursor,
                });
            }
        }

        // Check to see if received two hex digits
        if count != hex_size {
            self.cursor = start_pos; // Reset the cursor
            return Ok(None);
        }

        // Collet the address into a string
        let address: String = self.input[(start_pos + 1)..self.cursor].iter().collect();
        match parse_hex8_with_position(&address, self.cursor) {
            Ok(val) => Ok(Some(Token::AddressU8(val))),
            Err(e) => Err(e),
        }
    }

    /// Parses a u8 hex literal (#$00) if applicable. EBNF is defined as
    ///
    /// literal_u8 = "#$" hex_digit hex_digit;
    ///
    /// See assembler.ebnf line 29
    pub fn literal_u8(&mut self) -> TokenResult {
        let start_pos = self.cursor;
        let hex_size = 2;

        if !(self.consume_char('#') && self.consume_char('$')) {
            self.cursor = start_pos;
            return Ok(None);
        }

        // Try to collect exactly two hex digits
        let mut count = 0;
        while let Some(c) = self.peek() {
            if !c.is_ascii_hexdigit() {
                // At this point we determined that we should be parsing a u16 and found an invalid digit. If this ok in the future then ignore
                if count != hex_size {
                    return Err(ParseError::ParseIntError {
                        msg: format!("found invalid character: {}", c),
                        position: self.cursor,
                    });
                }
                break;
            }
            self.next();
            count += 1;

            // We can break early if we already collected two hex digits
            if count == hex_size {
                break;
            }
        }

        if let Some(c) = self.peek() {
            if c.is_ascii_hexdigit() {
                return Err(ParseError::TooManyDigits {
                    msg: format!("literal u8 has too many digits"),
                    position: self.cursor,
                });
            }
        }

        // Check to see if received two hex digits
        if count != hex_size {
            self.cursor = start_pos; // Reset the cursor
            return Ok(None);
        }

        // Collet the address into a string
        let address: String = self.input[(start_pos + 2)..self.cursor].iter().collect();
        match parse_hex8_with_position(&address, self.cursor) {
            Ok(val) => Ok(Some(Token::LiteralU8(val))),
            Err(e) => Err(e),
        }
    }

    pub fn address_modes(&mut self) -> TokenResult {
        let start_pos = self.cursor;

        // Test indexed indirect X
        if let Some(token) = self.attempt_parser(Self::indexed_indirect_x_mode)? {
            return Ok(Some(token));
        }

        // Reset back
        self.cursor = start_pos;

        // Test indirect index y
        if let Some(token) = self.attempt_parser(Self::indirect_index_y_mode)? {
            return Ok(Some(token));
        }

        // Reset back
        self.cursor = start_pos;

        // Test zero page X addressing $00,X
        if let Some(token) = self.attempt_parser(Self::zero_page_x_mode)? {
            return Ok(Some(token));
        }

        // Reset back
        self.cursor = start_pos;

        // Test zero page Y addressing $00,Y
        if let Some(token) = self.attempt_parser(Self::zero_page_y_mode)? {
            return Ok(Some(token));
        }

        // Reset back
        self.cursor = start_pos;

        // Test zero page addressing $00
        if let Some(token) = self.attempt_parser(Self::zero_page_mode)? {
            return Ok(Some(token));
        }

        // Reset back
        self.cursor = start_pos;

        // Test immediate addressing #$00
        if let Some(token) = self.attempt_parser(Self::immediate_mode)? {
            return Ok(Some(token));
        }

        // Reset back
        self.cursor = start_pos;

        // Test absolute X
        if let Some(token) = self.attempt_parser(Self::absolute_x_mode)? {
            return Ok(Some(token));
        }

        // Reset back
        self.cursor = start_pos;

        // Test absolute Y
        if let Some(token) = self.attempt_parser(Self::absolute_y_mode)? {
            return Ok(Some(token));
        }

        // Reset back
        self.cursor = start_pos;

        // Test absolute
        if let Some(token) = self.attempt_parser(Self::absolute_mode)? {
            return Ok(Some(token));
        }

        // Reset back
        self.cursor = start_pos;

        // If all parsers have been tried then return Ok(None). For now this could be ok, let a parent parser
        // decide if not seeing an operand will be a total issue
        Ok(None)
    }
}

#[cfg(test)]
pub mod address_test {
    use forge_lib::address::AddressMode;

    use crate::scanner::{Scanner, Token};

    #[test]
    fn test_parse_addressu16_success_uppercase() {
        let mut scanner = Scanner::new("$FEDC");
        let result = scanner.address_u16();

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(Token::AddressU16(0xFEDC)))
    }

    #[test]
    fn test_parse_addressu16_success_lowercase() {
        let mut scanner = Scanner::new("$fedc");
        let result = scanner.address_u16();

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(Token::AddressU16(0xFEDC)))
    }

    #[test]
    fn test_parse_addressu16_non_address() {
        let mut scanner = Scanner::new("non-input");
        let result = scanner.address_u16();

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), None);
    }

    #[test]
    fn test_parse_addressu16_short_address() {
        let mut scanner = Scanner::new("$0fc");
        let result = scanner.address_u16();

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), None);
    }

    #[test]
    fn test_parse_addressu16_invalid_character() {
        let mut scanner = Scanner::new("$0fcz");
        let result = scanner.address_u16();

        assert!(result.is_err());
    }

    #[test]
    fn test_parse_addressu8_success_uppercase() {
        let mut scanner = Scanner::new("$FE");
        let result = scanner.address_u8();

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(Token::AddressU8(0xFE)))
    }

    #[test]
    fn test_parse_addressu8_success_lowercase() {
        let mut scanner = Scanner::new("$fe");
        let result = scanner.address_u8();

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(Token::AddressU8(0xFE)))
    }

    #[test]
    fn test_parse_addressu8_non_address() {
        let mut scanner = Scanner::new("non-input");
        let result = scanner.address_u8();

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), None);
    }

    #[test]
    fn test_parse_addressu8_short_address() {
        let mut scanner = Scanner::new("$0");
        let result = scanner.address_u8();

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), None);
    }

    #[test]
    fn test_parse_addressu8_invalid_character() {
        let mut scanner = Scanner::new("$0z");
        let result = scanner.address_u8();

        assert!(result.is_err());
    }

    #[test]
    fn test_parse_addressu8_no_spaces() {
        let mut scanner = Scanner::new("$44,X");
        let result = scanner.address_u8();

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(Token::AddressU8(0x44)));
        assert_eq!(scanner.cursor, 3);
    }

    #[test]
    fn test_parse_literalu8_success_uppercase() {
        let mut scanner = Scanner::new("#$F4");
        let result = scanner.literal_u8();

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(Token::LiteralU8(0xF4)));
    }

    #[test]
    fn test_parse_literalu8_success_lowercase() {
        let mut scanner = Scanner::new("#$f4");
        let result = scanner.literal_u8();

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(Token::LiteralU8(0xF4)));
    }

    #[test]
    fn test_parse_literalu8_success_non_input() {
        let mut scanner = Scanner::new("#non-input");
        let result = scanner.literal_u8();

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), None);
    }

    #[test]
    fn test_parse_literalu8_success_too_short() {
        let mut scanner = Scanner::new("#$0");
        let result = scanner.literal_u8();

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), None);
    }

    #[test]
    fn test_parse_literalu8_fail_invalid_char() {
        let mut scanner = Scanner::new("#$0Z");
        let result = scanner.literal_u8();

        assert!(result.is_err());
    }

    #[test]
    fn test_parse_address_mode_no_mode() {
        let mut scanner = Scanner::new("; Comment");
        let result = scanner.address_modes();

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), None);
    }

    #[test]
    fn test_parse_address_mode_immediate() {
        let mut scanner = Scanner::new("#$44");
        let result = scanner.address_modes();

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            Some(Token::AddressMode(AddressMode::Immediate(0x44)))
        )
    }

    #[test]
    fn test_parse_address_mode_zero_page() {
        let mut scanner = Scanner::new("$44");
        let result = scanner.address_modes();

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            Some(Token::AddressMode(AddressMode::ZeroPage(0x44)))
        );
    }

    #[test]
    fn test_parse_address_mode_zero_page_x() {
        let mut scanner = Scanner::new("$44,X");
        let result = scanner.address_modes();

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            Some(Token::AddressMode(AddressMode::ZeroPageX(0x44)))
        );
    }

    #[test]
    fn test_parse_address_mode_zero_page_y() {
        let mut scanner = Scanner::new("$44,Y");
        let result = scanner.address_modes();

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            Some(Token::AddressMode(AddressMode::ZeroPageY(0x44)))
        );
    }

    #[test]
    fn test_parse_address_mode_absolute() {
        let mut scanner = Scanner::new("$4400");
        let result = scanner.address_modes();

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            Some(Token::AddressMode(AddressMode::Absolute(0x4400)))
        );
    }

    #[test]
    fn test_parse_address_mode_absolute_x() {
        let mut scanner = Scanner::new("$4400,X");
        let result = scanner.address_modes();

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            Some(Token::AddressMode(AddressMode::AbsoluteX(0x4400)))
        );
    }

    #[test]
    fn test_parse_address_mode_absolute_y() {
        let mut scanner = Scanner::new("$4400,Y");
        let result = scanner.address_modes();

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            Some(Token::AddressMode(AddressMode::AbsoluteY(0x4400)))
        );
    }

    #[test]
    fn test_parse_address_mode_indexed_indirect_x() {
        let mut scanner = Scanner::new("($44,X)");
        let result = scanner.address_modes();

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            Some(Token::AddressMode(AddressMode::IndexedIndirectX(0x44)))
        );
    }

    #[test]
    fn test_parse_address_mode_indirect_index_y() {
        let mut scanner = Scanner::new("($44),Y");
        let result = scanner.address_modes();

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            Some(Token::AddressMode(AddressMode::IndirectIndexY(0x44)))
        );
    }
}
