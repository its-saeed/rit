use std::{
    fs,
    path::{Path, PathBuf},
};

#[derive(Debug, Clone)]
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

    pub fn is_toplevel_directory(path: &Path) -> bool {
        path.exists() && path.join(".git").is_dir() && path.join(".git/config").is_file()
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

    pub fn sha_to_file_path(&self, sha: &str) -> PathBuf {
        self.objects_path.join(&sha[0..2]).join(&sha[2..])
    }
}

#[cfg(test)]
mod tests {
    const PROJECT_DIR: &'static str = "~/home/projects/test";
    use std::path::{Path, PathBuf};

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

    #[test]
    fn sha_to_file_path_should_return_correct_path() {
        let dir_manager = DirectoryManager::new(PROJECT_DIR);

        let file_path = dir_manager.sha_to_file_path("e673d1b7eaa0aa01b5bc2442d570a765bdaae751");
        assert_eq!(
            file_path,
            PathBuf::from(format!(
                "{}/.git/objects/e6/73d1b7eaa0aa01b5bc2442d570a765bdaae751",
                PROJECT_DIR
            ))
        );
    }
}
