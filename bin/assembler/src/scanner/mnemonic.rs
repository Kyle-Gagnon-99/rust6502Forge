use std::collections::HashSet;

use crate::error::ParseError;

use super::{Scanner, Token, TokenResult};

impl Scanner {
    pub fn mnemonic(&mut self) -> TokenResult {
        let mnemonics: HashSet<&str> = [
            "ADC", "AND", "ASL", "BCC", "BCS", "BEQ", "BIT", "BMI", "BNE", "BPL", "BRK", "BVC",
            "BVS", "CLC", "CLD", "CLI", "CLV", "CMP", "CPX", "CPY", "DEC", "DEX", "DEY", "EQR",
            "INC", "INX", "INY", "JMP", "JSR", "LDA", "LDX", "LDY", "LSR", "NOP", "ORA", "PHA",
            "PHP", "PLA", "PLP", "ROL", "ROR", "RTI", "RTS", "SBC", "SEC", "SED", "SEI", "STA",
            "STX", "STY", "TAX", "TAY", "TSX", "TXA", "TXS", "TYA"
        ].iter().cloned().collect();

        let start_pos = self.cursor;

        while let Some(c) = self.peek() {
            if !(c.is_ascii_uppercase() || c.is_ascii_lowercase()) {
                break;
            }
            self.next();
        }

        let mnemonic: String = self.input[start_pos..self.cursor].iter().collect();
        let mnemonic = mnemonic.to_ascii_uppercase();

        if mnemonics.contains(mnemonic.as_str()) {
            Ok(Some(Token::Mnemonic(mnemonic.into())))
        } else {
            self.cursor = start_pos;
            Err(ParseError::ExpectedValidMnemonic)
        }
    }
}

#[cfg(test)]
mod mnemonic_tests {
    use forge_lib::mnemonic::Mnemonic;

    use super::*;

