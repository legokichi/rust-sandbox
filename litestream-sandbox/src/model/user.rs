#[derive(Clone, serde::Serialize, serde::Deserialize, sqlx::FromRow, Debug, PartialEq, Eq)]
pub struct User {
    pub id: i64,
    pub github_id: Option<i64>,
    pub facebook_id: Option<i64>,
    pub created_at: i64,
    pub updated_at: i64,
}

impl axum_login::AuthUser for User {
    type Id = i64;

    fn id(&self) -> Self::Id {
        self.id
    }

    fn session_auth_hash(&self) -> &[u8] {
        // ヤケクソ
        unsafe {
            std::slice::from_raw_parts(
                &self.id as *const i64 as *const u8,
                std::mem::size_of::<i64>(),
            )
        }
    }
}
