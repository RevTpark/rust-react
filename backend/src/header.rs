use rocket::{request, request::FromRequest, request::Request, outcome::Outcome, http::Status};
use crate::establish_connection;
use crate::schema::users;
use diesel::prelude::*;
use crate::model::User;


pub struct Token(String);

#[derive(Debug)]
pub enum ApiTokenError {
    UserTokenMissing,
    UserTokenInvalid,
    GlobalTokenMissing,
    GlobalTokenInvalid
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Token {
    type Error = ApiTokenError;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let token = request.headers().get_one("Api-Key");
        match token {
            Some(token) => {
                let conn = establish_connection();
                match users::table.filter(users::api_key.eq(token.to_string())).first::<User>(&conn) {
                    Ok(_value) => {
                        Outcome::Success(Token(token.to_string()))
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