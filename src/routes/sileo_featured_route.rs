use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

use crate::data::items::package::Package;
use crate::data::packages::get_featured_packages;
use crate::sileo::sileo_featured_banner::SileoFeaturedBanner;
use crate::sileo::sileo_featured_banners_view::SileoFeaturedBannersView;
use crate::sileo::sileo_item_size::SileoItemSize;

pub async fn sileo_featured_route() -> Response {
    let featured_packages: Vec<Package> = match get_featured_packages().await {
        Some(p) => p,
        None => return StatusCode::INTERNAL_SERVER_ERROR.into_response()
    };
    
    let banners = featured_packages
        .iter()
        .map(|fp| {
            SileoFeaturedBanner {
                url: fp.featured_image_url.clone(),
                title: fp.name.clone(),
                package_identifier: fp.identifier.clone(),
                should_hide_shadow: false,
            }
        })
        .collect();

    SileoFeaturedBannersView {
        item_size: SileoItemSize {
            width: 263,
            height: 148
        },
        item_corner_radius: 10,
        banners
    }.into_response()
}