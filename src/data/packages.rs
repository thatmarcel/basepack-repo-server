use crate::data::items::package::Package;
use crate::data::legacy_packages::get_legacy_packages;
use crate::data::traits::into_packages::IntoPackages;

pub async fn get_packages() -> Option<Vec<Package>> {
    match get_legacy_packages(false).await {
        Some(v) => Some(v.into_packages()),
        None => None
    }
}

pub async fn get_featured_packages() -> Option<Vec<Package>> {
    match get_legacy_packages(true).await {
        Some(v) => Some(v.into_packages()),
        None => None
    }
}