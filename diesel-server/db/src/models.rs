use super::schema::posts;
use ::chrono::NaiveDateTime;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[derive(Queryable)]
pub struct Post {
    pub id: i32,
    pub timestamp: NaiveDateTime,
    pub author: String,
    pub body: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[derive(Insertable)]
#[table_name = "posts"]
pub struct NewPost<'a> {
    pub timestamp: NaiveDateTime,
    pub author: &'a str,
    pub body: &'a str,
}