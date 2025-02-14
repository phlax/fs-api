use crate::config::Config;
use std::path;
use std::sync::Arc;
use tokio::fs;

#[derive(Debug)]
pub struct File {
    pub config: Arc<Config>,
    pub endpoint: path::PathBuf,
    pub path: String,
    pub root_dir: path::PathBuf,
}

impl<'a> File {
    pub fn new(config: Arc<Config>, endpoint: path::PathBuf, path: String, root_dir: path::PathBuf) -> Self {
        File { config, endpoint, path, root_dir }
    }

    pub async fn read(&self) -> Result<Vec<u8>, String> {
        match fs::read(&self.endpoint).await {
            Ok(bytes) => {
                if let Some(extension) = self.endpoint.extension() {
                    let ext = extension.to_str().unwrap_or("").to_lowercase();
                    // fix this - make explicit/config
                    if ["txt", "html", "json", "css", "js"].contains(&ext.as_str()) {
                        // Treat it as a text file and convert to String
                        match String::from_utf8(bytes) {
                            Ok(text) => return Ok(text.into_bytes()),
                            Err(_) => return Err("[Invalid UTF-8]".to_string()),
                        }
                    }
                }
                Ok(bytes)
            },
            Err(_) => Err("[File not found]".to_string()),
        }
    }

    pub fn mimetype(&self) -> String {
        let extension = self.endpoint.extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("")
            .to_lowercase();
        if let Some(mimetypes) = &self.config.mimetypes {
            if let Some(mapping) = mimetypes.0.iter().find(|m| m.extension == extension) {
                return mapping.mimetype.clone();
            }
        }
        "application/octet-stream".to_string()
    }
}
