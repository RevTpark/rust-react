extern crate rocket;
extern crate lib;

use rocket::*;
use rocket::serde::json::Json;
use lib::*;
use self::model::*;


#[get("/all")]
fn get_all() -> (http::Status, Result<Json<Vec<User>>, Json<String>>) {
    match User::get_all() {
        Ok(value) => {
            (http::Status::Ok, Ok(Json(value)))
        },
        Err(error) =>{
            println!("{}", error);
            (http::Status::NotFound, Err(Json(String::new())))
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
            (http::Status::NotFound, Err(Json(String::new())))
        }
    }
}

#[post("/create", format="application/json", data="<data>")]
fn create_user(data: String) -> (http::Status, Result<Json<User>, Json<String>>){
    match User::add_user(data){
        Ok(value) => {
            (http::Status::Created, Ok(Json(value)))
        },
        Err(error) => {
            println!("{}", error);
            (http::Status::BadRequest, Err(Json(String::new())))
        }
    }
}

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build()
        .mount(
            "/users",
            routes![get_all, create_user, get_user_from_id]
        )
}