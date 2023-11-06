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

pub trait GitObject {
    fn get_type() -> Type
    where
        Self: Sized;

    fn serialize(&self) -> String;

    fn deserialize(
        buf_reader: &mut impl std::io::BufRead,
        object_header: Header,
    ) -> Result<Self, ObjectParseError>
    where
        Self: Sized;
}
