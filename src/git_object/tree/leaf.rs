#[derive(Debug)]
pub struct Leaf {
    pub mode: Mode,
    pub path: String,
    pub hash: String,
}

impl Leaf {
    pub fn new(mode: &[u8], path: &[u8], hash: String) -> Result<Self, TreeLeafParseError> {
        let mode = Mode::new(String::from_utf8(mode.to_vec())?)?;
        let path = String::from_utf8(path.to_vec())?;

        Ok(Self { mode, path, hash })
    }

    pub fn parse(buf_reader: &mut impl std::io::BufRead) -> Result<Self, TreeLeafParseError> {
        let mut mode = vec![];
        let mode_size = buf_reader
            .read_until(b' ', &mut mode)
            .context("Failed to read mode")?;

        let mut path = vec![];
        let path_size = buf_reader
            .read_until(b'\x00', &mut path)
            .context("Failed to read path")?;

        let mut hash = [0_u8; 20];
        buf_reader
            .read_exact(&mut hash)
            .context("Failed to read sha1 hash")?;

        Self::new(
            &mode[..mode_size - 1],
            &path[..path_size - 1],
            hex::encode(hash),
        )
    }
