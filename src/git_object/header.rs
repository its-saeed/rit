use std::{fmt::Display, str::FromStr};

use anyhow::Context;

use crate::error::ObjectParseError;

#[derive(PartialEq, PartialOrd, Debug, Clone, Copy)]
pub enum Type {
    Commit,
    Tree,
    Tag,
    Blob,
}

impl FromStr for Type {
    type Err = ObjectParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "commit" => Ok(Type::Commit),
            "tree" => Ok(Type::Tree),
            "tag" => Ok(Type::Tag),
            "blob" => Ok(Type::Blob),
            _ => Err(ObjectParseError::InvalidObjectType),
        }
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = match self {
            Type::Commit => "commit",
            Type::Tree => "tree",
            Type::Tag => "tag",
            Type::Blob => "blob",
        };

        write!(f, "{}", string)
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Header {
    pub object_type: Type,
    pub object_size: usize,
}

impl Header {
    pub fn new(object_type: Type, object_size: usize) -> Self {
        Self {
            object_type,
            object_size,
        }
    }

    pub fn load(buf_reader: &mut impl std::io::BufRead) -> Result<Self, ObjectParseError> {
        let mut buffer = Vec::new();
        let length = buf_reader
            .read_until(b' ', &mut buffer)
            .context("Failed to read object type")?;
        let object_type = String::from_utf8_lossy(&buffer[..length - 1]);
        let object_type: Type = object_type.parse()?;

        buffer = Vec::new();
        let length = buf_reader.read_until(b'\x00', &mut buffer)?;
        let object_size = String::from_utf8_lossy(&buffer[..length - 1]);
        let object_size = object_size.parse()?;

        Ok(Header {
            object_type,
            object_size,
        })
    }
}

impl Display for Header {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}\x00", self.object_type, self.object_size,)
    }
}

#[cfg(test)]
mod tests {
    use super::Type;

    use super::Header;

    #[test]
    fn parse_object_file_header_should_read_correct_header() -> Result<(), anyhow::Error> {
        // 00000000  63 6f 6d 6d 69 74 20 31  30 38 36 00 74 72 65 65  |commit 1086.tree|
        let object_header = hex::decode("636f6d6d697420313038360074726565").unwrap();
        let object_header = Header::load(&mut object_header.as_ref())?;
        assert_eq!(object_header.object_type, Type::Commit);
        assert_eq!(object_header.object_size, 1086);

        Ok(())
    }

    #[test]
    fn to_string_of_header_should_serialize_it_correctly() -> Result<(), anyhow::Error> {
        let header = Header::new(Type::Tag, 1000);

        let serialized = format!("{}", header);
        let loaded = Header::load(&mut serialized.as_bytes()).unwrap();
        assert_eq!(header, loaded);

        Ok(())
    }
}
