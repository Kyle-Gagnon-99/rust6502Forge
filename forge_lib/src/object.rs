use std::collections::HashMap;

use chrono::{DateTime, Utc};
use semver::Version;
use serde_derive::{Serialize, Deserialize};

use crate::{line::Line, label::LabelMetaData};

#[derive(Debug, Serialize, Deserialize)]
pub struct Header {
    pub magic_number: String,     // Bytes for rust6502forge
    pub timestamp: DateTime<Utc>, 
    pub version: Version,
    pub file_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Contents {
    pub label_map: HashMap<String, LabelMetaData>,
    pub constant_map: HashMap<String, u16>,
    pub parsed_contents: Vec<Line>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OutFile {
    pub header: Header,
    pub contents: Contents
}