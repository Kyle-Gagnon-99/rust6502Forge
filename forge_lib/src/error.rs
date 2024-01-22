use std::fmt::Display;

#[derive(Debug)]
pub enum ForgeError {
    NoSuchFileOrDir { file: String },
    LabelOrConstantNotFound { label: String },
}

impl Display for ForgeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoSuchFileOrDir { file } => {
                write!(f, "No such file or directory: {}", file)
            }
            Self::LabelOrConstantNotFound { label } => {
                write!(f, "Label or constant not found: {}", label)
            }
        }
    }
}