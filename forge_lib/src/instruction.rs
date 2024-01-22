use serde_derive::{Serialize, Deserialize};

use crate::{operand::Operand, mnemonic::Mnemonic};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Instruction {
    pub mnemonic: Mnemonic,
    pub operand: Option<Operand>,
}