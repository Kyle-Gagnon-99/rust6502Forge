use std::collections::HashMap;

use lazy_static::lazy_static;
use serde_derive::{Serialize, Deserialize};
use strum_macros::{EnumString, Display};

use crate::address::AddressModeGeneric;

lazy_static! {
    static ref MNEMONIC_MAP: HashMap<&'static str, Mnemonic> = {
        let mut m = HashMap::new();
        m.insert("ADC", Mnemonic::ADC);
        m.insert("AND", Mnemonic::AND);
        m.insert("ASL", Mnemonic::ASL);
        m.insert("BCC", Mnemonic::BCC);
        m.insert("BCS", Mnemonic::BCS);
        m.insert("BEQ", Mnemonic::BEQ);
        m.insert("BIT", Mnemonic::BIT);
        m.insert("BMI", Mnemonic::BMI);
        m.insert("BNE", Mnemonic::BNE);
        m.insert("BPL", Mnemonic::BPL);
        m.insert("BRK", Mnemonic::BRK);
        m.insert("BVC", Mnemonic::BVC);
        m.insert("BVS", Mnemonic::BVS);
        m.insert("CLC", Mnemonic::CLC);
        m.insert("CLD", Mnemonic::CLD);
        m.insert("CLI", Mnemonic::CLI);
        m.insert("CLV", Mnemonic::CLV);
        m.insert("CMP", Mnemonic::CMP);
        m.insert("CPX", Mnemonic::CPX);
        m.insert("CPY", Mnemonic::CPY);
        m.insert("DEC", Mnemonic::DEC);
        m.insert("DEX", Mnemonic::DEX);
        m.insert("DEY", Mnemonic::DEY);
        m.insert("EQR", Mnemonic::EQR);
        m.insert("INC", Mnemonic::INC);
        m.insert("INX", Mnemonic::INX);
        m.insert("INY", Mnemonic::INY);
        m.insert("JMP", Mnemonic::JMP);
        m.insert("JSR", Mnemonic::JSR);
        m.insert("LDA", Mnemonic::LDA);
        m.insert("LDX", Mnemonic::LDX);
        m.insert("LDY", Mnemonic::LDY);
        m.insert("LSR", Mnemonic::LSR);
        m.insert("NOP", Mnemonic::NOP);
        m.insert("ORA", Mnemonic::ORA);
        m.insert("PHA", Mnemonic::PHA);
        m.insert("PHP", Mnemonic::PHP);
        m.insert("PLA", Mnemonic::PLA);
        m.insert("PLP", Mnemonic::PLP);
        m.insert("ROL", Mnemonic::ROL);
        m.insert("ROR", Mnemonic::ROR);
        m.insert("RTI", Mnemonic::RTI);
        m.insert("RTS", Mnemonic::RTS);
        m.insert("SBC", Mnemonic::SBC);
        m.insert("SEC", Mnemonic::SEC);
        m.insert("SED", Mnemonic::SED);
        m.insert("SEI", Mnemonic::SEI);
        m.insert("STA", Mnemonic::STA);
        m.insert("STX", Mnemonic::STX);
        m.insert("STY", Mnemonic::STY);
        m.insert("TAX", Mnemonic::TAX);
        m.insert("TAY", Mnemonic::TAY);
        m.insert("TSX", Mnemonic::TSX);
        m.insert("TXA", Mnemonic::TXA);
        m.insert("TXS", Mnemonic::TXS);
        m.insert("TYA", Mnemonic::TYA);
        m
    };
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, EnumString, Display, Serialize, Deserialize, Hash)]
pub enum Mnemonic {
    ADC,
    AND,
    ASL,
    BCC,
    BCS,
    BEQ,
    BIT,
    BMI,
    BNE,
    BPL,
    BRK,
    BVC,
    BVS,
    CLC,
    CLD,
    CLI,
    CLV,
    CMP,
    CPX,
    CPY,
    DEC,
    DEX,
    DEY,
    EQR,
    INC,
    INX,
    INY,
    JMP,
    JSR,
    LDA,
    LDX,
    LDY,
    LSR,
    NOP,
    ORA,
    PHA,
    PHP,
    PLA,
    PLP,
    ROL,
    ROR,
    RTI,
    RTS,
    SBC,
    SEC,
    SED,
    SEI,
    STA,
    STX,
    STY,
    TAX,
    TAY,
    TSX,
    TXA,
    TXS,
    TYA
}

