use std::collections::HashMap;

use lazy_static::lazy_static;
use serde_derive::{Serialize, Deserialize};
use strum_macros::EnumString;

use crate::expression::ExpressionNode;

#[derive(Debug, PartialEq, Clone, Copy, EnumString)]
pub enum DirectiveName {
    ORG,
    BYTE,
    WORD,
    SEGMENT,
    INCLUDE,
    PROC,
    ENDPROC,
    ENUM,
    ENDENUM,
    MACRO,
    ENDMACRO,
    SCOPE,
    ENDSCOPE,
    CODE,
    ADDR,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Directive {
    ORG(u16),
    BYTE(Vec<ByteArgs>),
    WORD(Vec<WordArgs>),
    SEGMENT(String),
    PROC(String),
    INCLUDE(String),
    ENDPROC,
    ENUM(String),
    ENDENUM,
    MACRO(String),
    ENDMACRO,
    SCOPE(String),
    ENDSCOPE
}


#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum ByteArgs {
    Value(u8),
    Identifier(String),
    Expression(ExpressionNode),
}

impl Default for ByteArgs {
    fn default() -> Self {
        ByteArgs::Value(0)
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum WordArgs {
    Value(u16),
    Identifier(String),
    Expression(ExpressionNode),
}

impl Default for WordArgs {
    fn default() -> Self {
        WordArgs::Value(0)
    }
}

lazy_static! {
    static ref DIRECTIVE_MAP: HashMap<&'static str, DirectiveName> = {
        let mut m = HashMap::new();
        m.insert("BYTE", DirectiveName::BYTE);
        m.insert("WORD", DirectiveName::WORD);
        m.insert("ORG", DirectiveName::ORG);
        m.insert("SEGMENT", DirectiveName::SEGMENT);
        m.insert("INCLUDE", DirectiveName::INCLUDE);
        m.insert("PROC", DirectiveName::PROC);
        m.insert("ENDPROC", DirectiveName::ENDPROC);
        m.insert("ENUM", DirectiveName::ENUM);
        m.insert("ENDENUM", DirectiveName::ENDENUM);
        m.insert("SCOPE", DirectiveName::SCOPE);
        m.insert("ENDSCOPE", DirectiveName::ENDSCOPE);
        m.insert("MACRO", DirectiveName::MACRO);
        m.insert("ENDMACRO", DirectiveName::ENDMACRO);
        m.insert("CODE", DirectiveName::CODE);
        m.insert("ADDR", DirectiveName::ADDR);
        m
    };
}

impl From<String> for DirectiveName {
    fn from(value: String) -> Self {
        if let Some(&directive) = DIRECTIVE_MAP.get(value.as_str()) {
            directive
        } else {
            panic!("Invalid directive: {}", value);
        }
    }
}

impl Directive {
    pub fn size(&self) -> u8 {
        match self {
            Directive::BYTE(args_list) => {
                args_list.len() as u8
            }
            Directive::WORD(args_list) => {
                (args_list.len() * 2) as u8
            }
            _ => {
                0
            }
        }
    }
}