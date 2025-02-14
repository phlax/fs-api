use crate::config::Config;
use log::debug;
use std::path;
use std::sync::Arc;
use tokio::fs;

#[derive(Debug)]
pub struct Directory {
    pub config: Arc<Config>,
    pub endpoint: path::PathBuf,
    pub path: String,
    pub root_dir: path::PathBuf,
}

impl<'a> Directory {
    pub fn new(config: Arc<Config>, endpoint: path::PathBuf, path: String, root_dir: path::PathBuf) -> Self {
        Directory { config, endpoint, path, root_dir }
    }

    pub async fn listing(&self) -> String {
        debug!("DPATH: {:?}", self.path);
        fs::read("./example/ui/index.html").await
            .ok()
            .and_then(|bytes| String::from_utf8(bytes).ok())
            .unwrap_or_else(|| "[File not found or invalid UTF-8]".to_string())
    }

    pub fn content_type(&self) -> String {
        "text/html".to_string()
    }
}
