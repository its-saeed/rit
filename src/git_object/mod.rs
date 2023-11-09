pub mod blob;
pub mod commit;
pub mod compressed;
pub mod header;
pub mod serialized;
pub mod tag;
pub mod tree;

pub use blob::*;
pub use commit::*;
pub use compressed::*;
pub use header::*;
pub use serialized::*;
pub use tag::*;
pub use tree::*;

use crate::error::ObjectParseError;

#[derive(Debug)]
pub enum GitObject {
    Commit(Commit),
    Blob(Blob),
    Tag(Tag),
    Tree(Tree),
}

impl GitObject {
    pub fn serialize(&self) -> String {
        match self {
            GitObject::Commit(commit) => commit.serialize(),
            GitObject::Blob(blob) => blob.serialize(),
            GitObject::Tag(_) => todo!(),
            GitObject::Tree(_) => todo!(),
        }
    }

    fn deserialize(
        buf_reader: &mut impl std::io::BufRead,
        object_header: Header,
    ) -> Result<Self, ObjectParseError> {
        match object_header.object_type {
            Type::Commit => Ok(Self::Commit(Commit::deserialize(
                buf_reader,
                object_header,
            )?)),
            Type::Tree => todo!(),
            Type::Tag => todo!(),
            Type::Blob => Ok(Self::Blob(Blob::deserialize(buf_reader, object_header)?)),
        }
    }
}