    #[test]
    fn test_parse_mnemonic_success() {
        let mut scanner = Scanner::new("ADC");
        assert_eq!(scanner.mnemonic().unwrap(), Some(Token::Mnemonic(Mnemonic::ADC)));

        let mut scanner = Scanner::new("AND");
        assert_eq!(scanner.mnemonic().unwrap(), Some(Token::Mnemonic(Mnemonic::AND)));

        let mut scanner = Scanner::new("ASL");
        assert_eq!(scanner.mnemonic().unwrap(), Some(Token::Mnemonic(Mnemonic::ASL)));

        let mut scanner = Scanner::new("BCC");
        assert_eq!(scanner.mnemonic().unwrap(), Some(Token::Mnemonic(Mnemonic::BCC)));

        let mut scanner = Scanner::new("BCS");
        assert_eq!(scanner.mnemonic().unwrap(), Some(Token::Mnemonic(Mnemonic::BCS)));

        let mut scanner = Scanner::new("BEQ");
        assert_eq!(scanner.mnemonic().unwrap(), Some(Token::Mnemonic(Mnemonic::BEQ)));

        let mut scanner = Scanner::new("BIT");
        assert_eq!(scanner.mnemonic().unwrap(), Some(Token::Mnemonic(Mnemonic::BIT)));

        let mut scanner = Scanner::new("BMI");
        assert_eq!(scanner.mnemonic().unwrap(), Some(Token::Mnemonic(Mnemonic::BMI)));

        let mut scanner = Scanner::new("BNE");
        assert_eq!(scanner.mnemonic().unwrap(), Some(Token::Mnemonic(Mnemonic::BNE)));

        let mut scanner = Scanner::new("BPL");
        assert_eq!(scanner.mnemonic().unwrap(), Some(Token::Mnemonic(Mnemonic::BPL)));

        let mut scanner = Scanner::new("BRK");
        assert_eq!(scanner.mnemonic().unwrap(), Some(Token::Mnemonic(Mnemonic::BRK)));

        let mut scanner = Scanner::new("BVC");
        assert_eq!(scanner.mnemonic().unwrap(), Some(Token::Mnemonic(Mnemonic::BVC)));

        let mut scanner = Scanner::new("BVS");
        assert_eq!(scanner.mnemonic().unwrap(), Some(Token::Mnemonic(Mnemonic::BVS)));

        let mut scanner = Scanner::new("CLC");
        assert_eq!(scanner.mnemonic().unwrap(), Some(Token::Mnemonic(Mnemonic::CLC)));

        let mut scanner = Scanner::new("CLD");
        assert_eq!(scanner.mnemonic().unwrap(), Some(Token::Mnemonic(Mnemonic::CLD)));

        let mut scanner = Scanner::new("CLI");
        assert_eq!(scanner.mnemonic().unwrap(), Some(Token::Mnemonic(Mnemonic::CLI)));

        let mut scanner = Scanner::new("CLV");
        assert_eq!(scanner.mnemonic().unwrap(), Some(Token::Mnemonic(Mnemonic::CLV)));

        let mut scanner = Scanner::new("CMP");
        assert_eq!(scanner.mnemonic().unwrap(), Some(Token::Mnemonic(Mnemonic::CMP)));

        let mut scanner = Scanner::new("CPX");
        assert_eq!(scanner.mnemonic().unwrap(), Some(Token::Mnemonic(Mnemonic::CPX)));

        let mut scanner = Scanner::new("CPY");
        assert_eq!(scanner.mnemonic().unwrap(), Some(Token::Mnemonic(Mnemonic::CPY)));

        let mut scanner = Scanner::new("DEC");
        assert_eq!(scanner.mnemonic().unwrap(), Some(Token::Mnemonic(Mnemonic::DEC)));

        let mut scanner = Scanner::new("DEX");
        assert_eq!(scanner.mnemonic().unwrap(), Some(Token::Mnemonic(Mnemonic::DEX)));

        let mut scanner = Scanner::new("DEY");
        assert_eq!(scanner.mnemonic().unwrap(), Some(Token::Mnemonic(Mnemonic::DEY)));

        let mut scanner = Scanner::new("EQR");
        assert_eq!(scanner.mnemonic().unwrap(), Some(Token::Mnemonic(Mnemonic::EQR)));

        let mut scanner = Scanner::new("INC");
        assert_eq!(scanner.mnemonic().unwrap(), Some(Token::Mnemonic(Mnemonic::INC)));

        let mut scanner = Scanner::new("INX");
        assert_eq!(scanner.mnemonic().unwrap(), Some(Token::Mnemonic(Mnemonic::INX)));

        let mut scanner = Scanner::new("INY");
        assert_eq!(scanner.mnemonic().unwrap(), Some(Token::Mnemonic(Mnemonic::INY)));

        let mut scanner = Scanner::new("JMP");
        assert_eq!(scanner.mnemonic().unwrap(), Some(Token::Mnemonic(Mnemonic::JMP)));

        let mut scanner = Scanner::new("JSR");
        assert_eq!(scanner.mnemonic().unwrap(), Some(Token::Mnemonic(Mnemonic::JSR)));

        let mut scanner = Scanner::new("LDA");
        assert_eq!(scanner.mnemonic().unwrap(), Some(Token::Mnemonic(Mnemonic::LDA)));

        let mut scanner = Scanner::new("LDX");
        assert_eq!(scanner.mnemonic().unwrap(), Some(Token::Mnemonic(Mnemonic::LDX)));

        let mut scanner = Scanner::new("LDY");
        assert_eq!(scanner.mnemonic().unwrap(), Some(Token::Mnemonic(Mnemonic::LDY)));

        let mut scanner = Scanner::new("LSR");
        assert_eq!(scanner.mnemonic().unwrap(), Some(Token::Mnemonic(Mnemonic::LSR)));

        let mut scanner = Scanner::new("NOP");
        assert_eq!(scanner.mnemonic().unwrap(), Some(Token::Mnemonic(Mnemonic::NOP)));

        let mut scanner = Scanner::new("ORA");
        assert_eq!(scanner.mnemonic().unwrap(), Some(Token::Mnemonic(Mnemonic::ORA)));

        let mut scanner = Scanner::new("PHA");
        assert_eq!(scanner.mnemonic().unwrap(), Some(Token::Mnemonic(Mnemonic::PHA)));

        let mut scanner = Scanner::new("PHP");
        assert_eq!(scanner.mnemonic().unwrap(), Some(Token::Mnemonic(Mnemonic::PHP)));

        let mut scanner = Scanner::new("PLA");
        assert_eq!(scanner.mnemonic().unwrap(), Some(Token::Mnemonic(Mnemonic::PLA)));

        let mut scanner = Scanner::new("PLP");
        assert_eq!(scanner.mnemonic().unwrap(), Some(Token::Mnemonic(Mnemonic::PLP)));

        let mut scanner = Scanner::new("ROL");
        assert_eq!(scanner.mnemonic().unwrap(), Some(Token::Mnemonic(Mnemonic::ROL)));

        let mut scanner = Scanner::new("ROR");
        assert_eq!(scanner.mnemonic().unwrap(), Some(Token::Mnemonic(Mnemonic::ROR)));

        let mut scanner = Scanner::new("RTI");
        assert_eq!(scanner.mnemonic().unwrap(), Some(Token::Mnemonic(Mnemonic::RTI)));

        let mut scanner = Scanner::new("RTS");
        assert_eq!(scanner.mnemonic().unwrap(), Some(Token::Mnemonic(Mnemonic::RTS)));

        let mut scanner = Scanner::new("SBC");
        assert_eq!(scanner.mnemonic().unwrap(), Some(Token::Mnemonic(Mnemonic::SBC)));

        let mut scanner = Scanner::new("SEC");
        assert_eq!(scanner.mnemonic().unwrap(), Some(Token::Mnemonic(Mnemonic::SEC)));

        let mut scanner = Scanner::new("SED");
        assert_eq!(scanner.mnemonic().unwrap(), Some(Token::Mnemonic(Mnemonic::SED)));

        let mut scanner = Scanner::new("SEI");
        assert_eq!(scanner.mnemonic().unwrap(), Some(Token::Mnemonic(Mnemonic::SEI)));

        let mut scanner = Scanner::new("STA");
        assert_eq!(scanner.mnemonic().unwrap(), Some(Token::Mnemonic(Mnemonic::STA)));

        let mut scanner = Scanner::new("STX");
        assert_eq!(scanner.mnemonic().unwrap(), Some(Token::Mnemonic(Mnemonic::STX)));

        let mut scanner = Scanner::new("STY");
        assert_eq!(scanner.mnemonic().unwrap(), Some(Token::Mnemonic(Mnemonic::STY)));

        let mut scanner = Scanner::new("TAX");
        assert_eq!(scanner.mnemonic().unwrap(), Some(Token::Mnemonic(Mnemonic::TAX)));

        let mut scanner = Scanner::new("TAY");
        assert_eq!(scanner.mnemonic().unwrap(), Some(Token::Mnemonic(Mnemonic::TAY)));

        let mut scanner = Scanner::new("TSX");
        assert_eq!(scanner.mnemonic().unwrap(), Some(Token::Mnemonic(Mnemonic::TSX)));

        let mut scanner = Scanner::new("TXA");
        assert_eq!(scanner.mnemonic().unwrap(), Some(Token::Mnemonic(Mnemonic::TXA)));

        let mut scanner = Scanner::new("TXS");
        assert_eq!(scanner.mnemonic().unwrap(), Some(Token::Mnemonic(Mnemonic::TXS)));

        let mut scanner = Scanner::new("TYA");
        assert_eq!(scanner.mnemonic().unwrap(), Some(Token::Mnemonic(Mnemonic::TYA)));

        let mut scanner = Scanner::new("tya");
        assert_eq!(scanner.mnemonic().unwrap(), Some(Token::Mnemonic(Mnemonic::TYA)));
        
    }

    #[test]
    fn test_parse_mnemonic_fail_invalid_chars_start() {
        let mut scanner = Scanner::new("1ADC");
        assert!(scanner.mnemonic().is_err());
    }

    #[test]
    fn test_parse_mnemonic_fail_invalid_mnemonic() {
        let mut scanner = Scanner::new("noo");
        assert!(scanner.mnemonic().is_err());
    }
}