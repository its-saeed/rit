use std::{
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};

use crate::{git_config::GitConfig, DirectoryManager};

#[derive(Debug)]
pub struct GitRepository {
    pub config: GitConfig,
    pub directory_manager: DirectoryManager,
}

impl GitRepository {
    /// Load an existing repository.
    pub fn load(base_path: &Path) -> Result<Self, String> {
        let directory_manager = DirectoryManager::new(base_path);
        let config_file = &directory_manager.config_file;

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
            config,
            directory_manager,
        })
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