impl From<String> for Mnemonic {
    fn from(value: String) -> Self {
        if let Some(&mnemonic) = MNEMONIC_MAP.get(value.as_str()) {
            mnemonic
        } else {
            panic!("Invalid mnemonic: {}", value);
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct OpCode {
    pub opcode: u8,
    pub mnemonic: Mnemonic,
    pub len: u8,
    pub address_mode: AddressModeGeneric
}

impl OpCode {
    pub fn new(opcode: u8, mnemonic: Mnemonic, len: u8, address_mode: AddressModeGeneric) -> Self {
        Self {
            opcode,
            mnemonic,
            len,
            address_mode
        }
    }
}

lazy_static! {
    pub static ref OPCODES_TO_BYTES: HashMap<(Mnemonic, AddressModeGeneric), OpCode> = {
        let mut m = HashMap::new();

        // ADC
        m.insert((Mnemonic::ADC, AddressModeGeneric::Immediate), OpCode::new(0x69, Mnemonic::ADC, 2, AddressModeGeneric::Immediate));
        m.insert((Mnemonic::ADC, AddressModeGeneric::ZeroPage), OpCode::new(0x65, Mnemonic::ADC, 2, AddressModeGeneric::ZeroPage));
        m.insert((Mnemonic::ADC, AddressModeGeneric::ZeroPageX), OpCode::new(0x75, Mnemonic::ADC, 2, AddressModeGeneric::ZeroPageX));
        m.insert((Mnemonic::ADC, AddressModeGeneric::Absolute), OpCode::new(0x6D, Mnemonic::ADC, 3, AddressModeGeneric::Absolute));
        m.insert((Mnemonic::ADC, AddressModeGeneric::AbsoluteX), OpCode::new(0x7D, Mnemonic::ADC, 3, AddressModeGeneric::AbsoluteX));
        m.insert((Mnemonic::ADC, AddressModeGeneric::AbsoluteY), OpCode::new(0x79, Mnemonic::ADC, 3, AddressModeGeneric::AbsoluteY));
        m.insert((Mnemonic::ADC, AddressModeGeneric::IndexedIndirectX), OpCode::new(0x61, Mnemonic::ADC, 2, AddressModeGeneric::IndexedIndirectX));
        m.insert((Mnemonic::ADC, AddressModeGeneric::IndirectIndexY), OpCode::new(0x71, Mnemonic::ADC, 2, AddressModeGeneric::IndirectIndexY));

        // AND
        m.insert((Mnemonic::AND, AddressModeGeneric::Immediate), OpCode::new(0x29, Mnemonic::AND, 2, AddressModeGeneric::Immediate));
        m.insert((Mnemonic::AND, AddressModeGeneric::ZeroPage), OpCode::new(0x25, Mnemonic::AND, 2, AddressModeGeneric::ZeroPage));
        m.insert((Mnemonic::AND, AddressModeGeneric::ZeroPageX), OpCode::new(0x35, Mnemonic::AND, 2, AddressModeGeneric::ZeroPageX));
        m.insert((Mnemonic::AND, AddressModeGeneric::Absolute), OpCode::new(0x2D, Mnemonic::AND, 3, AddressModeGeneric::Absolute));
        m.insert((Mnemonic::AND, AddressModeGeneric::AbsoluteX), OpCode::new(0x3D, Mnemonic::AND, 3, AddressModeGeneric::AbsoluteX));
        m.insert((Mnemonic::AND, AddressModeGeneric::AbsoluteY), OpCode::new(0x39, Mnemonic::AND, 3, AddressModeGeneric::AbsoluteY));
        m.insert((Mnemonic::AND, AddressModeGeneric::IndexedIndirectX), OpCode::new(0x21, Mnemonic::AND, 2, AddressModeGeneric::IndexedIndirectX));
        m.insert((Mnemonic::AND, AddressModeGeneric::IndirectIndexY), OpCode::new(0x31, Mnemonic::AND, 2, AddressModeGeneric::IndirectIndexY));

        // ASL
        m.insert((Mnemonic::ASL, AddressModeGeneric::Accumulator), OpCode::new(0x0A, Mnemonic::ASL, 1, AddressModeGeneric::Accumulator));
        m.insert((Mnemonic::ASL, AddressModeGeneric::ZeroPage), OpCode::new(0x06, Mnemonic::ASL, 2, AddressModeGeneric::ZeroPage));
        m.insert((Mnemonic::ASL, AddressModeGeneric::ZeroPageX), OpCode::new(0x16, Mnemonic::ASL, 2, AddressModeGeneric::ZeroPageX));
        m.insert((Mnemonic::ASL, AddressModeGeneric::Absolute), OpCode::new(0x0E, Mnemonic::ASL, 3, AddressModeGeneric::Absolute));
        m.insert((Mnemonic::ASL, AddressModeGeneric::AbsoluteX), OpCode::new(0x1E, Mnemonic::ASL, 3, AddressModeGeneric::AbsoluteX));

        // BCC
        m.insert((Mnemonic::BCC, AddressModeGeneric::Relative), OpCode::new(0x90, Mnemonic::BCC, 2, AddressModeGeneric::Relative));

        // BCS
        m.insert((Mnemonic::BCS, AddressModeGeneric::Relative), OpCode::new(0xB0, Mnemonic::BCS, 2, AddressModeGeneric::Relative));

        // BEQ
        m.insert((Mnemonic::BEQ, AddressModeGeneric::Relative), OpCode::new(0xF0, Mnemonic::BEQ, 2, AddressModeGeneric::Relative));

        // BIT
        m.insert((Mnemonic::BIT, AddressModeGeneric::ZeroPage), OpCode::new(0x24, Mnemonic::BIT, 2, AddressModeGeneric::ZeroPage));
        m.insert((Mnemonic::BIT, AddressModeGeneric::Absolute), OpCode::new(0x2C, Mnemonic::BIT, 3, AddressModeGeneric::Absolute));

        // BMI
        m.insert((Mnemonic::BMI, AddressModeGeneric::Relative), OpCode::new(0x30, Mnemonic::BMI, 2, AddressModeGeneric::Relative));

        // BNE
        m.insert((Mnemonic::BNE, AddressModeGeneric::Relative), OpCode::new(0xD0, Mnemonic::BNE, 2, AddressModeGeneric::Relative));

        // BPL
        m.insert((Mnemonic::BPL, AddressModeGeneric::Relative), OpCode::new(0x10, Mnemonic::BPL, 2, AddressModeGeneric::Relative));

        // BRK
        m.insert((Mnemonic::BRK, AddressModeGeneric::Implied), OpCode::new(0x00, Mnemonic::BRK, 1, AddressModeGeneric::Implied));

        // BVC
        m.insert((Mnemonic::BVC, AddressModeGeneric::Relative), OpCode::new(0x50, Mnemonic::BVC, 2, AddressModeGeneric::Relative));

        // BVS
        m.insert((Mnemonic::BVS, AddressModeGeneric::Relative), OpCode::new(0x70, Mnemonic::BVS, 2, AddressModeGeneric::Relative));

        // CLC
        m.insert((Mnemonic::CLC, AddressModeGeneric::Implied), OpCode::new(0x18, Mnemonic::CLC, 1, AddressModeGeneric::Implied));

        // CLD
        m.insert((Mnemonic::CLD, AddressModeGeneric::Implied), OpCode::new(0xD8, Mnemonic::CLD, 1, AddressModeGeneric::Implied));

        // CLI
        m.insert((Mnemonic::CLI, AddressModeGeneric::Implied), OpCode::new(0x58, Mnemonic::CLI, 1, AddressModeGeneric::Implied));

        // CLV
        m.insert((Mnemonic::CLV, AddressModeGeneric::Implied), OpCode::new(0xB8, Mnemonic::CLV, 1, AddressModeGeneric::Implied));

        // CMP
        m.insert((Mnemonic::CMP, AddressModeGeneric::Immediate), OpCode::new(0xC9, Mnemonic::CMP, 2, AddressModeGeneric::Immediate));
        m.insert((Mnemonic::CMP, AddressModeGeneric::ZeroPage), OpCode::new(0xC5, Mnemonic::CMP, 2, AddressModeGeneric::ZeroPage));
        m.insert((Mnemonic::CMP, AddressModeGeneric::ZeroPageX), OpCode::new(0xD5, Mnemonic::CMP, 2, AddressModeGeneric::ZeroPageX));
        m.insert((Mnemonic::CMP, AddressModeGeneric::Absolute), OpCode::new(0xCD, Mnemonic::CMP, 3, AddressModeGeneric::Absolute));
        m.insert((Mnemonic::CMP, AddressModeGeneric::AbsoluteX), OpCode::new(0xDD, Mnemonic::CMP, 3, AddressModeGeneric::AbsoluteX));
        m.insert((Mnemonic::CMP, AddressModeGeneric::AbsoluteY), OpCode::new(0xD9, Mnemonic::CMP, 3, AddressModeGeneric::AbsoluteY));
        m.insert((Mnemonic::CMP, AddressModeGeneric::IndexedIndirectX), OpCode::new(0xC1, Mnemonic::CMP, 2, AddressModeGeneric::IndexedIndirectX));
        m.insert((Mnemonic::CMP, AddressModeGeneric::IndirectIndexY), OpCode::new(0xD1, Mnemonic::CMP, 2, AddressModeGeneric::IndirectIndexY));

        // CPX
        m.insert((Mnemonic::CPX, AddressModeGeneric::Immediate), OpCode::new(0xE0, Mnemonic::CPX, 2, AddressModeGeneric::Immediate));
        m.insert((Mnemonic::CPX, AddressModeGeneric::ZeroPage), OpCode::new(0xE4, Mnemonic::CPX, 2, AddressModeGeneric::ZeroPage));
        m.insert((Mnemonic::CPX, AddressModeGeneric::Absolute), OpCode::new(0xEC, Mnemonic::CPX, 3, AddressModeGeneric::Absolute));

        // CPY
        m.insert((Mnemonic::CPY, AddressModeGeneric::Immediate), OpCode::new(0xC0, Mnemonic::CPY, 2, AddressModeGeneric::Immediate));
        m.insert((Mnemonic::CPY, AddressModeGeneric::ZeroPage), OpCode::new(0xC4, Mnemonic::CPY, 2, AddressModeGeneric::ZeroPage));
        m.insert((Mnemonic::CPY, AddressModeGeneric::Absolute), OpCode::new(0xCC, Mnemonic::CPY, 3, AddressModeGeneric::Absolute));

        // DEC
        m.insert((Mnemonic::DEC, AddressModeGeneric::ZeroPage), OpCode::new(0xC6, Mnemonic::DEC, 2, AddressModeGeneric::ZeroPage));
        m.insert((Mnemonic::DEC, AddressModeGeneric::ZeroPageX), OpCode::new(0xD6, Mnemonic::DEC, 2, AddressModeGeneric::ZeroPageX));
        m.insert((Mnemonic::DEC, AddressModeGeneric::Absolute), OpCode::new(0xCE, Mnemonic::DEC, 3, AddressModeGeneric::Absolute));
        m.insert((Mnemonic::DEC, AddressModeGeneric::AbsoluteX), OpCode::new(0xDE, Mnemonic::DEC, 3, AddressModeGeneric::AbsoluteX));

        // DEX
        m.insert((Mnemonic::DEX, AddressModeGeneric::Implied), OpCode::new(0xCA, Mnemonic::DEX, 1, AddressModeGeneric::Implied));

        // DEY
        m.insert((Mnemonic::DEY, AddressModeGeneric::Implied), OpCode::new(0x88, Mnemonic::DEY, 1, AddressModeGeneric::Implied));

        // EOR
        m.insert((Mnemonic::EQR, AddressModeGeneric::Immediate), OpCode::new(0x49, Mnemonic::EQR, 2, AddressModeGeneric::Immediate));
        m.insert((Mnemonic::EQR, AddressModeGeneric::ZeroPage), OpCode::new(0x45, Mnemonic::EQR, 2, AddressModeGeneric::ZeroPage));
        m.insert((Mnemonic::EQR, AddressModeGeneric::ZeroPageX), OpCode::new(0x55, Mnemonic::EQR, 2, AddressModeGeneric::ZeroPageX));
        m.insert((Mnemonic::EQR, AddressModeGeneric::Absolute), OpCode::new(0x4D, Mnemonic::EQR, 3, AddressModeGeneric::Absolute));
        m.insert((Mnemonic::EQR, AddressModeGeneric::AbsoluteX), OpCode::new(0x5D, Mnemonic::EQR, 3, AddressModeGeneric::AbsoluteX));
        m.insert((Mnemonic::EQR, AddressModeGeneric::AbsoluteY), OpCode::new(0x59, Mnemonic::EQR, 3, AddressModeGeneric::AbsoluteY));
        m.insert((Mnemonic::EQR, AddressModeGeneric::IndexedIndirectX), OpCode::new(0x41, Mnemonic::EQR, 2, AddressModeGeneric::IndexedIndirectX));
        m.insert((Mnemonic::EQR, AddressModeGeneric::IndirectIndexY), OpCode::new(0x51, Mnemonic::EQR, 2, AddressModeGeneric::IndirectIndexY));

        // INC
        m.insert((Mnemonic::INC, AddressModeGeneric::ZeroPage), OpCode::new(0xE6, Mnemonic::INC, 2, AddressModeGeneric::ZeroPage));
        m.insert((Mnemonic::INC, AddressModeGeneric::ZeroPageX), OpCode::new(0xF6, Mnemonic::INC, 2, AddressModeGeneric::ZeroPageX));
        m.insert((Mnemonic::INC, AddressModeGeneric::Absolute), OpCode::new(0xEE, Mnemonic::INC, 3, AddressModeGeneric::Absolute));
        m.insert((Mnemonic::INC, AddressModeGeneric::AbsoluteX), OpCode::new(0xFE, Mnemonic::INC, 3, AddressModeGeneric::AbsoluteX));

        // INX
        m.insert((Mnemonic::INX, AddressModeGeneric::Implied), OpCode::new(0xE8, Mnemonic::INX, 1, AddressModeGeneric::Implied));

        // INY
        m.insert((Mnemonic::INY, AddressModeGeneric::Implied), OpCode::new(0xC8, Mnemonic::INY, 1, AddressModeGeneric::Implied));

        // JMP
        m.insert((Mnemonic::JMP, AddressModeGeneric::Absolute), OpCode::new(0x4C, Mnemonic::JMP, 3, AddressModeGeneric::Absolute));
        m.insert((Mnemonic::JMP, AddressModeGeneric::Indirect), OpCode::new(0x6C, Mnemonic::JMP, 3, AddressModeGeneric::Indirect));

        // JSR
        m.insert((Mnemonic::JSR, AddressModeGeneric::Absolute), OpCode::new(0x20, Mnemonic::JSR, 3, AddressModeGeneric::Absolute));

        // LDA
        m.insert((Mnemonic::LDA, AddressModeGeneric::Immediate), OpCode::new(0xA9, Mnemonic::LDA, 2, AddressModeGeneric::Immediate));
        m.insert((Mnemonic::LDA, AddressModeGeneric::ZeroPage), OpCode::new(0xA5, Mnemonic::LDA, 2, AddressModeGeneric::ZeroPage));
        m.insert((Mnemonic::LDA, AddressModeGeneric::ZeroPageX), OpCode::new(0xB5, Mnemonic::LDA, 2, AddressModeGeneric::ZeroPageX));
        m.insert((Mnemonic::LDA, AddressModeGeneric::Absolute), OpCode::new(0xAD, Mnemonic::LDA, 3, AddressModeGeneric::Absolute));
        m.insert((Mnemonic::LDA, AddressModeGeneric::AbsoluteX), OpCode::new(0xBD, Mnemonic::LDA, 3, AddressModeGeneric::AbsoluteX));
        m.insert((Mnemonic::LDA, AddressModeGeneric::AbsoluteY), OpCode::new(0xB9, Mnemonic::LDA, 3, AddressModeGeneric::AbsoluteY));
        m.insert((Mnemonic::LDA, AddressModeGeneric::IndexedIndirectX), OpCode::new(0xA1, Mnemonic::LDA, 2, AddressModeGeneric::IndexedIndirectX));
        m.insert((Mnemonic::LDA, AddressModeGeneric::IndirectIndexY), OpCode::new(0xB1, Mnemonic::LDA, 2, AddressModeGeneric::IndirectIndexY));

        // LDX
        m.insert((Mnemonic::LDX, AddressModeGeneric::Immediate), OpCode::new(0xA2, Mnemonic::LDX, 2, AddressModeGeneric::Immediate));
        m.insert((Mnemonic::LDX, AddressModeGeneric::ZeroPage), OpCode::new(0xA6, Mnemonic::LDX, 2, AddressModeGeneric::ZeroPage));
        m.insert((Mnemonic::LDX, AddressModeGeneric::ZeroPageY), OpCode::new(0xB6, Mnemonic::LDX, 2, AddressModeGeneric::ZeroPageY));
        m.insert((Mnemonic::LDX, AddressModeGeneric::Absolute), OpCode::new(0xAE, Mnemonic::LDX, 3, AddressModeGeneric::Absolute));
        m.insert((Mnemonic::LDX, AddressModeGeneric::AbsoluteY), OpCode::new(0xBE, Mnemonic::LDX, 3, AddressModeGeneric::AbsoluteY));

        // LDY
        m.insert((Mnemonic::LDY, AddressModeGeneric::Immediate), OpCode::new(0xA0, Mnemonic::LDY, 2, AddressModeGeneric::Immediate));
        m.insert((Mnemonic::LDY, AddressModeGeneric::ZeroPage), OpCode::new(0xA4, Mnemonic::LDY, 2, AddressModeGeneric::ZeroPage));
        m.insert((Mnemonic::LDY, AddressModeGeneric::ZeroPageX), OpCode::new(0xB4, Mnemonic::LDY, 2, AddressModeGeneric::ZeroPageX));
        m.insert((Mnemonic::LDY, AddressModeGeneric::Absolute), OpCode::new(0xAC, Mnemonic::LDY, 3, AddressModeGeneric::Absolute));
        m.insert((Mnemonic::LDY, AddressModeGeneric::AbsoluteX), OpCode::new(0xBC, Mnemonic::LDY, 3, AddressModeGeneric::AbsoluteX));

        // LSR
        m.insert((Mnemonic::LSR, AddressModeGeneric::Accumulator), OpCode::new(0x4A, Mnemonic::LSR, 1, AddressModeGeneric::Accumulator));
        m.insert((Mnemonic::LSR, AddressModeGeneric::ZeroPage), OpCode::new(0x46, Mnemonic::LSR, 2, AddressModeGeneric::ZeroPage));
        m.insert((Mnemonic::LSR, AddressModeGeneric::ZeroPageX), OpCode::new(0x56, Mnemonic::LSR, 2, AddressModeGeneric::ZeroPageX));
        m.insert((Mnemonic::LSR, AddressModeGeneric::Absolute), OpCode::new(0x4E, Mnemonic::LSR, 3, AddressModeGeneric::Absolute));
        m.insert((Mnemonic::LSR, AddressModeGeneric::AbsoluteX), OpCode::new(0x5E, Mnemonic::LSR, 3, AddressModeGeneric::AbsoluteX));

        // NOP
        m.insert((Mnemonic::NOP, AddressModeGeneric::Implied), OpCode::new(0xEA, Mnemonic::NOP, 1, AddressModeGeneric::Implied));
        
        // ORA
        m.insert((Mnemonic::ORA, AddressModeGeneric::Immediate), OpCode::new(0x09, Mnemonic::ORA, 2, AddressModeGeneric::Immediate));
        m.insert((Mnemonic::ORA, AddressModeGeneric::ZeroPage), OpCode::new(0x05, Mnemonic::ORA, 2, AddressModeGeneric::ZeroPage));
        m.insert((Mnemonic::ORA, AddressModeGeneric::ZeroPageX), OpCode::new(0x15, Mnemonic::ORA, 2, AddressModeGeneric::ZeroPageX));
        m.insert((Mnemonic::ORA, AddressModeGeneric::Absolute), OpCode::new(0x0D, Mnemonic::ORA, 3, AddressModeGeneric::Absolute));
        m.insert((Mnemonic::ORA, AddressModeGeneric::AbsoluteX), OpCode::new(0x1D, Mnemonic::ORA, 3, AddressModeGeneric::AbsoluteX));
        m.insert((Mnemonic::ORA, AddressModeGeneric::AbsoluteY), OpCode::new(0x19, Mnemonic::ORA, 3, AddressModeGeneric::AbsoluteY));
        m.insert((Mnemonic::ORA, AddressModeGeneric::IndexedIndirectX), OpCode::new(0x01, Mnemonic::ORA, 2, AddressModeGeneric::IndexedIndirectX));
        m.insert((Mnemonic::ORA, AddressModeGeneric::IndirectIndexY), OpCode::new(0x11, Mnemonic::ORA, 2, AddressModeGeneric::IndirectIndexY));

        // PHA
        m.insert((Mnemonic::PHA, AddressModeGeneric::Implied), OpCode::new(0x48, Mnemonic::PHA, 1, AddressModeGeneric::Implied));

        // PHP
        m.insert((Mnemonic::PHP, AddressModeGeneric::Implied), OpCode::new(0x08, Mnemonic::PHP, 1, AddressModeGeneric::Implied));

        // PLA
        m.insert((Mnemonic::PLA, AddressModeGeneric::Implied), OpCode::new(0x68, Mnemonic::PLA, 1, AddressModeGeneric::Implied));

        // PLP
        m.insert((Mnemonic::PLP, AddressModeGeneric::Implied), OpCode::new(0x28, Mnemonic::PLP, 1, AddressModeGeneric::Implied));

        // ROL
        m.insert((Mnemonic::ROL, AddressModeGeneric::Accumulator), OpCode::new(0x2A, Mnemonic::ROL, 1, AddressModeGeneric::Accumulator));
        m.insert((Mnemonic::ROL, AddressModeGeneric::ZeroPage), OpCode::new(0x26, Mnemonic::ROL, 2, AddressModeGeneric::ZeroPage));
        m.insert((Mnemonic::ROL, AddressModeGeneric::ZeroPageX), OpCode::new(0x36, Mnemonic::ROL, 2, AddressModeGeneric::ZeroPageX));
        m.insert((Mnemonic::ROL, AddressModeGeneric::Absolute), OpCode::new(0x2E, Mnemonic::ROL, 3, AddressModeGeneric::Absolute));
        m.insert((Mnemonic::ROL, AddressModeGeneric::AbsoluteX), OpCode::new(0x3E, Mnemonic::ROL, 3, AddressModeGeneric::AbsoluteX));

        // ROR
        m.insert((Mnemonic::ROR, AddressModeGeneric::Accumulator), OpCode::new(0x6A, Mnemonic::ROR, 1, AddressModeGeneric::Accumulator));
        m.insert((Mnemonic::ROR, AddressModeGeneric::ZeroPage), OpCode::new(0x66, Mnemonic::ROR, 2, AddressModeGeneric::ZeroPage));
        m.insert((Mnemonic::ROR, AddressModeGeneric::ZeroPageX), OpCode::new(0x76, Mnemonic::ROR, 2, AddressModeGeneric::ZeroPageX));
        m.insert((Mnemonic::ROR, AddressModeGeneric::Absolute), OpCode::new(0x6E, Mnemonic::ROR, 3, AddressModeGeneric::Absolute));
        m.insert((Mnemonic::ROR, AddressModeGeneric::AbsoluteX), OpCode::new(0x7E, Mnemonic::ROR, 3, AddressModeGeneric::AbsoluteX));

        // RTI
        m.insert((Mnemonic::RTI, AddressModeGeneric::Implied), OpCode::new(0x40, Mnemonic::RTI, 1, AddressModeGeneric::Implied));

        // RTS
        m.insert((Mnemonic::RTS, AddressModeGeneric::Implied), OpCode::new(0x60, Mnemonic::RTS, 1, AddressModeGeneric::Implied));

        // SBC
        m.insert((Mnemonic::SBC, AddressModeGeneric::Immediate), OpCode::new(0xE9, Mnemonic::SBC, 2, AddressModeGeneric::Immediate));
        m.insert((Mnemonic::SBC, AddressModeGeneric::ZeroPage), OpCode::new(0xE5, Mnemonic::SBC, 2, AddressModeGeneric::ZeroPage));
        m.insert((Mnemonic::SBC, AddressModeGeneric::ZeroPageX), OpCode::new(0xF5, Mnemonic::SBC, 2, AddressModeGeneric::ZeroPageX));
        m.insert((Mnemonic::SBC, AddressModeGeneric::Absolute), OpCode::new(0xED, Mnemonic::SBC, 3, AddressModeGeneric::Absolute));
        m.insert((Mnemonic::SBC, AddressModeGeneric::AbsoluteX), OpCode::new(0xFD, Mnemonic::SBC, 3, AddressModeGeneric::AbsoluteX));
        m.insert((Mnemonic::SBC, AddressModeGeneric::AbsoluteY), OpCode::new(0xF9, Mnemonic::SBC, 3, AddressModeGeneric::AbsoluteY));
        m.insert((Mnemonic::SBC, AddressModeGeneric::IndexedIndirectX), OpCode::new(0xE1, Mnemonic::SBC, 2, AddressModeGeneric::IndexedIndirectX));
        m.insert((Mnemonic::SBC, AddressModeGeneric::IndirectIndexY), OpCode::new(0xF1, Mnemonic::SBC, 2, AddressModeGeneric::IndirectIndexY));

        // SEC
        m.insert((Mnemonic::SEC, AddressModeGeneric::Implied), OpCode::new(0x38, Mnemonic::SEC, 1, AddressModeGeneric::Implied));

        // SED
        m.insert((Mnemonic::SED, AddressModeGeneric::Implied), OpCode::new(0xF8, Mnemonic::SED, 1, AddressModeGeneric::Implied));

        // SEI
        m.insert((Mnemonic::SEI, AddressModeGeneric::Implied), OpCode::new(0x78, Mnemonic::SEI, 1, AddressModeGeneric::Implied));

        // STA
        m.insert((Mnemonic::STA, AddressModeGeneric::ZeroPage), OpCode::new(0x85, Mnemonic::STA, 2, AddressModeGeneric::ZeroPage));
        m.insert((Mnemonic::STA, AddressModeGeneric::ZeroPageX), OpCode::new(0x95, Mnemonic::STA, 2, AddressModeGeneric::ZeroPageX));
        m.insert((Mnemonic::STA, AddressModeGeneric::Absolute), OpCode::new(0x8D, Mnemonic::STA, 3, AddressModeGeneric::Absolute));
        m.insert((Mnemonic::STA, AddressModeGeneric::AbsoluteX), OpCode::new(0x9D, Mnemonic::STA, 3, AddressModeGeneric::AbsoluteX));
        m.insert((Mnemonic::STA, AddressModeGeneric::AbsoluteY), OpCode::new(0x99, Mnemonic::STA, 3, AddressModeGeneric::AbsoluteY));
        m.insert((Mnemonic::STA, AddressModeGeneric::IndexedIndirectX), OpCode::new(0x81, Mnemonic::STA, 2, AddressModeGeneric::IndexedIndirectX));
        m.insert((Mnemonic::STA, AddressModeGeneric::IndirectIndexY), OpCode::new(0x91, Mnemonic::STA, 2, AddressModeGeneric::IndirectIndexY));

        // STX
        m.insert((Mnemonic::STX, AddressModeGeneric::ZeroPage), OpCode::new(0x86, Mnemonic::STX, 2, AddressModeGeneric::ZeroPage));
        m.insert((Mnemonic::STX, AddressModeGeneric::ZeroPageY), OpCode::new(0x96, Mnemonic::STX, 2, AddressModeGeneric::ZeroPageY));
        m.insert((Mnemonic::STX, AddressModeGeneric::Absolute), OpCode::new(0x8E, Mnemonic::STX, 3, AddressModeGeneric::Absolute));

        // STY
        m.insert((Mnemonic::STY, AddressModeGeneric::ZeroPage), OpCode::new(0x84, Mnemonic::STY, 2, AddressModeGeneric::ZeroPage));
        m.insert((Mnemonic::STY, AddressModeGeneric::ZeroPageX), OpCode::new(0x94, Mnemonic::STY, 2, AddressModeGeneric::ZeroPageX));
        m.insert((Mnemonic::STY, AddressModeGeneric::Absolute), OpCode::new(0x8C, Mnemonic::STY, 3, AddressModeGeneric::Absolute));

        // TAX
        m.insert((Mnemonic::TAX, AddressModeGeneric::Implied), OpCode::new(0xAA, Mnemonic::TAX, 1, AddressModeGeneric::Implied));

        // TAY
        m.insert((Mnemonic::TAY, AddressModeGeneric::Implied), OpCode::new(0xA8, Mnemonic::TAY, 1, AddressModeGeneric::Implied));

        // TSX
        m.insert((Mnemonic::TSX, AddressModeGeneric::Implied), OpCode::new(0xBA, Mnemonic::TSX, 1, AddressModeGeneric::Implied));

        // TXA
        m.insert((Mnemonic::TXA, AddressModeGeneric::Implied), OpCode::new(0x8A, Mnemonic::TXA, 1, AddressModeGeneric::Implied));

        // TXS
        m.insert((Mnemonic::TXS, AddressModeGeneric::Implied), OpCode::new(0x9A, Mnemonic::TXS, 1, AddressModeGeneric::Implied));

        // TYA
        m.insert((Mnemonic::TYA, AddressModeGeneric::Implied), OpCode::new(0x98, Mnemonic::TYA, 1, AddressModeGeneric::Implied));

        m
    };
}