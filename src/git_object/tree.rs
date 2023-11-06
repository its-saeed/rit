use super::{GitObject, Type};

pub struct Tree;
impl GitObject for Tree {
    fn get_type() -> Type {
        Type::Tree
    }

    fn serialize(&self) -> String {
        todo!()
    }

    fn deserialize(
        _buf_reader: &mut impl std::io::BufRead,
        _object_header: super::Header,
    ) -> Result<Self, crate::error::ObjectParseError>
    where
        Self: Sized,
    {
        todo!()
    }
}
