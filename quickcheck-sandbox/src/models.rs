use chrono::NaiveDateTime;
use crate::schema::posts;

#[derive(Serialize, Deserialize, Debug, Clone, Queryable)]
pub struct Post {
    pub id: i32,
    pub timestamp: NaiveDateTime,
    pub author: String,
    pub body: String,
    pub like: i32,
    pub unlike: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Insertable)]
#[table_name = "posts"]
pub struct NewPost<'a> {
    pub timestamp: NaiveDateTime,
    pub author: &'a str,
    pub body: &'a str,
}

#[derive(Serialize, Deserialize, Debug, Clone, AsChangeset)]
#[table_name = "posts"]
pub struct UpdatePost {
    pub like: Option<i32>,
    pub unlike: Option<i32>,
}
