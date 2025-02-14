use log::error;
use serde::Deserialize;
use serde_yaml;
use std::collections::HashMap;
use std::fs::File;
use std::path;

#[derive(Debug, Deserialize)]
pub struct APIConfig {
    pub listen: Option<APIListen>,
    pub log: Option<ApiLogConfig>,
}

#[derive(Debug, Deserialize)]
pub struct APIListen {
    pub host: Option<String>,
    pub port: Option<u16>,
}

#[derive(Debug, Deserialize)]
pub struct ApiLogConfig {
    pub level: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub api: Option<APIConfig>,
    pub mimetypes: Option<Mimetypes>,
    pub paths: Option<Paths>,
    pub root_directory: Option<path::PathBuf>,
}

impl Config {
    pub async fn from_yaml(file_str: &str) -> Result<Config, Box<dyn std::error::Error>> {
        let config_path = path::Path::new(file_str);
        if ! config_path.exists() {
            error!("Path does not exist: {}", config_path.display());
        }
        let file = File::open(config_path)
            .map_err(|e| format!("Failed to open {}: {}", config_path.display(), e))?;
        let config: Config = serde_yaml::from_reader(file)?;
        Ok(config)
    }
}

#[derive(Debug, Deserialize)]
pub struct Mimetype {
    pub extension: String,
    pub icon: String,
    pub mimetype: String,
}

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct Mimetypes(pub Vec<Mimetype>);

impl Mimetypes {
    pub fn new() -> Self {
        Mimetypes(Vec::new())
    }

    pub fn get(&self, key: &str) -> &Mimetype {
        self.mapping()
            .get(key)
            .cloned()
            .unwrap()
    }

    pub fn mapping(&self) -> HashMap<String, &Mimetype> {
        let mut map = HashMap::new();
        for mapping in &self.0 {
            map.insert(mapping.extension.clone(), mapping.clone());
        }
        map
    }
}

impl<'a> IntoIterator for &'a Mimetypes {
    type Item = &'a Mimetype;
    type IntoIter = std::slice::Iter<'a, Mimetype>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

#[derive(Debug, Deserialize)]
pub struct PathMapping {
    pub pattern: String,
    pub path: Option<String>,
    pub provider: Option<Provider>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Provider {
    Api,
    Fs,
}

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct Paths(Vec<PathMapping>);

impl Paths {
    pub fn new() -> Self {
        Paths(Vec::new())
    }

    pub fn get(&self, key: &str) -> Option<String> {
        self.mapping()
            .get(key)
            .cloned()
            .unwrap()
    }

    pub fn mapping(&self) -> HashMap<String, Option<String>> {
        let mut map = HashMap::new();
        for mapping in &self.0 {
            map.insert(mapping.pattern.clone(), mapping.path.clone());
        }
        map
    }
}

impl<'a> IntoIterator for &'a Paths {
    type Item = &'a PathMapping;
    type IntoIter = std::slice::Iter<'a, PathMapping>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    }
}
