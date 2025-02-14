use crate::api::API;
use crate::config::{Config, Provider};
use crate::directory::Directory;
use crate::file::File;
use crate::path::FSPath;
use hyper::{Body, Request};
use log::{debug, error};
use regex::Regex;
use std::env;
use std::path;
use std::sync::Arc;

#[derive(Debug)]
pub struct Resolver {
    config: Arc<Config>,
}

impl<'a> Resolver {
    pub fn new(config: Arc<Config>) -> Self {
        Resolver { config }
    }

    pub fn api(&self, _path: &str, endpoint: &str, root_directory: path::PathBuf) -> Option<FSPath> {
        debug!("API: {:?}", endpoint);
        let resolved_path = root_directory.join(endpoint.trim_start_matches('/')).to_path_buf();
        debug!("RESOLVED: {:?}", resolved_path);
        if !resolved_path.is_dir() {
            error!("Path does not exist: {}", resolved_path.display());
            return None;
        }
        return Some(FSPath::API(API::new(self.config.clone(), resolved_path, endpoint.to_string(), root_directory)));
    }

    pub fn fs(&self, path: &str, endpoint: &str, root_directory: path::PathBuf) -> Option<FSPath> {
        debug!("ROOTDIR: {:?}", root_directory);
        debug!("REQUEST: {:?}", endpoint);
        let resolved_path = root_directory.join(endpoint.trim_start_matches('/')).to_path_buf();
        debug!("RESOLVED: {:?}", resolved_path);
        if !resolved_path.exists() {
            error!("Path does not exist: {}", resolved_path.display());
            return None;
        }
        if resolved_path.is_dir() {
            return Some(FSPath::Directory(Directory::new(self.config.clone(), resolved_path, path.to_string(), root_directory)));
        } else {
            return Some(FSPath::File(File::new(self.config.clone(), resolved_path, path.to_string(), root_directory)));
        }
    }

    pub fn root_directory(&self) -> Option<path::PathBuf>{
        let mut root_directory = self.config.root_directory.clone()
            .unwrap_or_else(|| path::PathBuf::from("default/path"));
        let cwd = match env::current_dir() {
            Ok(path) => path,
            Err(e) => {
                error!("Failed to get current directory: {}", e);
                return None;
            }
        };
        debug!("CWD: {:?}", cwd);
        root_directory = cwd.join(root_directory);
        Some(root_directory)
    }

    pub async fn resolve(&self, req: &'a Request<Body>) -> Option<FSPath> {
        let req_path = req.uri().path();
        if let Some(paths) = &self.config.paths {
            for mapping in paths {
                debug!("MATCHING: {:?} {}", mapping.pattern, req_path);
                let re = Regex::new(&mapping.pattern).ok()?;
                if let Some(captures) = re.captures(req_path) {
                    if re.captures_len() == 1 {
                        debug!("MATCHED without path: {:?}", mapping.pattern);
                        let root_directory = match self.root_directory() {
                            Some(path) => path,
                            None => {
                                error!("Root directory not found, exiting.");
                                return None;
                            }
                        };
                        return self.fs(req_path, mapping.path.clone().unwrap_or_else(|| "default/path".to_string()).as_str(), root_directory);
                    }

                    if let Some(path_match) = captures.name("path") {
                        debug!("MATCHED: {:?}", mapping.pattern);
                        let root_directory = match self.root_directory() {
                            Some(path) => path,
                            None => {
                                error!("Root directory not found, exiting.");
                                return None;
                            }
                        };

                        // API
                        if let Some(Provider::Api) = mapping.provider {
                            return self.api(req_path, path_match.as_str(), root_directory.join(mapping.path.clone().unwrap_or_else(|| "default/path".to_string())));
                        }

                        // Fs paths
                        debug!("FSDIR: {:?}", root_directory);
                        return self.fs(req_path, path_match.as_str(), root_directory.join(mapping.path.clone().unwrap_or_else(|| "default/path".to_string())));
                    }
                }
            }
        }
        None
    }
}
