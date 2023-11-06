use anyhow::Context;
use std::io::{BufRead, BufWriter, Write};

use crate::{
    error::{ObjectCreateError, ObjectParseError},
    GitObject,
};

use super::{Blob, Commit, Header, Tag, Tree, Type};

pub struct SerializedGitObject {
    raw: String,
    pub hash: String,
}

impl AsRef<[u8]> for SerializedGitObject {
    fn as_ref(&self) -> &[u8] {
        self.raw.as_ref()
    }
}

impl SerializedGitObject {
    pub fn new(raw: String) -> Self {
        Self {
            hash: sha1_smol::Sha1::from(&raw).hexdigest(),
            raw,
        }
    }

    pub fn serialize(
        mut buf_reader: impl BufRead,
        object_type: Type,
    ) -> Result<SerializedGitObject, ObjectCreateError> {
        let mut buffer = String::new();
        buf_reader.read_to_string(&mut buffer)?;
        let serialized = match object_type {
            Type::Commit => todo!(),
            Type::Tree => todo!(),
            Type::Tag => todo!(),
            Type::Blob => {
                let object = Blob { blob: buffer };
                object.serialize()
            }
        };

        let buffer = Vec::<u8>::new();
        let mut buf_writer = BufWriter::new(buffer);

        write!(
            buf_writer,
            "{}{}",
            Header::new(object_type, serialized.len()),
            serialized
        )?;

        buf_writer.flush()?;
        let buffer = buf_writer
            .into_inner()
            .context("Failed to take buffer out of buf writer")?;

        Ok(SerializedGitObject::new(String::from_utf8(buffer)?))
    }
}

impl TryInto<Box<dyn GitObject>> for SerializedGitObject {
    type Error = ObjectParseError;

    fn try_into(self) -> Result<Box<dyn GitObject>, Self::Error> {
        let mut buffer = self.raw.as_bytes();
        let object_header = Header::load(&mut buffer)?;

        match object_header.object_type {
            Type::Commit => Ok(Box::new(Commit::deserialize(&mut buffer, object_header)?)),
            Type::Tree => Ok(Box::new(Tree::deserialize(&mut buffer, object_header)?)),
            Type::Tag => Ok(Box::new(Tag::deserialize(&mut buffer, object_header)?)),
            Type::Blob => Ok(Box::new(Blob::deserialize(&mut buffer, object_header)?)),
        }
    }
}
