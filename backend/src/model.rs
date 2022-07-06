use rocket::{serde::Serialize, serde::Deserialize, http};
use crate::establish_connection;
use crate::schema::users::{self, dsl::*};
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use diesel::{insert_into, update, delete};
use pwhash::{bcrypt, unix};
use crate::error::CustomError;

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
pub struct UpdateUser{
    pub name: String,
    pub role: String,
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

    pub fn get_all() -> Result<Vec<Self>, CustomError>{
        let conn = establish_connection();
        let results = users.load(&conn)?;
        Ok(results)
    }

    pub fn from_id(other_id: i32) -> Result<Self, CustomError>{
        let conn = establish_connection();
        let user = users::table.filter(users::id.eq(other_id)).first(&conn)?;
        Ok(user)
    }

    pub fn add_user(data: String) -> Result<Self, CustomError>{
        let conn = establish_connection();
        let mut new_user: NewUser = serde_json::from_str(&*data).unwrap();
        new_user.password = bcrypt::hash(new_user.password).unwrap();
        new_user.api_key = bcrypt::hash(format!("{}{}", new_user.name, new_user.email)).unwrap();
        let inserted_user = insert_into(users).values(new_user).get_result(&conn)?;
        Ok(inserted_user)
    }

    pub fn update_user(data: String, other_id: i32) -> Result<Self, CustomError>{
        let conn = establish_connection();
        let user: UpdateUser = serde_json::from_str(&*data).unwrap();
        let updated_user = update(users).filter(users::id.eq(other_id)).set(user).get_result(&conn)?;
        Ok(updated_user)
    }

    pub fn delete_user(other_id: i32) -> Result<usize, DieselError>{
        let conn = establish_connection();
        let result: Result<usize, DieselError> = delete(users).filter(users::id.eq(other_id)).execute(&conn);
        result
    }

    pub fn verify_user(data: String) -> Result<User, CustomError>{
        let conn = establish_connection();
        let creds: LoginUser = serde_json::from_str(&*data).unwrap();
        let user = users::table.filter(users::email.eq(creds.email)).first::<User>(&conn)?;

        if unix::verify(creds.password, &user.password) {
            Ok(user)
        }
        else{
            Err(CustomError::new(http::Status::Unauthorized, "Invalid Credentials Provided!".to_string()))
        }
    }
}
