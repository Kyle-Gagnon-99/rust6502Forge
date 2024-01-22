use std::{fmt, collections::HashMap};

use serde_derive::{Deserialize, Serialize};

use crate::{scoped_ref_to_string, label::LabelMetaData, error::ForgeError};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AddressMode {
    ZeroPageOrAbsoluteIdent(String),
    ZeroPageOrAbsoluteScopedRef(Vec<String>),
    ZeroPageOrAbsoluteXIdent(String),
    ZeroPageOrAbsoluteXScopedRef(Vec<String>),
    ZeroPageOrAbsoluteYIdent(String),
    ZeroPageOrAbsoluteYScopedRef(Vec<String>),
    Immediate(u8),
    ImmediateIdent(String),
    ImmediateScopedRef(Vec<String>),
    ZeroPage(u8),
    ZeroPageX(u8),
    ZeroPageY(u8),
    Absolute(u16),
    AbsoluteX(u16),
    AbsoluteY(u16),
    IndexedIndirectX(u8),
    IndexedIndirectXIdent(String),
    IndexedIndirectXScopedRef(Vec<String>),
    IndirectIndexY(u8),
    IndirectIndexYIdent(String),
    IndirectIndexYScopedRef(Vec<String>),
    Accumulator,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum AddressModeGeneric {
    Immediate,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    IndexedIndirectX,
    IndirectIndexY,
    Implied,
    Accumulator,
    Relative,
    Indirect
}

impl fmt::Display for AddressMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AddressMode::ZeroPageOrAbsoluteIdent(val) => {
                write!(f, "Zero / Absolute Address Mode: {}", val)
            }
            AddressMode::ZeroPageOrAbsoluteScopedRef(val) => {
                write!(
                    f,
                    "Zero / Absolute Address MOde: {}",
                    scoped_ref_to_string(val)
                )
            }
            AddressMode::ZeroPageOrAbsoluteXIdent(val) => {
                write!(f, "Zero / Absolute X Address Mode: {}", val)
            }
            AddressMode::ZeroPageOrAbsoluteXScopedRef(val) => {
                write!(
                    f,
                    "Zero / Absolute X Address Mode: {}",
                    scoped_ref_to_string(val)
                )
            }
            AddressMode::ZeroPageOrAbsoluteYIdent(val) => {
                write!(f, "Zero / Absolute Y Address Mode: {}", val)
            }
            AddressMode::ZeroPageOrAbsoluteYScopedRef(val) => {
                write!(
                    f,
                    "Zero / Absolute Y Address Mode: {}",
                    scoped_ref_to_string(val)
                )
            }
            AddressMode::Immediate(val) => {
                write!(f, "Immediate Address Mode: #${:02X}", val)
            }
            AddressMode::ImmediateIdent(val) => {
                write!(f, "Immediate Address Mode: #{}", val)
            }
            AddressMode::ImmediateScopedRef(val) => {
                write!(f, "Immediate Address Mode: #{}", scoped_ref_to_string(val))
            }
            AddressMode::ZeroPage(val) => {
                write!(f, "Zero Page Address Mode: ${:02X}", val)
            }
            AddressMode::ZeroPageX(val) => {
                write!(f, "Zero Page X Address Mode: ${:02X},X", val)
            }
            AddressMode::ZeroPageY(val) => {
                write!(f, "Zero Page Y Address Mode: ${:02X},Y", val)
            }
            AddressMode::Absolute(val) => {
                write!(f, "Absolute Address Mode: ${:04X}", val)
            }
            AddressMode::AbsoluteX(val) => {
                write!(f, "Absolute X Address Mode: ${:04X},X", val)
            }
            AddressMode::AbsoluteY(val) => {
                write!(f, "Absolute Y Address Mode: ${:04X},Y", val)
            }
            AddressMode::IndexedIndirectX(val) => {
                write!(f, "Indexed Indirect X Address Mode: (${:02X},X)", val)
            }
            AddressMode::IndexedIndirectXIdent(val) => {
                write!(f, "Indexed Indirect X Address Mode: ({},X)", val)
            }
            AddressMode::IndexedIndirectXScopedRef(val) => {
                write!(
                    f,
                    "Indexed Indirect X Address Mode: ({},X)",
                    scoped_ref_to_string(val)
                )
            }
            AddressMode::IndirectIndexY(val) => {
                write!(f, "Indirect Index Y Address Mode: (${:02X}),Y", val)
            }
            AddressMode::IndirectIndexYIdent(val) => {
                write!(f, "Indirect Index Y Address Mode: ({}),Y", val)
            }
            AddressMode::IndirectIndexYScopedRef(val) => {
                write!(
                    f,
                    "Indirect Index Y Address Mode: ({}),Y",
                    scoped_ref_to_string(val)
                )
            }
            AddressMode::Accumulator => {
                write!(f, "Accumulator Address Mode: A")
            }
        }
    }
}

