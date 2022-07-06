extern crate rocket;
extern crate lib;

use rocket::*;
use rocket::serde::json::Json;
use lib::*;
use self::model::*;
use self::header::Token;


#[get("/all")]
fn get_all() -> (http::Status, Result<Json<Vec<User>>, Json<String>>) {
    match User::get_all() {
        Ok(value) => {
            (http::Status::Ok, Ok(Json(value)))
        },
        Err(error) =>{
            println!("{}", error);
            (http::Status::NotFound, Err(Json(get_empty_json())))
        }
    }
}

#[get("/<get_id>")]
fn get_user_from_id(get_id: i32) -> (http::Status, Result<Json<User>, Json<String>>){
    match User::from_id(get_id){
        Ok(value) => {
            (http::Status::Ok, Ok(Json(value)))
        },
        Err(error) => {
            println!("{}", error);
            (http::Status::NotFound, Err(Json(get_empty_json())))
        }
    }
}

#[post("/create", format="application/json", data="<data>")]
fn create_user(token: Token, data: String) -> (http::Status, Result<Json<User>, Json<String>>){
    match User::add_user(data){
        Ok(value) => {
            (http::Status::Created, Ok(Json(value)))
        },
        Err(error) => {
            println!("{}", error);
            (http::Status::BadRequest, Err(Json(get_empty_json())))
        }
    }
}

#[post("/update/<get_id>", format="application/json", data="<data>")]
fn update_user(data: String, get_id: i32) -> (http::Status, Result<Json<User>, Json<String>>){
    match User::update_user(data, get_id) {
        Ok(value) => {
            (http::Status::Ok, Ok(Json(value)))
        },
        Err(error) => {
            println!("{}", error);
            (http::Status::Ok, Err(Json(get_empty_json())))
        }
    }
}

#[get("/delete/<get_id>")]
fn delete_user(get_id: i32) -> http::Status{
    match User::delete_user(get_id) {
        Ok(_value) => {
            http::Status::NoContent
        },
        Err(error) => {
            println!("{}", error);
            http::Status::BadRequest
        }
    }
}

#[post("/login", format="application/json", data="<data>")]
fn login(data: String) -> (http::Status, Result<Json<User>, Json<String>>){
    match User::verify_user(data) {
        Ok(value) => {
            (http::Status::Ok, Ok(Json(value)))
        },
        Err(error) => {
            println!("{}", error);
            (http::Status::BadRequest, Err(Json(get_empty_json())))
        }
    }
}

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build()
        .mount(
            "/users",
            routes![get_all, create_user, get_user_from_id, update_user, delete_user]
        )
        .mount(
            "/auth",
            routes![login]
        )
}