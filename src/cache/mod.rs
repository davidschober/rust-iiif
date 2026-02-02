use moka::future::Cache;
use tokio::fs;
use std::path::PathBuf;
use sha2::{Sha256, Digest};

pub struct TileCache {
    memory: Cache<String, Vec<u8>>,
    disk_dir: PathBuf,
}

impl TileCache {
    pub fn new(disk_dir: String, memory_limit_bytes: u64) -> Self {
        let memory = Cache::builder()
            .max_capacity(memory_limit_bytes)
            .build();
        
        let disk_dir = PathBuf::from(disk_dir);
        
        Self {
            memory,
            disk_dir,
        }
    }

    pub fn get_key(identifier: &str, params: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(identifier);
        hasher.update(params);
        format!("{:x}", hasher.finalize())
    }

    pub async fn get(&self, key: &str) -> Option<Vec<u8>> {
        // Try Memory
        if let Some(data) = self.memory.get(key).await {
            return Some(data.to_vec());
        }

        // Try Disk
        let disk_path = self.disk_dir.join(key);
        if let Ok(data) = fs::read(&disk_path).await {
            // Populate memory cache
            self.memory.insert(key.to_string(), data.clone()).await;
            return Some(data);
        }

        None
    }

    pub async fn set(&self, key: &str, data: Vec<u8>) {
        // Set Memory
        self.memory.insert(key.to_string(), data.clone()).await;

        // Set Disk
        let disk_path = self.disk_dir.join(key);
        if let Some(parent) = disk_path.parent() {
            let _ = fs::create_dir_all(parent).await;
        }
        let _ = fs::write(disk_path, data).await;
    }
}
