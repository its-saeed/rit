use std::path::{Path, PathBuf};

use crate::{git_config::GitConfig, DirectoryManager};

#[derive(Debug)]
pub struct GitRepository {
    pub config: GitConfig,
    pub directory_manager: DirectoryManager,
}

impl GitRepository {
    /// Load an existing repository.
    pub fn load<T: Into<PathBuf>>(base_path: T) -> Result<Self, String> {
        GitRepository::try_from(DirectoryManager::new(base_path))
    }

    /// Try to load a git repo in `working_dir`, if it fails, recursively try parent directory.
    pub fn find(working_dir: &Path) -> Result<Self, String> {
        match DirectoryManager::is_toplevel_directory(working_dir) {
            true => GitRepository::load(working_dir),
            false => {
                let parent_path = working_dir.parent().ok_or("Not a git repository")?;
                GitRepository::find(parent_path)
            }
        }
    }

    /// Create a new repository
    pub fn create<T: Into<PathBuf>>(base_path: T) -> Result<Self, String> {
        let directory_manager = DirectoryManager::new(base_path);

        if directory_manager.work_tree.exists() && !directory_manager.work_tree.is_dir() {
            return Err(format!(
                "{} is not a directory!",
                directory_manager.work_tree.display()
            ));
        }

        if !directory_manager
            .is_dot_git_empty()
            .map_err(|e| e.to_string())?
        {
            return Err(format!(
                "{} is not empty!",
                directory_manager.dot_git_path.display()
            ));
        }

        directory_manager
            .create_directory_tree()
            .map_err(|e| e.to_string())?;

        // Write initial contents of .git/description
        std::fs::write(
            &directory_manager.description_file,
            "Unnamed repository; edit this file 'description' to name the repository.\n",
        )
        .map_err(|e| e.to_string())?;

        // Write initial contents of .git/HEAD
        std::fs::write(&directory_manager.head_file, "ref: refs/heads/master\n")
            .map_err(|e| e.to_string())?;

        // Write initial contents of .git/config
        std::fs::write(&directory_manager.config_file, GitConfig::default_str())
            .map_err(|e| e.to_string())?;

        Ok(Self {
            directory_manager,
            config: GitConfig::default(),
        })
    }
}

impl TryFrom<DirectoryManager> for GitRepository {
    type Error = String;

    fn try_from(directory_manager: DirectoryManager) -> Result<Self, Self::Error> {
        let config = GitConfig::load_from_file(&directory_manager.config_file)?;

        if !config.is_repository_format_version_valid()? {
            return Err("Repository format version not supported".to_string());
        }

        Ok(Self {
            config,
            directory_manager,
        })
    }
}
