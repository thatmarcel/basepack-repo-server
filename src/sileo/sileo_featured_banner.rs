use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct SileoFeaturedBanner {
    pub url: String,
    pub title: String,
    #[serde(rename = "package")]
    pub package_identifier: String,
    #[serde(rename = "hideShadow")]
    pub should_hide_shadow: bool
}