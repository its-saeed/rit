pub mod refs;

use crate::{
    error::{repository::ResolveRefError, CreateRepoError, ObjectCreateError, ObjectParseError},
    git_config::GitConfig,
    git_object::{self, Blob, CompressedGitObject, KeyValueList, SerializedGitObject, Tag, Type},
    DirectoryManager, GitObject,
};

use std::{
    collections::BTreeMap,
    fs::File,
    io::{BufReader, Read},
    path::{Path, PathBuf},
};

use anyhow::Context;

#[derive(Debug)]
pub struct GitRepository {
    pub config: GitConfig,
    pub directory_manager: DirectoryManager,
}

// Constructors
impl GitRepository {
    pub fn new(config: GitConfig, directory_manager: DirectoryManager) -> Self {
        Self {
            config,
            directory_manager,
        }
    }

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

        Ok(Self::new(GitConfig::default(), directory_manager))
    }
}

// Object related methods
impl GitRepository {
    pub fn find_object(
        &self,
        _object_type: git_object::Type,
        name: String,
    ) -> Result<String, anyhow::Error> {
        if name == "HEAD" {
            let ref_entry = refs::resolve_ref(
                &self.directory_manager.dot_git_path,
                &self.directory_manager.dot_git_path.join("HEAD"),
            )?;
            Ok(ref_entry)
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

    pub fn create_object(
        file_path: &Path,
        object_type: Type,
    ) -> Result<SerializedGitObject, ObjectCreateError> {
        let mut buf_reader = BufReader::new(File::open(file_path)?);
        let mut buffer = String::new();
        buf_reader.read_to_string(&mut buffer)?;

        let object = match object_type {
            Type::Commit => todo!(),
            Type::Tree => todo!(),
            Type::Tag => todo!(),
            Type::Blob => GitObject::Blob(Blob { blob: buffer }),
        };

        object.try_into()
    }

    pub fn write_object(
        &self,
        serialized_object: &SerializedGitObject,
    ) -> Result<(), anyhow::Error> {
        let file_path = self
            .directory_manager
            .sha_to_file_path(&serialized_object.hash, true)?;

        std::fs::write(file_path, CompressedGitObject::try_from(serialized_object)?)?;

        Ok(())
    }
}

// Refs methods
impl GitRepository {
    pub fn resolve_ref(&self, ref_relative_path: &str) -> Result<String, ResolveRefError> {
        let ref_path = self.directory_manager.dot_git_path.join(ref_relative_path);
        refs::resolve_ref(&self.directory_manager.dot_git_path, &ref_path)
    }

    pub fn list_refs(&self) -> Result<Vec<refs::Ref>, ResolveRefError> {
        self.list_refs_in_absolute(&self.directory_manager.refs_path)
    }

    pub fn list_refs_in(&self, path: &Path) -> Result<Vec<refs::Ref>, ResolveRefError> {
        self.list_refs_in_absolute(&self.directory_manager.refs_path.join(path))
    }

    pub fn list_refs_in_absolute(&self, path: &Path) -> Result<Vec<refs::Ref>, ResolveRefError> {
        let refs = refs::list_refs(&self.directory_manager.dot_git_path, path)?;
        refs.into_iter()
            .map(|ref_item| {
                Ok(refs::Ref {
                    hash: ref_item.hash,
                    path: ref_item
                        .path
                        .strip_prefix(&self.directory_manager.dot_git_path)
                        .context("Failed to strip_prefix")?
                        .to_path_buf(),
                })
            })
            .collect()
    }
}

// Tag methods
impl GitRepository {
    pub fn create_lightweight_tag(
        &self,
        name: String,
        object: String,
    ) -> Result<(), anyhow::Error> {
        let object = self.find_object(git_object::Type::Tag, object)?;
        std::fs::write(self.directory_manager.refs_tags_path.join(name), object)?;

        Ok(())
    }

    pub fn create_tag_object(&self, name: String, object: String) -> Result<(), anyhow::Error> {
        let object = self.find_object(git_object::Type::Tag, object)?;
        let kvl = BTreeMap::from([
            ("object".to_string(), object),
            ("type".to_string(), "commit".to_string()),
            ("tag".to_string(), name.clone()),
            (
                "tagger".to_string(),
                "Saeed <saeed@zilliqa.com>".to_string(),
            ),
            ("message".to_string(), "This is the message".to_string()),
        ]);

        let tag = Tag {
            kvl: KeyValueList::new(kvl),
        };

        let serialized = SerializedGitObject::try_from(GitObject::Tag(tag))?;

        self.write_object(&serialized)?;
        std::fs::write(
            self.directory_manager.refs_tags_path.join(name),
            serialized.hash,
        )?;
        Ok(())
    }
}

impl TryFrom<DirectoryManager> for GitRepository {
    type Error = CreateRepoError;

    fn try_from(directory_manager: DirectoryManager) -> Result<Self, Self::Error> {
        let config = GitConfig::load_from_file(&directory_manager.config_file)?;

        if !config.is_repository_format_version_valid()? {
            return Err(CreateRepoError::InvalidRepositoryFormatVersionError);
        }

        Ok(Self::new(config, directory_manager))
    }
}
