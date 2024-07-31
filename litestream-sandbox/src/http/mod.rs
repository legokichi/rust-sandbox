pub mod auth;
// mod point;
// mod oauth;
// pub use auth::*;
// pub use point::*;

#[derive(Clone)]
pub struct State {
    #[allow(dead_code)]
    pub db: sqlx::sqlite::SqlitePool,
}
impl State {
    pub fn from_pool(pool: sqlx::sqlite::SqlitePool) -> Result<Self, anyhow::Error> {
        Ok(Self { db: pool })
    }
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
