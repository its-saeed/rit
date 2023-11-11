use std::{fmt::Display, str::FromStr, usize};

use crate::error::TreeLeafParseError;

#[derive(Debug, Clone, Copy)]
pub enum Type {
    Tree = 4,
    RegularFile = 10,
    SymbolicLink = 12,
    Submodule = 16,
}

impl FromStr for Type {
    type Err = TreeLeafParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "04" | "4" => Ok(Self::Tree),
            "10" => Ok(Self::RegularFile),
            "12" => Ok(Self::SymbolicLink),
            "16" => Ok(Self::Submodule),
            _ => Err(TreeLeafParseError::InvalidFileMode),
        }
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Type::Tree => "tree",
            Type::RegularFile => "blob",
            Type::SymbolicLink => "blob",
            Type::Submodule => "commit",
        };

        write!(f, "{}", str)
    }
}

#[derive(Debug)]
pub struct Mode {
    pub type_: Type,
    file_permissions: String,
}

impl Mode {
    pub fn new(mode: String) -> Result<Self, TreeLeafParseError> {
        let mode = if mode.len() == 5 {
            format!("0{}", mode)
        } else {
            mode
        };
        let type_length: usize = if mode.len() == 5 { 1 } else { 2 };
        let type_str = &mode[0..type_length];
        let type_: Type = type_str.parse()?;
        Ok(Self {
            file_permissions: mode,
            type_,
        })
    }
}

impl Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.type_, self.file_permissions)
    }
}
