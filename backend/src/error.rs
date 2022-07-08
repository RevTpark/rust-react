use diesel::result::Error as DieselError;
use rocket::{http::Status};


pub struct CustomError{
    pub status_code: Status,
    pub message: String
}

impl CustomError {
    pub fn new(status_code: Status, message: String) -> CustomError {
        CustomError {
            status_code,
            message,
        }
    }
}

impl From<DieselError> for CustomError{
    fn from(error: DieselError) -> Self {
        match error {
            DieselError::DatabaseError(_, err) => CustomError::new(Status::Conflict, err.message().to_string()),
            DieselError::NotFound => CustomError::new(Status::NotFound, "User not found!".to_string()),
            err => CustomError::new(Status::InternalServerError, format!("Something went wrong: {}", err))
        }
    }
}