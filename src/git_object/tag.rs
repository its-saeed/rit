use super::{GitObject, Type};

pub struct Tag;
impl GitObject for Tag {
    fn get_type() -> Type {
        Type::Tag
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
