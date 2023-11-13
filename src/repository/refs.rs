use std::{
    fmt::Display,
    fs,
    path::{Path, PathBuf},
};

use crate::error::repository::ResolveRefError;

#[derive(Debug)]
pub struct Ref {
    pub hash: String,
    pub path: PathBuf,
}

impl Display for Ref {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.hash, self.path.display())
    }
}

pub fn resolve_ref(dot_git_path: &Path, ref_path: &Path) -> Result<String, ResolveRefError> {
    if ref_path.is_file() == false {
        return Err(ResolveRefError::RelativePathIsNotAFile(format!(
            "{}",
            ref_path.display()
        )));
    }

    let ref_value = fs::read_to_string(ref_path)?;
    let ref_value = ref_value.trim_end();

    if ref_value.starts_with("ref: ") {
        return resolve_ref(
            dot_git_path,
            &dot_git_path.join(&PathBuf::from(&ref_value[5..])),
        );
        // Skip ref:
    }

    Ok(ref_value.to_string())
}

pub fn list_refs(dot_git_path: &Path, path: &Path) -> Result<Vec<Ref>, ResolveRefError> {
    let mut refs = vec![];

    for entry in std::fs::read_dir(path)? {
        let entry = entry?;
        let entry_path = entry.path();
        if entry_path.is_dir() {
            let out = list_refs(dot_git_path, &entry_path)?;
            refs.extend(out);
        } else {
            let hash = resolve_ref(dot_git_path, &entry_path)?;
            refs.push(Ref {
                hash,
                path: entry_path,
            });
        }
    }

    Ok(refs)
}
