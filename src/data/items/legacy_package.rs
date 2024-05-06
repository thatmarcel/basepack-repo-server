use futures::StreamExt;
use mongodb::bson::{doc, Document};
use serde::Deserialize;

use crate::helpers::mongo_database::get_mongo_database;

#[derive(Deserialize, Clone, PartialEq)]
pub struct LegacyPackage {
    #[serde(rename = "id")]
    pub identifier: String,
    pub size: i64,
    #[serde(rename = "icon")]
    pub icon_url: String,
    #[serde(rename = "sha1")]
    pub hash_sha1: String,
    #[serde(rename = "packageName")]
    pub package_name: String,
    #[serde(rename = "longDescription")]
    pub long_description: String,
    pub name: String,
    pub state: String,
    pub approved: String /* yes / no */,
    #[serde(rename = "authorName")]
    pub author_name: String,
    pub section: String,
    pub screenshots: Vec<String>,
    #[serde(rename = "md5")]
    pub hash_md5: String,
    pub dependencies: Vec<String>,
    #[serde(rename = "headerImage")]
    pub header_image_url: String,
    #[serde(rename = "sha256")]
    pub hash_sha256: String,
    pub version: String,
    #[serde(rename = "downloadURL")]
    pub download_url: String,
    #[serde(default)]
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(default)]
    pub is_featured: bool,
    #[serde(rename = "authorEmail")]
    pub author_email: String,
    #[serde(rename = "minOS")]
    pub min_os: i32,
    #[serde(default)]
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    #[serde(rename = "featuredImage")]
    pub featured_image_url: String,
    #[serde(rename = "ownerUid")]
    pub owner_uid: String,
    #[serde(rename = "description")]
    pub short_description: String,
    pub price: i32,
    #[serde(rename = "installedSize")]
    pub installed_size: Option<i64>,
    #[serde(rename = "coOwners")]
    pub co_owners: Option<String>,
    #[serde(default)]
    pub conflicts: Vec<String>,
    #[serde(rename = "downloadsSinceLastUpdate")]
    pub downloads_since_last_update: i64,
    pub downloads: i64,
    pub changelog: Option<Vec<LegacyPackageChangelogEntry>>
}

#[derive(Deserialize, Clone, PartialEq)]
pub struct LegacyPackageChangelogEntry {
    pub(crate) version: String,
    pub(crate) content: String
}

impl LegacyPackage {
    pub async fn get(package_identifier: &str) -> Option<LegacyPackage> {
        let database = get_mongo_database().await;

        let mut cursor = match database.collection::<LegacyPackage>("packages").find(doc! {
            "id": package_identifier
        }, None).await {
            Ok(cursor) => cursor,
            Err(e) => {
                println!("{}", e);
                return None;
            }
        };

        match cursor.next().await {
            Some(r) => match r {
                Ok(lp) => Some(lp),
                Err(e) => {
                    println!("{}", e);
                    return None;
                }
            },
            None => {
                return None;
            }
        }
    }

    pub async fn record_download(package_identifier: &str) {
        let database = get_mongo_database().await;
        
        match database.collection::<Document>("packages").update_one(
            doc! {
                "id": package_identifier
            },
            doc! {
                "$inc": {
                    "downloadsSinceLastUpdate": 1,
                    "downloads": 1
                }
            },
            None
        ).await {
            Ok(_) => {},
            Err(e) => {
                println!("{}", e);
            }
        }
    }
}