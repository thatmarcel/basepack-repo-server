use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use crate::sileo::sileo_depiction_tab_view::SileoDepictionTabView;

impl IntoResponse for SileoDepictionTabView {
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