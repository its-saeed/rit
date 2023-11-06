use flate2::{bufread::ZlibDecoder, read::ZlibEncoder, Compression};
use std::io::{BufRead, Read};

use crate::error::ObjectParseError;

use super::SerializedGitObject;

pub struct CompressedGitObject {
    pub compressed: Vec<u8>,
}

impl CompressedGitObject {
    pub fn decompress(buf_reader: impl BufRead) -> Result<SerializedGitObject, ObjectParseError> {
        let mut zlib = ZlibDecoder::new(buf_reader);
        let mut buffer = String::new();
        zlib.read_to_string(&mut buffer)?;
        Ok(SerializedGitObject::new(buffer))
    }
}

impl TryFrom<&SerializedGitObject> for CompressedGitObject {
    type Error = std::io::Error;

    fn try_from(object: &SerializedGitObject) -> Result<Self, Self::Error> {
        let mut z = ZlibEncoder::new(object.as_ref(), Compression::fast());
        let mut buffer = Vec::new();
        z.read_to_end(&mut buffer)?;
        Ok(Self { compressed: buffer })
    }
}

impl AsRef<[u8]> for CompressedGitObject {
    fn as_ref(&self) -> &[u8] {
        &self.compressed
    }
}
