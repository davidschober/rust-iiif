use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageInfo {
    #[serde(rename = "@context")]
    pub context: String,
    pub id: String,
    pub r#type: String,
    pub protocol: String,
    pub profile: String,
    pub width: u32,
    pub height: u32,
    pub extra_features: Vec<String>,
}

impl ImageInfo {
    pub fn new(id: String, width: u32, height: u32) -> Self {
        Self {
            context: "http://iiif.io/api/image/3/context.json".to_string(),
            id,
            r#type: "ImageService3".to_string(),
            protocol: "http://iiif.io/api/image".to_string(),
            profile: "level2".to_string(),
            width,
            height,
            extra_features: vec![
                "rotationArbitrary".to_string(),
                "mirroring".to_string(),
                "regionSquare".to_string(),
            ],
        }
    }
}
