use axum::response::{IntoResponse, Response};

pub async fn cydia_icon_route() -> Response {
    include_bytes!("../resources/CydiaIcon.png").into_response()
}