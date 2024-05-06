use mongodb::bson::doc;
use serde::Deserialize;
use crate::data::items::legacy_package::LegacyPackage;
use crate::data::traits::into_package::IntoPackage;

#[derive(Deserialize, Clone, PartialEq)]
pub struct Package {
    pub identifier: String,
    pub size: i64,
    pub icon_url: String,
    pub package_name: String,
    pub long_description: String,
    pub short_description: String,
    pub name: String,
    pub is_approved: bool,
    pub author_name: String,
    pub section: String,
    pub screenshots: Vec<String>,
    pub hash_sha1: String,
    pub hash_md5: String,
    pub hash_sha256: String,
    pub dependencies: Vec<String>,
    pub header_image_url: String,
    pub version: String,
    pub download_url: String,
    pub is_featured: bool,
    pub author_email: String,
    pub min_os: i32,
    pub featured_image_url: String,
    pub price: i32,
    pub installed_size: i64,
    pub conflicts: Vec<String>,
    pub changelog: Option<Vec<PackageChangelogEntry>>
}

#[derive(Deserialize, Clone, PartialEq)]
pub struct PackageChangelogEntry {
    pub(crate) version: String,
    pub(crate) content: String
}

impl Package {
    pub async fn get(package_identifier: &str) -> Option<Package> {
        match LegacyPackage::get(package_identifier).await {
            Some(lp) => Some(lp.into_package()),
            None => None
        }
    }
    
    pub async fn record_package_download(package_identifier: &str) {
        LegacyPackage::record_download(package_identifier).await;
    }
    
    pub async fn record_download(self) {
        Self::record_package_download(&*self.identifier).await;
    }
}