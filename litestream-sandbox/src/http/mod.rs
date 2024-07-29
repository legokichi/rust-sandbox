mod point;
pub use point::*;

pub fn app() -> axum::Router<sqlx::sqlite::SqlitePool> {
    axum::Router::new()
        .route("/points", axum::routing::get(crate::http::list_points))
        .route("/points", axum::routing::post(crate::http::create_point))
        .route("/points/:point", axum::routing::get(crate::http::get_point))
        .route("/points/:point", axum::routing::patch(crate::http::update_point))
        .route("/points/:point", axum::routing::delete(crate::http::delete_point))
}

pub struct Ise(anyhow::Error);

impl axum::response::IntoResponse for Ise {
    fn into_response(self) -> axum::response::Response {
        (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
            .into_response()
    }
}

impl<E> From<E> for Ise
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
