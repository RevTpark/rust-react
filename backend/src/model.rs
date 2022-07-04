use rocket::serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Queryable)]
pub struct User{
    pub id: i32,
    pub name: String,
    pub role: String,
    pub email: String,
    pub password: String
}
