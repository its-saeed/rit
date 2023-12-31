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
            GitObject::Tag(tag) => tag.serialize(),
            GitObject::Tree(tree) => tree.serialize(),
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
            Type::Tree => Ok(Self::Tree(Tree::deserialize(buf_reader, object_header)?)),
            Type::Tag => Ok(Self::Tag(Tag::deserialize(buf_reader, object_header)?)),
            Type::Blob => Ok(Self::Blob(Blob::deserialize(buf_reader, object_header)?)),
        }
    }
}
