use rocket::{request, request::FromRequest, request::Request, outcome::Outcome, http::Status};
use crate::establish_connection;
use crate::schema::users;
use diesel::prelude::*;
use crate::model::User;
use pwhash::unix;
use dotenv::dotenv;
use std::env;

#[derive(Debug)]
pub enum ApiTokenError {
    UserTokenMissing,
    UserTokenInvalid,
    GlobalTokenMissing,
    GlobalTokenInvalid
}

pub struct UserToken(String);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for UserToken {
    type Error = ApiTokenError;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let token = request.headers().get_one("Api-Key");
        match token {
            Some(token) => {
                let conn = establish_connection();
                match users::table.filter(users::api_key.eq(token.to_string())).first::<User>(&conn) {
                    Ok(_value) => {
                        Outcome::Success(UserToken(token.to_string()))
                    },
                    Err(_error) => {
                        Outcome::Failure((Status::Unauthorized, ApiTokenError::UserTokenInvalid))
                    }
                }
            },
            None => Outcome::Failure((Status::Unauthorized, ApiTokenError::UserTokenMissing))
        }
    }
}


pub struct GlobalToken(String);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for GlobalToken {
    type Error = ApiTokenError;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        dotenv().ok();

        let token = request.headers().get_one("Global-Api-Key");
        match token {
            Some(token) => {
                let global_key = env::var("KEY").expect("KEY must be set");
                if unix::verify(global_key, token){
                    Outcome::Success(GlobalToken(token.to_string()))
                }
                else{
                    Outcome::Failure((Status::Unauthorized, ApiTokenError::GlobalTokenInvalid))
                }
            },
            None => Outcome::Failure((Status::Unauthorized, ApiTokenError::GlobalTokenMissing))
        }
    }
}