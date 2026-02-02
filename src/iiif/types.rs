use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImageRequest {
    pub identifier: String,
    pub region: Region,
    pub size: Size,
    pub rotation: Rotation,
    pub quality: Quality,
    pub format: Format,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Region {
    Full,
    Square,
    // x,y,w,h
    Absolute(f64, f64, f64, f64),
    // pct:x,y,w,h
    Percentage(f64, f64, f64, f64),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Size {
    Max,
    ScaleAsFull, // ^max
    Width(u32),
    Height(u32),
    WidthHeight(u32, u32),
    Percentage(f64),
    WidthHeightMin(u32, u32), // !w,h
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Rotation {
    pub degrees: f64,
    pub mirror: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Quality {
    Default,
    Color,
    Gray,
    Bitonal,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Format {
    Jpg,
    Png,
    Tif,
    Webp,
    Gif,
    Pdf,
}
