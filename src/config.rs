use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub server: ServerConfig,
    pub iiif: IiifConfig,
    pub cache: CacheConfig,
    pub remote: Option<RemoteConfig>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub port: u16,
    pub host: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct IiifConfig {
    pub source_dir: String,
    pub base_url: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CacheConfig {
    pub memory_limit: String, // e.g., "512MB"
    pub disk_cache_dir: String,
    #[allow(dead_code)]
    pub disk_limit: String, // e.g., "10GB"
}

#[derive(Debug, Deserialize, Clone)]
pub struct RemoteConfig {
    pub base_url: String, // e.g., "https://s3.amazonaws.com/my-bucket/"
    pub local_proxy_dir: String, // where to save downloaded remote files
}

impl Config {
    pub fn load() -> Result<Self, config::ConfigError> {
        let settings = config::Config::builder()
            .add_source(config::File::with_name("config").required(false))
            .add_source(config::Environment::with_prefix("IIIF"))
            .set_default("server.port", 8080)?
            .set_default("server.host", "0.0.0.0")?
            .set_default("iiif.source_dir", "./images")?
            .set_default("iiif.base_url", "http://localhost:8080/iiif/3/")?
            .set_default("cache.memory_limit", "512MB")?
            .set_default("cache.disk_cache_dir", "./cache")?
            .set_default("cache.disk_limit", "10GB")?
            .build()?;

        settings.try_deserialize()
    }

    pub fn parse_memory_limit(&self) -> u64 {
        parse_size_string(&self.cache.memory_limit).unwrap_or(512 * 1024 * 1024)
    }
}

fn parse_size_string(s: &str) -> Option<u64> {
    let s = s.to_uppercase();
    if s.ends_with("GB") {
        s.trim_end_matches("GB").trim().parse::<u64>().ok().map(|v| v * 1024 * 1024 * 1024)
    } else if s.ends_with("MB") {
        s.trim_end_matches("MB").trim().parse::<u64>().ok().map(|v| v * 1024 * 1024)
    } else if s.ends_with("KB") {
        s.trim_end_matches("KB").trim().parse::<u64>().ok().map(|v| v * 1024)
    } else if s.ends_with('B') {
        s.trim_end_matches('B').trim().parse::<u64>().ok()
    } else {
        s.parse::<u64>().ok()
    }
}
