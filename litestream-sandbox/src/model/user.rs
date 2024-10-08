#[derive(Clone, serde::Serialize, serde::Deserialize, sqlx::FromRow, Debug, PartialEq, Eq)]
pub struct User {
    pub user_id: i64,
    pub github_id: Option<i64>,
    pub facebook_id: Option<i64>,
    pub created_at: i64,
    pub updated_at: i64,
}

impl axum_login::AuthUser for User {
    type Id = i64;

    fn id(&self) -> Self::Id {
        self.user_id
    }

    fn session_auth_hash(&self) -> &[u8] {
        // ヤケクソ
        unsafe {
            std::slice::from_raw_parts(
                &self.user_id as *const i64 as *const u8,
                std::mem::size_of::<i64>(),
            )
        }
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, PartialEq, Eq, sqlx::FromRow)]
pub struct AccessLog {
    pub access_log_id: i64,
    pub user_id: i64,
    pub request: String,
    pub created_at: i64,
}
