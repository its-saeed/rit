use std::{collections::HashMap, str::FromStr};

use configparser::ini::Ini;

type Config = HashMap<String, HashMap<String, Option<String>>>;

#[derive(Debug)]
pub struct GitConfig {
    config: Config,
}

impl GitConfig {
    pub fn repository_format_version(&self) -> Result<u16, String> {
        let core = self
            .config
            .get("core")
            .ok_or("Core section doesn't exist".to_string())?;

        match core
            .get("repositoryformatversion")
            .ok_or("repositoryformatversion not found")?
            .clone()
            .map(|ver| ver.parse::<u16>())
            .transpose()
            .map_err(|e| e.to_string())?
        {
            Some(v) => Ok(v),
            None => return Err("Failed to parse repositoryformatversion".to_string()),
        }
    }
}

impl FromStr for GitConfig {
    type Err = String;

    fn from_str(config_str: &str) -> Result<Self, Self::Err> {
        let mut config = Ini::new();
        let config = config
            .read(config_str.to_string())
            .map_err(|_| "Failed to parse config".to_string())?;

        Ok(Self { config })
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
