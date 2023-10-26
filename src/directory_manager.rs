use std::{fs, path::PathBuf};

#[derive(Debug)]
pub struct DirectoryManager {
    pub work_tree: PathBuf,
    pub dot_git_path: PathBuf,
    pub config_file: PathBuf,
    pub description_file: PathBuf,
    pub head_file: PathBuf,
    pub branches_path: PathBuf,
    pub objects_path: PathBuf,
    pub refs_tags_path: PathBuf,
    pub refs_heads_path: PathBuf,
}

impl DirectoryManager {
    pub fn new<T: Into<PathBuf>>(base_path: T) -> Self {
        let base_path: PathBuf = base_path.into();
        let dot_git_path = base_path.join(".git");

        Self {
            work_tree: base_path,
            config_file: dot_git_path.join("config"),
            description_file: dot_git_path.join("description"),
            head_file: dot_git_path.join("HEAD"),
            branches_path: dot_git_path.join("branches"),
            objects_path: dot_git_path.join("objects"),
            refs_tags_path: dot_git_path.join("refs").join("tags"),
            refs_heads_path: dot_git_path.join("refs").join("heads"),
            dot_git_path,
        }
    }

    pub fn is_dot_git_empty(&self) -> Result<bool, std::io::Error> {
        Ok(!self.dot_git_path.exists() || self.dot_git_path.read_dir()?.next().is_none())
    }

    pub fn create_directory_tree(&self) -> Result<(), std::io::Error> {
        fs::create_dir_all(&self.work_tree)?;
        fs::create_dir_all(&self.dot_git_path)?;
        fs::create_dir_all(&self.branches_path)?;
        fs::create_dir_all(&self.objects_path)?;
        fs::create_dir_all(&self.refs_heads_path)?;
        fs::create_dir_all(&self.refs_tags_path)?;
        Ok(())
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
    fn should_return_correct_file_paths() {
        let dir_manager = DirectoryManager::new(PROJECT_DIR);
        assert_eq!(
            dir_manager.config_file,
            Path::new("~/home/projects/test/.git/config")
        );
        assert_eq!(
            dir_manager.description_file,
            Path::new("~/home/projects/test/.git/description")
        );
        assert_eq!(
            dir_manager.head_file,
            Path::new("~/home/projects/test/.git/HEAD")
        );
    }

    #[test]
    fn should_return_correct_paths() {
        let dir_manager = DirectoryManager::new(PROJECT_DIR);
        assert_eq!(
            dir_manager.objects_path,
            Path::new("~/home/projects/test/.git/objects")
        );
        assert_eq!(
            dir_manager.branches_path,
            Path::new("~/home/projects/test/.git/branches")
        );
        assert_eq!(
            dir_manager.refs_heads_path,
            Path::new("~/home/projects/test/.git/refs/heads")
        );
        assert_eq!(
            dir_manager.refs_tags_path,
            Path::new("~/home/projects/test/.git/refs/tags")
        );
        assert_eq!(
            dir_manager.dot_git_path,
            Path::new("~/home/projects/test/.git/")
        );
    }
}
