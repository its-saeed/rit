use std::{
    fs::{self},
    io::{BufReader, Read},
    str::FromStr,
};

use anyhow::Context;
use flate2::bufread::ZlibDecoder;

use crate::{error::ObjectParseError, repository::GitRepository};

#[derive(PartialEq, PartialOrd, Debug, Clone, Copy)]
pub enum GitObjectType {
    Commit,
    Tree,
    Tag,
    Blob,
}

impl FromStr for GitObjectType {
    type Err = ObjectParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "commit" => Ok(GitObjectType::Commit),
            "tree" => Ok(GitObjectType::Tree),
            "tag" => Ok(GitObjectType::Tag),
            "blob" => Ok(GitObjectType::Blob),
            _ => Err(ObjectParseError::InvalidObjectType),
        }
    }
}

#[derive(Debug)]
struct GitObjectHeader {
    object_type: GitObjectType,
    object_size: usize,
}

pub trait GitObject {
    fn get_type() -> GitObjectType
    where
        Self: Sized;

    fn serialize(&self) -> String;
}

pub struct TreeObject;
impl GitObject for TreeObject {
    fn get_type() -> GitObjectType {
        GitObjectType::Tree
    }

    fn serialize(&self) -> String {
        todo!()
    }
}

pub struct CommitObject;
impl GitObject for CommitObject {
    fn get_type() -> GitObjectType {
        GitObjectType::Commit
    }

    fn serialize(&self) -> String {
        todo!()
    }
}

pub struct TagObject;
impl GitObject for TagObject {
    fn get_type() -> GitObjectType {
        GitObjectType::Tag
    }

    fn serialize(&self) -> String {
        todo!()
    }
}

pub struct BlobObject {
    pub blob: String,
}

impl GitObject for BlobObject {
    fn get_type() -> GitObjectType {
        GitObjectType::Blob
    }

    fn serialize(&self) -> String {
        // TODO: Make it memory-friendly
        self.blob.clone()
    }
}

pub fn read(repo: &GitRepository, sha: String) -> Result<Box<dyn GitObject>, ObjectParseError> {
    let real_file_path = repo.directory_manager.sha_to_file_path(&sha);

    let file = fs::File::open(real_file_path)?;
    let mut buf_reader = BufReader::new(file);
    let mut zlib = ZlibDecoder::new(&mut buf_reader);
    let mut buffer = String::new();
    zlib.read_to_string(&mut buffer)?;
    let mut buffer = buffer.as_bytes();
    let object_header = parse_object_file_header(&mut buffer)?;

    match object_header.object_type {
        GitObjectType::Commit => Ok(Box::new(read_commit_object(&mut buffer, object_header)?)),
        GitObjectType::Tree => Ok(Box::new(read_tree_object(&mut buffer, object_header)?)),
        GitObjectType::Tag => Ok(Box::new(read_tag_object(&mut buffer, object_header)?)),
        GitObjectType::Blob => Ok(Box::new(read_blob_object(&mut buffer, object_header)?)),
    }
}

fn read_blob_object(
    buf_reader: &mut impl std::io::BufRead,
    object_header: GitObjectHeader,
) -> Result<BlobObject, ObjectParseError> {
    let mut blob = String::new();
    let length = buf_reader.read_to_string(&mut blob)?;
    if length != object_header.object_size {
        return Err(ObjectParseError::MismatchedObjectSize);
    }
    Ok(BlobObject { blob })
}

fn read_commit_object(
    _buf_reader: &mut impl std::io::BufRead,
    _object_header: GitObjectHeader,
) -> Result<CommitObject, ObjectParseError> {
    todo!()
}

fn read_tree_object(
    _buf_reader: &mut impl std::io::BufRead,
    _object_header: GitObjectHeader,
) -> Result<TreeObject, ObjectParseError> {
    todo!()
}

fn read_tag_object(
    _buf_reader: &mut impl std::io::BufRead,
    _object_header: GitObjectHeader,
) -> Result<TagObject, ObjectParseError> {
    todo!()
}

fn parse_object_file_header(
    buf_reader: &mut impl std::io::BufRead,
) -> Result<GitObjectHeader, ObjectParseError> {
    let mut buffer = Vec::new();
    let length = buf_reader
        .read_until(b' ', &mut buffer)
        .context("Failed to read object type")?;
    let object_type = String::from_utf8_lossy(&buffer[..length - 1]);
    let object_type: GitObjectType = object_type.parse()?;

    buffer = Vec::new();
    let length = buf_reader.read_until(b'\x00', &mut buffer)?;
    let object_size = String::from_utf8_lossy(&buffer[..length - 1]);
    let object_size = object_size.parse()?;

    Ok(GitObjectHeader {
        object_type,
        object_size,
    })
}

#[cfg(test)]
mod tests {
    use super::GitObjectType;

    use super::parse_object_file_header;

    #[test]
    fn parse_object_file_header_should_read_correct_header() -> Result<(), anyhow::Error> {
        // 00000000  63 6f 6d 6d 69 74 20 31  30 38 36 00 74 72 65 65  |commit 1086.tree|
        let object_header = hex::decode("636f6d6d697420313038360074726565").unwrap();
        let object_header = parse_object_file_header(&mut object_header.as_ref())?;
        assert_eq!(object_header.object_type, GitObjectType::Commit);
        assert_eq!(object_header.object_size, 1086);

        Ok(())
    }
}
