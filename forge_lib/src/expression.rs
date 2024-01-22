use std::collections::HashMap;

use serde_derive::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum HighPrecedenceOp {
    Mul,
    Div,
    ShiftLeft,
    ShiftRight,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LowPrecedenceOp {
    Add,
    Sub,
    Or,
    And,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ExpressionNode {
    BinOp(BinaryOp, Box<ExpressionNode>, Box<ExpressionNode>),
    Number(u16),
    Identifier(String),
    Parenthesized(Box<ExpressionNode>),
    ScopedReference(Vec<String>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BinaryOp {
    Add,
    Subtract,
    Or,
    And,
    Multiply,
    Divide,
    ShiftLeft,
    ShiftRight,
}

pub fn evaluate_expression(node: &ExpressionNode, constant_map: &HashMap<String, u16>) -> u16 {
    match node {
        ExpressionNode::BinOp(op, left, right) => {
            let l_val = evaluate_expression(left, constant_map);
            let r_val = evaluate_expression(right, constant_map);

            match op {
                BinaryOp::Add => l_val + r_val,
                BinaryOp::Subtract => l_val - r_val,
                BinaryOp::Multiply => l_val * r_val,
                BinaryOp::Divide => l_val / r_val,
                BinaryOp::And => l_val & r_val,
                BinaryOp::Or => l_val | r_val,
                BinaryOp::ShiftLeft => l_val << r_val,
                BinaryOp::ShiftRight => l_val >> r_val
            }
        },
        ExpressionNode::Number(n) => *n,
        ExpressionNode::Identifier(ident) => {
            constant_map.get(ident).cloned().unwrap()
        },
        ExpressionNode::Parenthesized(expr) => {
            evaluate_expression(&expr, constant_map)
        }
        ExpressionNode::ScopedReference(_scoped_ref) => {
            0
        }
    }
}