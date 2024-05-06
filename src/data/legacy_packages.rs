use futures::TryStreamExt;
use mongodb::bson::doc;
use mongodb::Cursor;

use crate::data::items::legacy_package::LegacyPackage;
use crate::helpers::database::get_database;

pub async fn get_legacy_packages(featured_only: bool) -> Option<Vec<LegacyPackage>> {
    let database = get_database().await;
    
    let query = match featured_only {
        true => doc! {
            "state": "released",
            "isFeatured": true
        },
        false => doc! {
            "state": "released"
        }
    };

    let cursor: Cursor<LegacyPackage> = match database.collection::<LegacyPackage>("packages").find(query, None).await {
        Ok(r) => r,
        Err(e) => {
            println!("{}", e);
            return None;
        }
    };

    let results: Vec<LegacyPackage> = match cursor.try_collect().await {
        Ok(r) => r,
        Err(e) => {
            println!("{}", e);
            return None;
        }
    };

    Some(results)
}