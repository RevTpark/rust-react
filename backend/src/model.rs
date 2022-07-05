use rocket::serde::{Serialize, Deserialize};
use crate::establish_connection;
use crate::schema::users::{self, dsl::*};
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use diesel::insert_into;

#[derive(Deserialize, Insertable)]
#[table_name = "users"]
pub struct NewUser{
    pub name: String,
    pub role: String,
    pub email: String,
    pub password: String
}

#[derive(Debug, Serialize, Queryable)]
pub struct User{
    pub id: i32,
    pub name: String,
    pub role: String,
    pub email: String,
    pub password: String
}


impl User{

    pub fn get_all() -> Result<Vec<Self>, DieselError>{
        let conn = establish_connection();
        let results: Result<Vec<User>, DieselError> = users.load::<User>(&conn);
        results
    }

    pub fn from_id(other_id: i32) -> Result<User, DieselError>{
        let conn = establish_connection();
        let user: Result<User, DieselError> = users::table.filter(users::id.eq(other_id)).first(&conn);
        user
    }

    pub fn add_user(data: String) -> Result<User, DieselError>{
        let conn = establish_connection();
        let new_user: NewUser = serde_json::from_str(&*data).unwrap();
        let inserted_user: Result<User, DieselError> = insert_into(users).values(new_user).get_result(&conn);
        inserted_user
    }
}