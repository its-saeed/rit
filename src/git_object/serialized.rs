use anyhow::Context;
use std::io::{BufWriter, Write};

use crate::{
    error::{ObjectCreateError, ObjectParseError},
    git_object, GitObject,
};

use super::Header;

pub struct SerializedGitObject {
    raw: Vec<u8>,
    pub hash: String,
}

impl AsRef<[u8]> for SerializedGitObject {
    fn as_ref(&self) -> &[u8] {
        self.raw.as_ref()
    }
}

impl SerializedGitObject {
    pub fn new(raw: Vec<u8>) -> Self {
        Self {
            hash: sha1_smol::Sha1::from(&raw).hexdigest(),
            raw,
        }
    }
}

impl TryFrom<GitObject> for SerializedGitObject {
    type Error = ObjectCreateError;

    fn try_from(value: GitObject) -> Result<Self, Self::Error> {
        let (serialized_object, object_type) = match value {
            GitObject::Commit(commit) => (commit.serialize(), git_object::Type::Commit),
            GitObject::Blob(blob) => (blob.serialize(), git_object::Type::Blob),
            GitObject::Tag(tag) => (tag.serialize(), git_object::Type::Tag),
            GitObject::Tree(tree) => (tree.serialize(), git_object::Type::Tree),
        };

        let buffer = Vec::<u8>::new();
        let mut buf_writer = BufWriter::new(buffer);

        write!(
            buf_writer,
            "{}{}",
            Header::new(object_type, serialized_object.len()),
            serialized_object
        )?;

        buf_writer.flush()?;
        let buffer = buf_writer
            .into_inner()
            .context("Failed to take buffer out of buf writer")?;

        Ok(SerializedGitObject::new(buffer))
    }
}

impl TryInto<GitObject> for SerializedGitObject {
    type Error = ObjectParseError;

    fn try_into(self) -> Result<GitObject, Self::Error> {
        let mut buffer = self.raw.as_ref();
        let object_header = Header::load(&mut buffer)?;
        GitObject::deserialize(&mut buffer, object_header)
    }
}
