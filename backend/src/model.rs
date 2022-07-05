use rocket::{serde::Serialize, serde::Deserialize, request::FromRequest, Request, request};
use crate::establish_connection;
use crate::schema::users::{self, dsl::*};
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use diesel::{insert_into, update, delete};
use pwhash::{bcrypt, unix};
use rocket::http::Status;
use rocket::outcome::Outcome;

#[derive(Deserialize, Insertable, AsChangeset)]
#[table_name = "users"]
pub struct NewUser{
    pub name: String,
    pub role: String,
    pub email: String,
    pub password: String,
    pub api_key: String
}

#[derive(Deserialize, Insertable, AsChangeset)]
#[table_name = "users"]
pub struct LoginUser{
    pub email: String,
    pub password: String
}

#[derive(Debug, Serialize, Queryable)]
pub struct User{
    pub id: i32,
    pub name: String,
    pub role: String,
    pub email: String,
    pub password: String,
    pub api_key: String
}


impl User{

    pub fn get_all() -> Result<Vec<Self>, DieselError>{
        let conn = establish_connection();
        let results: Result<Vec<User>, DieselError> = users.load(&conn);
        results
    }

    pub fn from_id(other_id: i32) -> Result<Self, DieselError>{
        let conn = establish_connection();
        let user: Result<User, DieselError> = users::table.filter(users::id.eq(other_id)).first(&conn);
        user
    }

    pub fn add_user(data: String) -> Result<Self, DieselError>{
        let conn = establish_connection();
        let mut new_user: NewUser = serde_json::from_str(&*data).unwrap();
        new_user.password = bcrypt::hash(new_user.password).unwrap();
        let inserted_user: Result<User, DieselError> = insert_into(users).values(new_user).get_result(&conn);
        inserted_user
    }

    pub fn update_user(data: String, other_id: i32) -> Result<Self, DieselError>{
        let conn = establish_connection();
        let user: NewUser = serde_json::from_str(&*data).unwrap();
        let updated_user:Result<User, DieselError> = update(users).filter(users::id.eq(other_id)).set(user).get_result(&conn);
        updated_user
    }

    pub fn delete_user(other_id: i32) -> Result<usize, DieselError>{
        let conn = establish_connection();
        let result: Result<usize, DieselError> = delete(users).filter(users::id.eq(other_id)).execute(&conn);
        result
    }

    pub fn verify_user(data: String) -> Result<User, DieselError>{
        let conn = establish_connection();
        let creds: LoginUser = serde_json::from_str(&*data).unwrap();
        match users::table.filter(users::email.eq(creds.email)).first::<User>(&conn) {
            Ok(user) => {
                if unix::verify(creds.password, &user.password) {
                    Ok(user)
                }
                else{
                    Err(DieselError::NotFound)
                }
            },
            Err(error) => {
                Err(DieselError::NotFound)
            }
        }
    }
}

// struct Token(String);
//
// #[derive(Debug)]
// enum ApiTokenError {
//     Missing,
//     Invalid,
// }
//
// impl<'a, 'r> FromRequest<'a, 'r> for Token {
//     type Error = ApiTokenError;
//
//     fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
//         let token = request.headers().get_one("token");
//         match token {
//             Some(token) => {
//                 Outcome::Success(Token(token.to_string()))
//             },
//
//             None => Outcome::Failure((Status::Unauthorized, ApiTokenError::Missing))
//         }
//     }
// }