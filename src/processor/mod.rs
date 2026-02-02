pub mod pdf;

use libvips_rs::{ops, VipsImage, VipsApp};
use crate::iiif::types::*;
use std::sync::Once;

static START: Once = Once::new();

pub struct ImageProcessor {
    _app: VipsApp,
}

impl ImageProcessor {
    pub fn new() -> Self {
        START.call_once(|| {
            // Initialization
        });
        Self {
            _app: VipsApp::new("iiif-processor", false).expect("Failed to init libvips"),
        }
    }

    pub fn get_image_size(&self, path: &str, identifier: &str) -> Result<(i32, i32), libvips_rs::error::Error> {
        let img = if path.to_lowercase().ends_with(".pdf") {
            if let Some((_, page_str)) = identifier.split_once(":page:") {
                let page: i32 = page_str.parse().unwrap_or(0);
                pdf::load_pdf_page(path, page)?
            } else {
                pdf::load_pdf_page(path, 0)?
            }
        } else {
            VipsImage::new_from_file(path)?
        };
        Ok((img.get_width(), img.get_height()))
    }

    pub fn process_image(&self, path: &str, req: &ImageRequest) -> Result<Vec<u8>, libvips_rs::error::Error> {
        let img = if path.to_lowercase().ends_with(".pdf") {
            if let Some((_, page_str)) = req.identifier.split_once(":page:") {
                let page: i32 = page_str.parse().unwrap_or(0);
                pdf::load_pdf_page(path, page)?
            } else {
                pdf::load_pdf_page(path, 0)?
            }
        } else {
            VipsImage::new_from_file(path)?
        };
        
        // 1. Region
        let img = match req.region {
            Region::Full => img,
            Region::Square => {
                let size = img.get_width().min(img.get_height());
                let x = (img.get_width() - size) / 2;
                let y = (img.get_height() - size) / 2;
                ops::extract_area(&img, x, y, size, size)?
            },
            Region::Absolute(x, y, w, h) => ops::extract_area(&img, x as i32, y as i32, w as i32, h as i32)?,
            Region::Percentage(px, py, pw, ph) => {
                let x = (img.get_width() as f64 * px / 100.0) as i32;
                let y = (img.get_height() as f64 * py / 100.0) as i32;
                let w = (img.get_width() as f64 * pw / 100.0) as i32;
                let h = (img.get_height() as f64 * ph / 100.0) as i32;
                ops::extract_area(&img, x, y, w, h)?
            }
        };

        // 2. Size
        let img = match req.size {
            Size::Max => img,
            Size::Width(w) => {
                let scale = w as f64 / img.get_width() as f64;
                ops::resize(&img, scale)?
            },
            Size::Height(h) => {
                let scale = h as f64 / img.get_height() as f64;
                ops::resize(&img, scale)?
            },
            Size::WidthHeight(w, h) => {
                let h_scale = h as f64 / img.get_height() as f64;
                let w_scale = w as f64 / img.get_width() as f64;
                ops::resize_with_opts(&img, w_scale, &ops::ResizeOptions {
                    vscale: h_scale,
                    ..Default::default()
                })?
            },
            _ => img 
        };

        // 3. Rotation
        let img = if req.rotation.mirror {
            ops::flip(&img, ops::Direction::Horizontal)?
        } else {
            img
        };
        let img = if req.rotation.degrees != 0.0 {
            ops::rotate(&img, req.rotation.degrees)?
        } else {
            img
        };

        // 4. Quality
        let img = match req.quality {
            Quality::Default | Quality::Color => img,
            Quality::Gray => ops::colourspace(&img, ops::Interpretation::BW)?,
            Quality::Bitonal => {
                let gray = ops::colourspace(&img, ops::Interpretation::BW)?;
                let mut threshold = [128.0];
                ops::relational_const(&gray, ops::OperationRelational::More, &mut threshold)?
            }
        };

        // 5. Format and Output
        match req.format {
            Format::Jpg => ops::jpegsave_buffer(&img),
            Format::Png => ops::pngsave_buffer(&img),
            Format::Webp => ops::webpsave_buffer(&img),
            _ => ops::jpegsave_buffer(&img)
        }
    }
}
