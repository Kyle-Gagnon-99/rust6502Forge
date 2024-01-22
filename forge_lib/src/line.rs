use serde_derive::{Serialize, Deserialize};

use crate::{instruction::Instruction, directive::Directive, operand::Operand, address::AddressMode};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Line {
    pub comment: Option<String>,
    pub constant: Option<(String, u16)>,
    pub label: Option<Labels>,
    pub main_component: Option<MainComponent>,
    pub newlines: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MainComponent {
    Instruction(Instruction),
    Directive(Directive)
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Labels {
    Label(String),
    LocalLabel(String),
}

impl Instruction {
    pub fn size(&self) -> u8 {
        // The Mnemonic will always take 1 byte
        let mut size = 1;

        // Now it is time to get what kind of addressing we have
        match &self.operand {
            Some(operand) => match operand {
                Operand::AddressMode(address_mode) => match address_mode {
                    AddressMode::ZeroPage(_)
                    | AddressMode::ZeroPageX(_)
                    | AddressMode::ZeroPageY(_)
                    | AddressMode::Immediate(_)
                    | AddressMode::IndexedIndirectX(_)
                    | AddressMode::IndirectIndexY(_) => {
                        size += 1;
                    }
                    // For now, the value of an expression or constant will always be assume to be in absolute addressing mode
                    _ => size += 2,
                },
                _ => size += 2,
            },
            None => {}
        }

        size
    }
}