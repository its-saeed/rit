#[cfg(test)]
pub mod directory_manager {
    use rit::DirectoryManager;

    use super::general::generate_random_path;
    pub fn create_directory_manager() -> DirectoryManager {
        rit::DirectoryManager::new(generate_random_path())
    }
}

#[cfg(test)]
pub mod general {
    use std::path::PathBuf;

    pub fn generate_random_path() -> PathBuf {
        std::env::temp_dir().join(uuid::Uuid::new_v4().to_string())
    }
}
