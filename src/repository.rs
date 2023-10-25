use std::{fs::File, io::Read, path::Path};

use crate::{git_config::GitConfig, DirectoryManager};

#[derive(Debug)]
pub struct GitRepository {
    config: GitConfig,
    directory_manager: DirectoryManager,
}

impl GitRepository {
    /// Load an existing repository.
    pub fn load(base_path: &Path) -> Result<Self, String> {
        let directory_manager = DirectoryManager::new(base_path);
        let config_file = directory_manager.config_file();

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
    pub fn create(_base_path: &Path) -> Self {
        todo!()
    }
}
