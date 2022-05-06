use serde::{Deserialize, Serialize};

#[derive(Queryable, Debug, Serialize, Deserialize)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
}

use super::schema::posts;

#[derive(Insertable, Debug, Serialize, Deserialize)]
#[table_name = "posts"]
pub struct NewPost {
    pub title: String,
    pub body: String,
}
