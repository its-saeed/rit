use crate::error::ObjectParseError;

#[derive(Debug)]
pub struct Blob {
    pub blob: String,
}

impl Blob {
    pub fn serialize(&self) -> String {
        // TODO: Make it memory-friendly
        self.blob.clone()
    }

    pub fn deserialize(
        buf_reader: &mut impl std::io::BufRead,
        object_header: super::Header,
    ) -> Result<Self, crate::error::ObjectParseError> {
        let mut blob = String::new();
        let length = buf_reader.read_to_string(&mut blob)?;
        if length != object_header.object_size {
            return Err(ObjectParseError::MismatchedObjectSize);
        }
        Ok(Blob { blob })
    }
}
