use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct SileoDepictionScreenshot {
    pub url: String,
    #[serde(rename = "accessibilityText")]
    pub accessibility_text: String,
    #[serde(rename = "video")]
    pub is_video: Option<bool>
}