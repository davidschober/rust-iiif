# Rust IIIF Server

A high-performance IIIF Image API v3 server written in Rust, optimized foruse on a single VM. Inspired by `serverless-iiif` but ported to Rust and `libvips` for maximum efficiency.

## Features

- **IIIF Image API v3 compliant**: Supports standard IIIF URI patterns.
- **Fast Image Processing**: Leverages `libvips` for low-latency, low-memory transformations.
- **Supported Formats**: Pyramidal TIFF (optimized), TIFF, JPG, PNG, WebP.
- **PDF Support**: Dynamically extracts pages from PDFs using `:page:N` in the identifier (e.g., `my-doc.pdf:page:0`).
- **Remote Storage**: Supports fetching and caching images from S3-compatible or HTTP sources (like Petabox).
- **Two-Level Caching**:
    - **L1 (In-Memory)**: Fast access to frequently used tiles (using `moka`).
    - **L2 (Disk)**: Persistent cache for generated tiles to survive restarts.
- **Simple Setup**: Single configuration file (`config.toml`) and easy environment variable overrides.

## Prerequisites

- **libvips**: Must be installed on the host system.
  - macOS: `brew install vips`
  - Ubuntu/Debian: `sudo apt-get install libvips-dev`

## Installation & Setup

1. **Clone the repository.**
2. **Configure your server**: Create or edit `config.toml`.
   ```toml
   [server]
   port = 8080
   host = "0.0.0.0"

   [iiif]
   source_dir = "./images"  # Directory containing your source images
   base_url = "http://localhost:8080/iiif/3/"

   [cache]
   memory_limit = "512MB"   # Max RAM for tile cache
   disk_cache_dir = "./cache"
   disk_limit = "10GB"

   # (Optional) Remote S3/HTTP storage
   [remote]
   base_url = "https://s3.amazonaws.com/my-bucket/"
   local_proxy_dir = "./remote_proxy"
   ```
3. **Run the server**:
   - On macOS (via Homebrew): Use the provided helper script:
     ```bash
     ./run.sh
     ```
   - On other systems:
     ```bash
     cargo run --release
     ```

## Usage

Place your images in the `images` directory. Access them via:
`http://localhost:8080/iiif/3/{identifier}/{region}/{size}/{rotation}/{quality}.{format}`

**Example**:
`http://localhost:8080/iiif/3/test.tif/full/max/0/default.jpg`

**PDF Example**:
`http://localhost:8080/iiif/3/real.pdf:page:0/full/512,/0/default.jpg`

## Architecture

- **Web Server**: [Axum](https://github.com/tokio-rs/axum)
- **Image Processing**: [libvips-rs](https://github.com/chandanpasunoori/libvips-rust-bindings)
- **Caching**: [Moka](https://github.com/moka-rs/moka) for L1, Local disk for L2.
