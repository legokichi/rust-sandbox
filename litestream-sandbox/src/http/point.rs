        // .route("/points", axum::routing::get(crate::http::list_points))
        // .route("/points", axum::routing::post(crate::http::create_point))
        // .route("/points/:point", axum::routing::get(crate::http::get_point))
        // .route(
        //     "/points/:point",
        //     axum::routing::patch(crate::http::update_point),
        // )
        // .route(
        //     "/points/:point",
        //     axum::routing::delete(crate::http::delete_point),
        // )

/// GET /points
pub async fn list_points(
    axum::extract::State(ref st): axum::extract::State<crate::http::State>,
    axum::extract::Query(ref query): axum::extract::Query<crate::model::PointsQuery>,
) -> Result<(axum::http::StatusCode, axum::Json<serde_json::Value>), crate::http::Ise> {
    let (rows, offest) = crate::db::list_points(&st.db, query).await?;
    let rows = crate::model::List {
        offset: Some(offest),
        list: rows,
    };
    let rows = serde_json::to_value(&rows)?;
    Ok((axum::http::StatusCode::OK, rows.into()))
}

/// POST /points
pub async fn create_point(
    axum::extract::State(ref st): axum::extract::State<crate::http::State>,
    axum::extract::Json(ref point): axum::extract::Json<crate::model::Point>,
) -> Result<(axum::http::StatusCode, axum::Json<serde_json::Value>), crate::http::Ise> {
    crate::db::create_point(&st.db, point).await?;
    Ok((axum::http::StatusCode::OK, serde_json::json!({}).into()))
}

/// GET /points/:point
pub async fn get_point(
    axum::extract::State(ref st): axum::extract::State<crate::http::State>,
    axum::extract::Path(ref id): axum::extract::Path<String>,
) -> Result<(axum::http::StatusCode, axum::Json<serde_json::Value>), crate::http::Ise> {
    let row = crate::db::get_point(&st.db, id).await?;
    match row {
        None => Ok((
            axum::http::StatusCode::NOT_FOUND,
            serde_json::json!({}).into(),
        )),
        Some(row) => {
            let row = serde_json::to_value(&row)?;
            Ok((axum::http::StatusCode::OK, row.into()))
        }
    }
}

/// PATCH /points/:point
pub async fn update_point(
    axum::extract::State(ref st): axum::extract::State<crate::http::State>,
    axum::extract::Path(ref id): axum::extract::Path<String>,
    axum::extract::Json(ref point): axum::extract::Json<crate::model::UpdatePoint>,
) -> Result<(axum::http::StatusCode, axum::Json<serde_json::Value>), crate::http::Ise> {
    crate::db::update_point(&st.db, id, point).await?;
    Ok((axum::http::StatusCode::OK, serde_json::json!({}).into()))
}

/// DELETE /points/:point
pub async fn delete_point(
    axum::extract::State(ref st): axum::extract::State<crate::http::State>,
    axum::extract::Path(ref id): axum::extract::Path<String>,
) -> Result<(axum::http::StatusCode, axum::Json<serde_json::Value>), crate::http::Ise> {
    crate::db::delete_point(&st.db, id).await?;
    Ok((axum::http::StatusCode::OK, serde_json::json!({}).into()))
}

#[cfg(test)]
mod tests {
    // use super::*;
    use http_body_util::BodyExt;

    #[sqlx::test]
    async fn test(pool: sqlx::sqlite::SqlitePool) {
        dotenvy::dotenv().ok();
        env_logger::builder().is_test(true).try_init().ok();
        use tower::ServiceExt;
        let st = crate::http::State::from_pool(pool).unwrap();
        let app = crate::http::app().with_state(st);
        let req = axum::http::Request::get("/points")
            .body(axum::body::Body::empty())
            .unwrap();
        let response = app.oneshot(req).await.unwrap();

        assert_eq!(response.status(), axum::http::StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        assert_eq!(
            std::str::from_utf8(&body[..]).unwrap(),
            r##"{"list":[],"offset":0}"##
        );
    }
}
