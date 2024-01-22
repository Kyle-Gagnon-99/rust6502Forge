use std::{collections::HashMap, path::PathBuf};

use chrono::Utc;
use forge_lib::{line::{Line, MainComponent, Labels}, object::{OutFile, Header, Contents}, write_object_file_to_contents, label::LabelMetaData, directive::{Directive, ByteArgs, WordArgs}, expression::evaluate_expression, operand::Operand, address::{AddressMode, AddressModeGeneric}, mnemonic::OPCODES_TO_BYTES};
use tracing::debug;

use crate::error::ParseError;

pub fn process_file(lines: &mut Vec<Line>, file_name: &PathBuf, out_file: &PathBuf) -> Result<(), ParseError> {
    let mut constant_map: HashMap<String, u16> = HashMap::new();
    let mut label_map: HashMap<String, LabelMetaData> = HashMap::new();
    let _starting_address: u16 = 0;
    let mut offset_tracker: u16 = 0;
    let _line_num: u16 = 1;

    // Go through and resolve all constants and labels
    for line in lines.iter() {
        resolve_labels_and_constants(
            line,
            &mut constant_map,
            &mut label_map,
            &mut offset_tracker,
        );

    }

    // Now serialize the out file
    let data = OutFile {
        header: Header {
            magic_number: String::from("rust6502forge"),
            timestamp: Utc::now(),
            version: semver::Version::new(0, 1, 0),
            file_name: file_name.to_str().unwrap().to_string()
        },
        contents: Contents {
            label_map,
            constant_map,
            parsed_contents: lines.to_vec()
        }
    };

    debug!("{:?}", data);

    write_object_file_to_contents(data, out_file);

    Ok(())
}

pub fn resolve_labels_and_constants(
    line: &Line,
    constant_map: &mut HashMap<String, u16>,
    label_map: &mut HashMap<String, LabelMetaData>,
    offset_tracker: &mut u16,
) {
    // Check if there is a constant
    let line = line.clone();
    if line.constant.is_some() {
        let (constant, value) = line.constant.unwrap();
        constant_map.insert(constant, value);
    }

    // If there is a label, then check where we are and insert it
    if line.label.is_some() {
        let (is_local, label) = match line.label.unwrap() {
            Labels::Label(label) => {
                (false, label)
            }
            Labels::LocalLabel(label) => {
                (true, label)
            }
        };

        label_map.insert(label.clone(), LabelMetaData { offset: offset_tracker.clone(), is_local });
    }

    // Now get the size of either the directive or instruction
    if line.main_component.is_some() {
        let main_component = line.main_component.unwrap();
        match main_component {
            MainComponent::Directive(directive) => {
                *offset_tracker += directive.size() as u16;
            }
            MainComponent::Instruction(instruction) => {
                *offset_tracker += instruction.size() as u16;
            }
        }
    }
}

pub fn process_lines(lines: &mut Vec<Line>) -> Vec<u8> {
    let mut constant_map: HashMap<String, u16> = HashMap::new();
    let mut label_map: HashMap<String, LabelMetaData> = HashMap::new();
    let mut starting_address: u16 = 0;
    let mut offset_tracker: u16 = 0;
    let _line_num: u16 = 1;

    // Go through and resolve all constants and labels
    for line in lines.iter() {
        resolve_labels_and_constants(
            line,
            &mut constant_map,
            &mut label_map,
            &mut offset_tracker,
        );

    }

    debug!("{:?}", label_map);

    for line in lines.iter_mut() {
        let _result = resolve_expressions(line, &mut constant_map, &mut label_map);
    }

    for line in lines.iter() {
        match line.main_component.clone() {
            Some(component) => {
                match component {
                    MainComponent::Directive(directive) => {

                    }
                    MainComponent::Instruction(instruction) => {
                        let gen_operand = if instruction.operand.is_some() {
                            match instruction.operand.unwrap() {
                                Operand::Expression(expression) => {
                                    let value = evaluate_expression(&expression, &constant_map);
                                    if value <= 0xFF {
                                        AddressModeGeneric::ZeroPage
                                    } else {
                                        AddressModeGeneric::Absolute
                                    }
                                }
                                Operand::LocalLabel(_) => {
                                    AddressModeGeneric::Absolute
                                }
                                Operand::AddressMode(addr_mode) => {
                                    addr_mode.to_generic(&label_map, &constant_map).unwrap()
                                }
                            }
                        } else {
                            AddressModeGeneric::Implied
                        };

                        let opcode = OPCODES_TO_BYTES.get(&(instruction.mnemonic, gen_operand.clone()));
                        debug!("({:?}, {:?}): {:?}", instruction.mnemonic, gen_operand, opcode);
                    }
                }
            }
            _ => {}
        }
    }

    vec![0x00]
}

pub fn resolve_expressions(line: &mut Line, constant_map: &mut HashMap<String, u16>, _label_map: &mut HashMap<String, LabelMetaData>) -> Result<(), ParseError> {
    // Expressions could be found at operands or directives
    if let Some(main_component) = &mut line.main_component {
        match main_component {
            MainComponent::Directive(directive) => {
                match directive {
                    Directive::BYTE(args_list) => {
                        for arg in args_list.iter_mut() {
                            let taken_arg = std::mem::take(arg);
                            match taken_arg {
                                ByteArgs::Expression(expression) => {
                                    debug!("Found an expression in a BYTE directive. Should update it");
                                    let value = evaluate_expression(&expression, constant_map);
                                    if value <= 0xFF {
                                        *arg = ByteArgs::Value(value as u8);
                                    } else {
                                        return Err(ParseError::ValueTooLarge)
                                    }
                                }
                                _ => {
                                    *arg = taken_arg;
                                }
                            };
                        }
                    }
                    Directive::WORD(args_list) => {
                        for arg in args_list {
                            let taken_arg = std::mem::take(arg);
                            match taken_arg {
                                WordArgs::Expression(expr) => {
                                    let value = evaluate_expression(&expr, constant_map);
                                    *arg = WordArgs::Value(value);
                                }
                                _ => {
                                    *arg = taken_arg;
                                }
                            };
                        }
                    }
                    _ => {}
                }
            },
            MainComponent::Instruction(instruction) => {
                if let Some(operand) = &instruction.operand {
                    match operand {
                        Operand::Expression(expression) => {
                            let value = evaluate_expression(&expression, constant_map);
                            // Here we could look at the value and determine whether or not to use absolute, for now, assume absolute
                            let address_mode = if value <= 0xFF {
                                AddressMode::ZeroPage(value as u8)
                            } else {
                                AddressMode::Absolute(value)
                            };

                            instruction.operand = Some(Operand::AddressMode(address_mode));
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    Ok(())
}