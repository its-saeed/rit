use std::path::{Path, PathBuf};

use anyhow::Context;

use crate::{error::CreateRepoError, git_config::GitConfig, git_object, DirectoryManager};

#[derive(Debug)]
pub struct GitRepository {
    pub config: GitConfig,
    pub directory_manager: DirectoryManager,
}

impl GitRepository {
    /// Load an existing repository.
    pub fn load<T: Into<PathBuf>>(base_path: T) -> Result<Self, CreateRepoError> {
        GitRepository::try_from(DirectoryManager::new(base_path))
    }

    /// Try to load a git repo in `working_dir`, if it fails, recursively try parent directory.
    pub fn find(working_dir: &Path) -> Result<Self, CreateRepoError> {
        match DirectoryManager::is_toplevel_directory(working_dir) {
            true => GitRepository::load(working_dir),
            false => {
                let parent_path = working_dir
                    .parent()
                    .ok_or(CreateRepoError::NoToplevelFoundError)?;
                GitRepository::find(parent_path)
            }
        }
    }

    /// Create a new repository
    pub fn create<T: Into<PathBuf>>(base_path: T) -> Result<Self, CreateRepoError> {
        let directory_manager = DirectoryManager::new(base_path);

        if directory_manager.work_tree.exists() && !directory_manager.work_tree.is_dir() {
            return Err(CreateRepoError::TopLevelIsNotDirectory);
        }

        if !directory_manager
            .is_dot_git_empty()
            .context("Failed to check if .git is empty")?
        {
            return Err(CreateRepoError::TopLevelIsNotEmpty);
        }

        directory_manager
            .create_directory_tree()
            .context("Failed to create directory tree")?;

        // Write initial contents of .git/description
        std::fs::write(
            &directory_manager.description_file,
            "Unnamed repository; edit this file 'description' to name the repository.\n",
        )
        .context("Failed to write to .git/description")?;

        // Write initial contents of .git/HEAD
        std::fs::write(&directory_manager.head_file, "ref: refs/heads/master\n")
            .context("Failed to write to .git/HEAD")?;

        // Write initial contents of .git/config
        std::fs::write(&directory_manager.config_file, GitConfig::default_str())
            .context("Failed to write to .git/config")?;

        Ok(Self {
            directory_manager,
            config: GitConfig::default(),
        })
    }

    pub fn find_object(&self, _object_type: git_object::GitObjectType, name: String) -> String {
        name
    }
}

impl TryFrom<DirectoryManager> for GitRepository {
    type Error = CreateRepoError;

    fn try_from(directory_manager: DirectoryManager) -> Result<Self, Self::Error> {
        let config = GitConfig::load_from_file(&directory_manager.config_file)?;

        if !config.is_repository_format_version_valid()? {
            return Err(CreateRepoError::InvalidRepositoryFormatVersionError);
        }

        Ok(Self {
            config,
            directory_manager,
        })
    }
}
