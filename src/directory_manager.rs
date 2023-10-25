use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct DirectoryManager {
    pub work_tree: PathBuf,
    pub dot_git_path: PathBuf,
}

impl DirectoryManager {
    pub fn new<T: Into<PathBuf>>(base_path: T) -> Self {
        let base_path: PathBuf = base_path.into();
        Self {
            dot_git_path: base_path.join(".git"),
            work_tree: base_path,
        }
    }

    pub fn config_file(&self) -> PathBuf {
        self.dot_git_path.join("config")
    }

    pub fn mkdir_in_dot_git(&self, _path: &[&Path]) -> Result<(), String> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    const PROJECT_DIR: &'static str = "~/home/projects/test";
    use std::path::Path;

    use crate::DirectoryManager;

    #[test]
    fn should_return_correct_git_path() {
        let dir_manager = DirectoryManager::new(PROJECT_DIR);
        assert_eq!(
            dir_manager.dot_git_path,
            Path::new("~/home/projects/test/.git")
        );
    }

    #[test]
    fn should_return_correct_config_file_path() {
        let dir_manager = DirectoryManager::new(PROJECT_DIR);
        assert_eq!(
            dir_manager.config_file(),
            Path::new("~/home/projects/test/.git/config")
        );
    }
}
