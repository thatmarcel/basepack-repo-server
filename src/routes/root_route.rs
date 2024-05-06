use axum::response::Redirect;

pub async fn root_route() -> Redirect {
    Redirect::temporary("https://basepack.co")
}