impl AddressMode {
    pub fn to_generic(&self, label_map: &HashMap<String, LabelMetaData>, constant_map: &HashMap<String, u16>) -> Result<AddressModeGeneric, ForgeError> {
        let value = match self {
            AddressMode::Immediate(_) => AddressModeGeneric::Immediate,
            AddressMode::Accumulator => AddressModeGeneric::Accumulator,
            AddressMode::ZeroPage(_) => AddressModeGeneric::ZeroPage,
            AddressMode::ZeroPageX(_) => AddressModeGeneric::ZeroPageX,
            AddressMode::ZeroPageY(_) => AddressModeGeneric::ZeroPageY,
            AddressMode::Absolute(_) => AddressModeGeneric::Absolute,
            AddressMode::AbsoluteX(_) => AddressModeGeneric::AbsoluteX,
            AddressMode::AbsoluteY(_) => AddressModeGeneric::AbsoluteY,
            AddressMode::IndexedIndirectX(_) => AddressModeGeneric::IndexedIndirectX,
            AddressMode::IndirectIndexY(_) => AddressModeGeneric::IndirectIndexY,
            AddressMode::ImmediateIdent(_) => AddressModeGeneric::Immediate,
            AddressMode::ImmediateScopedRef(_) => AddressModeGeneric::Immediate,
            AddressMode::IndexedIndirectXIdent(_) => AddressModeGeneric::IndexedIndirectX,
            AddressMode::IndexedIndirectXScopedRef(_) => AddressModeGeneric::IndexedIndirectX,
            AddressMode::IndirectIndexYIdent(_) => AddressModeGeneric::IndirectIndexY,
            AddressMode::IndirectIndexYScopedRef(_) => AddressModeGeneric::IndirectIndexY,
            AddressMode::ZeroPageOrAbsoluteIdent(ident) => {
                if label_map.contains_key(ident) {
                    return Ok(AddressModeGeneric::Absolute)
                }

                if constant_map.contains_key(ident) {
                    let value = constant_map.get(ident).unwrap();
                    if value <= &0xFF {
                        return Ok(AddressModeGeneric::ZeroPage)
                    } else {
                        return Ok(AddressModeGeneric::Absolute)
                    }
                }

                return Err(ForgeError::LabelOrConstantNotFound{ label: ident.clone() })
            },
            AddressMode::ZeroPageOrAbsoluteScopedRef(_) => AddressModeGeneric::Absolute,
            AddressMode::ZeroPageOrAbsoluteXIdent(ident) => {
                if label_map.contains_key(ident) {
                    return Ok(AddressModeGeneric::AbsoluteX)
                }

                if constant_map.contains_key(ident) {
                    let value = constant_map.get(ident).unwrap();
                    if value <= &0xFF {
                        return Ok(AddressModeGeneric::ZeroPageX)
                    } else {
                        return Ok(AddressModeGeneric::AbsoluteX)
                    }
                }

                return Err(ForgeError::LabelOrConstantNotFound{ label: ident.clone() })
            }
            AddressMode::ZeroPageOrAbsoluteXScopedRef(_) => AddressModeGeneric::AbsoluteX,
            AddressMode::ZeroPageOrAbsoluteYIdent(ident) => {
                if label_map.contains_key(ident) {
                    return Ok(AddressModeGeneric::AbsoluteY)
                }

                if constant_map.contains_key(ident) {
                    let value = constant_map.get(ident).unwrap();
                    if value <= &0xFF {
                        return Ok(AddressModeGeneric::ZeroPageY)
                    } else {
                        return Ok(AddressModeGeneric::AbsoluteY)
                    }
                }

                return Err(ForgeError::LabelOrConstantNotFound { label: ident.clone() })
            }
            AddressMode::ZeroPageOrAbsoluteYScopedRef(_) => AddressModeGeneric::AbsoluteY,
        };

        Ok(value)
    }
}
