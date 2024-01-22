use serde_derive::{Serialize, Deserialize};

use crate::{address::AddressMode, expression::ExpressionNode};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Operand {
    Expression(ExpressionNode),
    AddressMode(AddressMode),
    LocalLabel(String),
}