pub mod leaf;
pub mod mode;

use std::{fmt::Display, ops::Deref};

use self::leaf::Leaf;

#[derive(Debug)]
pub struct Tree {
    pub leaves: Vec<Leaf>,
}

impl Tree {
    pub fn serialize(&self) -> String {
        todo!()
    }

    pub fn deserialize(
        mut buf_reader: impl std::io::BufRead,
        _object_header: super::Header,
    ) -> Result<Self, crate::error::ObjectParseError> {
        let mut leaves = vec![];
        loop {
            match Leaf::parse(&mut buf_reader) {
                Ok(leaf) => leaves.push(leaf),
                // TODO: Fix this
                Err(_) => break,
            }
        }

        Ok(Self { leaves })
    }
}

impl Display for Tree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for leaf in &self.leaves {
            writeln!(f, "{}", leaf)?
        }

        Ok(())
    }
}

impl Deref for Tree {
    type Target = Vec<Leaf>;

    fn deref(&self) -> &Self::Target {
        &self.leaves
    }
}
