use axum::response::Redirect;

pub async fn root_route() -> Redirect {
    Redirect::temporary(crate::repo_metadata::REPO_WEBSITE_URL)
}