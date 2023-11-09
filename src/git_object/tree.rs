#[derive(Debug)]
pub struct Tree;
impl Tree {
    pub fn serialize(&self) -> String {
        todo!()
    }

    pub fn deserialize(
        _buf_reader: &mut impl std::io::BufRead,
        _object_header: super::Header,
    ) -> Result<Self, crate::error::ObjectParseError> {
        todo!()
    }
}
