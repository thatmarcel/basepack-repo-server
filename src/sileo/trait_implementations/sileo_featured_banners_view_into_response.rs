use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use crate::sileo::sileo_featured_banners_view::SileoFeaturedBannersView;

impl IntoResponse for SileoFeaturedBannersView {
    fn into_response(self) -> Response {
        match serde_json::to_string(&self) {
            Ok(sj) => sj.into_response(),
            Err(e) => {
                println!("{}", e);
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
        }
    }
}