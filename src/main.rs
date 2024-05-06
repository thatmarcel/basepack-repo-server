use std::env;

use axum::Router;
use axum::routing::get;
use tokio::net::TcpListener;

use crate::helpers::database::initialize_database;
use crate::routes::cydia_icon_route::cydia_icon_route;
use crate::routes::download_route::download_route;
use crate::routes::packages_bz2_route::packages_bz2_route;
use crate::routes::packages_route::packages_route;
use crate::routes::release_route::release_route;
use crate::routes::root_route::root_route;
use crate::routes::sileo_depiction_route::sileo_depiction_route;
use crate::routes::sileo_featured_route::sileo_featured_route;
use crate::routes::status_route::status_route;

mod routes;
mod helpers;
mod data;
mod macros;
mod sileo;

#[tokio::main]
async fn main() {
    initialize_database().await;
    
    let app = Router::new()
        .route("/", get(root_route))
        .route("/status", get(status_route))
        .route("/sileo-featured.json", get(sileo_featured_route))
        .route("/sileo-depictions/:package_identifier", get(sileo_depiction_route))
        .route("/Release", get(release_route))
        .route("/./Release", get(release_route))
        .route("/Packages", get(packages_route))
        .route("/./Packages", get(packages_route))
        .route("/Packages.bz2", get(packages_bz2_route))
        .route("/./Packages.bz2", get(packages_bz2_route))
        .route("/download/:package_file_name", get(download_route))
        .route("/./download/:package_file_name", get(download_route))
        .route("/CydiaIcon.png", get(cydia_icon_route))
        .route("/./CydiaIcon.png", get(cydia_icon_route));

    let listener = TcpListener::bind(
        format!("0.0.0.0:{}", env::var("PORT").unwrap_or("4000".to_string()))
    ).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}