use configparser::ini::Ini;
use std::{
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};

use crate::git_config::GitConfig;

#[derive(Debug)]
pub struct GitRepository {
    worktree: PathBuf,
    config: GitConfig,
}

impl GitRepository {
    pub fn new(path: &Path, _force: bool) -> Result<Self, String> {
        let git_path = GitRepository::git_dir(path);
        let config_file = GitRepository::repo_path(&git_path, &["config"]);

        let mut config_file = File::open(config_file).map_err(|e| e.to_string())?;
        let mut config_string = String::new();
        config_file
            .read_to_string(&mut config_string)
            .map_err(|e| e.to_string())?;

        let config: GitConfig = config_string.parse()?;

        let version = config.repository_format_version()?;
        if version != 0 {
            return Err(format!(
                "Repository format version {} not supported",
                version
            ));
        }

        Ok(Self {
            worktree: path.into(),
            config,
        })
    }

    pub fn git_dir(path: &Path) -> PathBuf {
        path.to_owned().join(".git")
    }

    pub fn repo_path(git_path: &Path, paths: &[&str]) -> PathBuf {
        let mut git_dir = git_path.to_owned();
        for path in paths {
            git_dir.push(path);
        }
        git_dir
    }
}

#[cfg(test)]
mod tests {
    const PROJECT_DIR: &'static str = "~/home/projects/test";
    use std::path::Path;

    use super::GitRepository;

    #[test]
    fn should_return_correct_git_path() {
        assert_eq!(
            GitRepository::git_dir(&Path::new(&PROJECT_DIR)),
            Path::new("~/home/projects/test/.git")
        );
    }

    #[test]
    fn repo_path_function_should_return_correct_path() {
        let git_path = GitRepository::git_dir(Path::new(PROJECT_DIR));
        assert_eq!(
            GitRepository::repo_path(&git_path, &["config"]),
            Path::new("~/home/projects/test/.git/config")
        );

        assert_eq!(
            GitRepository::repo_path(&git_path, &["another", "file"]),
            Path::new("~/home/projects/test/.git/another/file")
        );
    }
}
