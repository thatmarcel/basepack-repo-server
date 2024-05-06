use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Redirect, Response};
use crate::data::items::package::Package;

pub async fn download_route(Path(package_file_name): Path<String>) -> Response {
    // Remove ".deb" file extension
    let mut package_identifier = package_file_name;
    package_identifier.pop();
    package_identifier.pop();
    package_identifier.pop();
    package_identifier.pop();
    
    let package = Package::get(&*package_identifier).await;

    match package {
        Some(p) => {
            let p2 = p.clone();
            tokio::spawn(async move {
                _ = &p2.record_download().await;
            });
            
            Redirect::temporary(p.download_url.as_str()).into_response()
        },
        None => StatusCode::NOT_FOUND.into_response()
    }
}