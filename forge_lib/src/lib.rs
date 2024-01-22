use std::{
    fs::File,
    io::{Read, Write},
    path::PathBuf,
};

use error::ForgeError;
use object::OutFile;

pub mod address;
pub mod directive;
pub mod error;
pub mod expression;
pub mod instruction;
pub mod label;
pub mod line;
pub mod mnemonic;
pub mod object;
pub mod operand;
pub mod linker;

pub fn write_object_file_to_contents(data: OutFile, output_file: &PathBuf) {
    let encoded: Vec<u8> = bincode::serialize(&data).unwrap();

    let mut file = File::create(output_file).unwrap();
    file.write_all(&encoded).unwrap();
}

pub fn get_file_contents(input_file: &PathBuf) -> Result<OutFile, ForgeError> {
    let mut file = File::open(input_file).map_err(|_| ForgeError::NoSuchFileOrDir {
        file: input_file.to_string_lossy().into_owned(),
    })?;
    let mut encoded = Vec::new();
    file.read_to_end(&mut encoded).unwrap();

    let data: OutFile = bincode::deserialize(&encoded).unwrap();
    Ok(data)
}

pub fn scoped_ref_to_string(val: &Vec<String>) -> String {
    val.join("::")
}
