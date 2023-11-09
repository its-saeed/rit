use std::{
    fs::{self, File},
    io::BufReader,
    path::{Path, PathBuf},
};

use anyhow::Context;

use crate::{
    error::{CreateRepoError, ObjectCreateError, ObjectParseError},
    git_config::GitConfig,
    git_object::{self, CompressedGitObject, SerializedGitObject, Type},
    DirectoryManager, GitObject,
};

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

    pub fn find_object(
        &self,
        object_type: git_object::Type,
        name: String,
    ) -> Result<String, anyhow::Error> {
        if name == "HEAD" && object_type == git_object::Type::Commit {
            let hash = fs::read_to_string(self.directory_manager.refs_heads_path.join("master"))
                .context("Can't open refs/heads/master file")?;
            Ok(hash.trim_end().to_string())
        } else {
            Ok(name)
        }
    }

    pub fn read_object(&self, sha: &str) -> Result<GitObject, ObjectParseError> {
        let real_file_path = self.directory_manager.sha_to_file_path(sha, false)?;
        let file = File::open(real_file_path)?;
        let buf_reader = BufReader::new(file);

        let serialized: SerializedGitObject = CompressedGitObject::decompress(buf_reader)?;

        serialized.try_into()
    }

    pub fn write_object(
        &self,
        file_path: &Path,
        object_type: Type,
    ) -> Result<String, ObjectCreateError> {
        let serialized_object = Self::create_object(file_path, object_type)?;

        let file_path = self
            .directory_manager
            .sha_to_file_path(&serialized_object.hash, true)?;

        std::fs::write(
            file_path,
            CompressedGitObject::try_from(&serialized_object)?,
        )?;
        Ok(serialized_object.hash)
    }

    pub fn create_object(
        file_path: &Path,
        object_type: Type,
    ) -> Result<SerializedGitObject, ObjectCreateError> {
        let buf_reader = BufReader::new(File::open(file_path)?);
        SerializedGitObject::serialize(buf_reader, object_type)
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
