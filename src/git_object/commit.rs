use super::{GitObject, Type};

pub struct Commit;
impl GitObject for Commit {
    fn get_type() -> Type {
        Type::Commit
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
