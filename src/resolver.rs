use std::path::{Path, PathBuf};
use crate::config::Config;
use tokio::fs;
use tracing::{info, debug, error};

pub struct Resolver {
    config: Config,
    client: reqwest::Client,
}

impl Resolver {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            client: reqwest::Client::new(),
        }
    }

    pub async fn resolve(&self, identifier: &str) -> Option<PathBuf> {
        // 1. Try local source_dir
        let base_id = if let Some((base, _)) = identifier.split_once(":page:") {
            base
        } else {
            identifier
        };

        let local_path = Path::new(&self.config.iiif.source_dir).join(base_id);
        if local_path.exists() {
            return Some(local_path);
        }

        // 2. Try remote if configured
        if let Some(remote_cfg) = &self.config.remote {
            let proxy_path = Path::new(&remote_cfg.local_proxy_dir).join(base_id);
            
            // Return proxy if already downloaded
            if proxy_path.exists() {
                debug!("Remote file already in proxy: {}", base_id);
                return Some(proxy_path);
            }

            // Otherwise, fetch from remote
            let remote_url = format!("{}{}", remote_cfg.base_url, base_id);
            info!("Fetching remote file: {}", remote_url);

            match self.fetch_remote(&remote_url, &proxy_path).await {
                Ok(_) => {
                    info!("Successfully cached remote file to {}", proxy_path.display());
                    return Some(proxy_path);
                }
                Err(e) => {
                    error!("Failed to fetch remote file {}: {:?}", remote_url, e);
                    return None;
                }
            }
        }

        None
    }

    async fn fetch_remote(&self, url: &str, dest: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        let response = self.client.get(url).send().await?;
        if !response.status().is_success() {
            return Err(format!("Remote server returned status {}", response.status()).into());
        }

        if let Some(parent) = dest.parent() {
            fs::create_dir_all(parent).await?;
        }

        let content = response.bytes().await?;
        fs::write(dest, content).await?;
        Ok(())
    }
}
