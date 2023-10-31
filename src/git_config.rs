use std::{collections::HashMap, fs::File, io::Read, path::Path, str::FromStr};

use anyhow::Context;
use configparser::ini::Ini;

use crate::error::ConfigParseError;

type Config = HashMap<String, HashMap<String, Option<String>>>;

#[derive(Debug)]
pub struct GitConfig {
    config: Config,
}

impl GitConfig {
    pub fn load_from_file(path: &Path) -> Result<Self, ConfigParseError> {
        let mut config_file = File::open(path).context("Failed to open config file")?;
        let mut config_string = String::new();
        config_file
            .read_to_string(&mut config_string)
            .context("Failed to read config file")?;

        config_string.parse()
    }

    pub fn repository_format_version(&self) -> Result<u16, ConfigParseError> {
        let core = self
            .config
            .get("core")
            .ok_or(ConfigParseError::ParseFailed(
                "Core section doesn't exist".to_string(),
            ))?;

        match core
            .get("repositoryformatversion")
            .ok_or(ConfigParseError::ParseFailed(
                "repositoryformatversion not found.".to_string(),
            ))?
            .clone()
            .map(|ver| ver.parse::<u16>())
            .transpose()
            .map_err(|e| ConfigParseError::ParseFailed(e.to_string()))?
        {
            Some(v) => Ok(v),
            None => Err(ConfigParseError::ParseFailed(
                "repositoryformatversion doesn't exist in config".to_string(),
            )),
        }
    }

    pub fn is_repository_format_version_valid(&self) -> Result<bool, ConfigParseError> {
        Ok(self.repository_format_version()? == 0)
    }

    pub fn default_str() -> &'static str {
        r#"[core]
            bare = false
            repositoryformatversion = 0
            filemode = false"#
    }
}

impl FromStr for GitConfig {
    type Err = ConfigParseError;

    fn from_str(config_str: &str) -> Result<Self, Self::Err> {
        let mut config = Ini::new();
        let config = config
            .read(config_str.to_string())
            .map_err(ConfigParseError::ParseFailed)?;

        Ok(Self { config })
    }
}

impl Default for GitConfig {
    fn default() -> Self {
        GitConfig::default_str().parse().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::GitConfig;

    #[test]
    fn if_config_string_is_valid_repository_format_version_should_return_version() {
        let config_string = r#"
        [core]
            bare = false
            repositoryformatversion = 0
        "#;

        let config: GitConfig = config_string.parse().unwrap();

        assert_eq!(config.repository_format_version().unwrap(), 0);
        assert!(config.is_repository_format_version_valid().unwrap());
    }

    #[test]
    fn if_config_string_doesnt_have_version_repository_format_version_function_should_return_error()
    {
        let config_string = r#"
        [core]
            bare = false
        "#;

        let config: GitConfig = config_string.parse().unwrap();
        let version = config.repository_format_version();
        assert!(version.is_err());
    }

    #[test]
    fn if_repository_format_version_is_not_inside_core_function_should_return_error() {
        let config_string = r#"
        [notcore]
            bare = false
            repositoryformatversion = 0
        "#;

        let config: GitConfig = config_string.parse().unwrap();
        let version = config.repository_format_version();
        assert!(version.is_err());
    }
}
