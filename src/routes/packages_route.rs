use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

use crate::data::items::package::Package;
use crate::data::packages::get_packages;
use crate::data::traits::into_kv::IntoKV;

pub async fn packages_route() -> Response {
    let packages: Vec<Package> = match get_packages().await {
        Some(p) => p,
        None => return StatusCode::INTERNAL_SERVER_ERROR.into_response()
    };

    packages.into_kv().into_response()
}