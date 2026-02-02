mod config;
mod iiif;
mod processor;
mod cache;

use crate::config::Config;
use crate::iiif::parser;
use crate::iiif::types::*;
use crate::iiif::info::ImageInfo;
use crate::processor::ImageProcessor;
use crate::cache::TileCache;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json,
    Router,
};
use std::net::SocketAddr;
use std::sync::Arc;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

struct AppState {
    config: Config,
    processor: ImageProcessor,
    cache: TileCache,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "rust_iiif=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let cfg = Config::load().expect("Failed to load configuration");
    
    std::fs::create_dir_all(&cfg.cache.disk_cache_dir).expect("Failed to create cache directory");

    let state = Arc::new(AppState {
        config: cfg.clone(),
        processor: ImageProcessor::new(),
        cache: TileCache::new(cfg.cache.disk_cache_dir.clone(), cfg.parse_memory_limit()),
    });

    let app = Router::new()
        .route("/", get(|| async { "Rust IIIF Server is running" }))
        .route("/iiif/3/{identifier}/info.json", get(get_info))
        .route("/iiif/3/{identifier}/{region}/{size}/{rotation}/{quality_format}", get(get_image))
        .with_state(state);

    let victory_msg = format!("Listening on http://{}:{}", cfg.server.host, cfg.server.port);
    let addr = format!("{}:{}", cfg.server.host, cfg.server.port)
        .parse::<SocketAddr>()
        .expect("Invalid address");
    
    tracing::info!("{}", victory_msg);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn get_info(
    State(state): State<Arc<AppState>>,
    Path(identifier): Path<String>,
) -> impl IntoResponse {
    let base_id = if let Some((base, _)) = identifier.split_once(":page:") {
        base
    } else {
        &identifier
    };

    let file_path = format!("{}/{}", state.config.iiif.source_dir, base_id);
    if !std::path::Path::new(&file_path).exists() {
        return (StatusCode::NOT_FOUND, "Image not found").into_response();
    }

    match state.processor.get_image_size(&file_path, &identifier) {
        Ok((w, h)) => {
            let id_url = format!("{}{}", state.config.iiif.base_url, identifier);
            let info = ImageInfo::new(id_url, w as u32, h as u32);
            (StatusCode::OK, Json(info)).into_response()
        }
        Err(e) => {
            tracing::error!("Failed to get image size: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to get image info").into_response()
        }
    }
}

async fn get_image(
    State(state): State<Arc<AppState>>,
    Path((identifier, region_str, size_str, rotation_str, quality_format)): Path<(String, String, String, String, String)>,
) -> impl IntoResponse {
    let parts: Vec<&str> = quality_format.split('.').collect();
    if parts.len() != 2 {
        return (StatusCode::BAD_REQUEST, "Invalid quality.format").into_response();
    }
    let quality_str = parts[0];
    let format_str = parts[1];

    let region = parser::parse_region(&region_str);
    let size = parser::parse_size(&size_str);
    let rotation = parser::parse_rotation(&rotation_str);
    let quality = parser::parse_quality(quality_str);
    let format = parser::parse_format(format_str);

    if let (Some(region), Some(size), Some(rotation), Some(quality), Some(format)) = (region, size, rotation, quality, format) {
        let req = ImageRequest {
            identifier: identifier.clone(),
            region,
            size,
            rotation,
            quality,
            format,
        };

        let cache_params = format!("{}/{}/{}/{}.{}", region_str, size_str, rotation_str, quality_str, format_str);
        let cache_key = TileCache::get_key(&identifier, &cache_params);

        if let Some(cached_data) = state.cache.get(&cache_key).await {
            tracing::debug!("Cache hit for {}", cache_key);
            return (StatusCode::OK, [("content-type", format!("image/{}", format_str))], cached_data).into_response();
        }

        let base_id = if let Some((base, _)) = identifier.split_once(":page:") {
            base
        } else {
            &identifier
        };

        let file_path = format!("{}/{}", state.config.iiif.source_dir, base_id);
        if !std::path::Path::new(&file_path).exists() {
             return (StatusCode::NOT_FOUND, "Image not found").into_response();
        }

        match state.processor.process_image(&file_path, &req) {
            Ok(data) => {
                state.cache.set(&cache_key, data.clone()).await;
                (StatusCode::OK, [("content-type", format!("image/{}", format_str))], data).into_response()
            }
            Err(e) => {
                tracing::error!("Image processing error: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Image processing failed").into_response()
            }
        }
    } else {
        (StatusCode::BAD_REQUEST, "Invalid IIIF parameters").into_response()
    }
}
