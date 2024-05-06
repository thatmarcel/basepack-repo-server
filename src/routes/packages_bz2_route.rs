use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

use crate::data::items::package::Package;
use crate::data::packages::get_packages;
use crate::data::traits::bzip2_compress::Bzip2Compress;
use crate::data::traits::into_kv::IntoKV;

pub async fn packages_bz2_route() -> Response {
    let packages: Vec<Package> = match get_packages().await {
        Some(p) => p,
        None => return StatusCode::INTERNAL_SERVER_ERROR.into_response()
    };

    let kv_string = packages.into_kv();

    match kv_string.compress_via_bzip2() {
        Some(c) => c.into_response(),
        None => StatusCode::INTERNAL_SERVER_ERROR.into_response()
    }
}