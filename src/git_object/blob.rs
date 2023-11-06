use crate::error::ObjectParseError;

use super::{GitObject, Type};

pub struct Blob {
    pub blob: String,
}

impl GitObject for Blob {
    fn get_type() -> Type {
        Type::Blob
    }

    fn serialize(&self) -> String {
        // TODO: Make it memory-friendly
        self.blob.clone()
    }

    fn deserialize(
        buf_reader: &mut impl std::io::BufRead,
        object_header: super::Header,
    ) -> Result<Self, crate::error::ObjectParseError>
    where
        Self: Sized,
    {
        let mut blob = String::new();
        let length = buf_reader.read_to_string(&mut blob)?;
        if length != object_header.object_size {
            return Err(ObjectParseError::MismatchedObjectSize);
        }
        Ok(Blob { blob })
    }
}
