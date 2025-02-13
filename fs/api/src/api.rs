use crate::config::Config;
use log::debug;
use serde::{Serialize, Deserialize};
use std::path;
use std::sync::Arc;
use tokio::fs;

#[derive(Debug)]
pub struct API {
    pub config: Arc<Config>,
    pub endpoint: path::PathBuf,
    pub path: String,
    pub root_dir: path::PathBuf,
}

impl<'a> API {
    pub fn new(config: Arc<Config>, endpoint: path::PathBuf, path: String, root_dir: path::PathBuf) -> Self {
        API { config, endpoint, path, root_dir }
    }

    async fn listing(&self) -> Vec<DirectoryEntry> {
        let mut children = Vec::new();
        let path = self.root_dir.join(&self.endpoint);
        if let Ok(mut entries) = fs::read_dir(&path).await {
            while let Ok(Some(entry)) = entries.next_entry().await {
                let mut mimetype_str = "application/directory".to_string();
                let mut mimetype_icon = "📂".to_string();
                let path = entry.path();
                if let Some(ext) = path.extension() {
                    if let Some(ext_str) = ext.to_str() {
                        if !ext_str.is_empty() {
                            if let Some(mimetypes) = &self.config.mimetypes {
                                let mimetype = mimetypes.get(ext_str);
                                mimetype_str = mimetype.mimetype.clone();
                                mimetype_icon = mimetype.icon.clone();
                            }
                        }
                    }
                }
                let name = path.file_name()
                    .and_then(|name| name.to_str())
                    .map(|s| s.to_string())
                    .unwrap_or_else(|| "".to_string());
                let dentry = DirectoryEntry {
                    name,
                    mimetype: mimetype_str,
                    mimetype_icon: mimetype_icon,
                    is_dir: path.is_dir(),
                    request: self.path.clone(),
                };
                children.push(dentry);
            }
        }
        children
    }

    fn to_json(&self, children: Vec<DirectoryEntry>) -> String {
        // Serialize the vector of directory entries to JSON
        serde_json::to_string(&children).unwrap()
    }

    pub async fn json(&self) -> String {
        self.to_json(self.listing().await)
    }

    pub fn content_type(&self) -> String {
        "application/json".to_string()
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct DirectoryEntry {
    name: String,
    mimetype: String,
    mimetype_icon: String,
    is_dir: bool,
    request: String,
}
