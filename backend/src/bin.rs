extern crate rocket;
extern crate lib;

use rocket::*;
use rocket::serde::json::Json;
use serde_json::Value;
use lib::*;
use lib::header::GlobalToken;
use self::model::*;
use self::header::UserToken;
use std::env;

#[catch(500)]
fn internal_error() -> Json<Value> {
    Json(get_json("Whoops! There was an Internal Server Error".to_string()))
}

#[catch(401)]
fn unauthorized_error() -> Json<Value>{
    Json(get_json("User Authorization Required!".to_string()))
}

#[catch(404)]
fn invalid_route() -> Json<Value>{
    Json(get_json("Route Does not Exists!".to_string()))
}

#[get("/all")]
fn get_all(_token1: GlobalToken) -> (http::Status, Result<Json<Vec<User>>, Json<Value>>) {
    match User::get_all() {
        Ok(value) => {
            (http::Status::Ok, Ok(Json(value)))
        },
        Err(error) =>{
            (error.status_code, Err(Json(get_json(error.message))))
        }
    }
}

#[get("/<get_id>")]
fn get_user_from_id(_token1: GlobalToken, get_id: i32) -> (http::Status, Result<Json<User>, Json<Value>>){
    match User::from_id(get_id){
        Ok(value) => {
            (http::Status::Ok, Ok(Json(value)))
        },
        Err(error) => {
            (error.status_code, Err(Json(get_json(error.message))))
        }
    }
}

#[post("/create", format="application/json", data="<data>")]
fn create_user(_token1: GlobalToken, _token2: UserToken, data: String) -> (http::Status, Result<Json<User>, Json<Value>>){
    match User::add_user(data){
        Ok(value) => {
            (http::Status::Created, Ok(Json(value)))
        },
        Err(error) => {
            (error.status_code, Err(Json(get_json(error.message))))
        }
    }
}

#[post("/update/<get_id>", format="application/json", data="<data>")]
fn update_user(_token1: GlobalToken, _token2: UserToken, data: String, get_id: i32) -> (http::Status, Result<Json<User>, Json<Value>>){
    match User::update_user(data, get_id) {
        Ok(value) => {
            (http::Status::Ok, Ok(Json(value)))
        },
        Err(error) => {
            (error.status_code, Err(Json(get_json(error.message))))
        }
    }
}

#[get("/delete/<get_id>")]
fn delete_user(_token1: GlobalToken, _token2: UserToken, get_id: i32) -> http::Status{
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
fn login(_token1: GlobalToken, data: String) -> (http::Status, Result<Json<User>, Json<Value>>){
    match User::verify_user(data) {
        Ok(value) => {
            (http::Status::Ok, Ok(Json(value)))
        },
        Err(error) => {
            (error.status_code, Err(Json(get_json(error.message))))
        }
    }
}

#[get("/logout")]
fn logout(_token1: GlobalToken, _token2: UserToken) -> Json<Value>{
    Json(get_json("Logout Success".to_string()))
}

#[get("/all")]
fn get_all_products(_token1: GlobalToken) -> (http::Status, Result<Json<Vec<DisplayProduct>>, Json<Value>>) {
    match Products::get_all() {
        Ok(value) => {
            (http::Status::Ok, Ok(Json(value)))
        },
        Err(error) =>{
            (error.status_code, Err(Json(get_json(error.message))))
        }
    }
}

#[get("/<get_id>")]
fn get_product_from_id(_token1: GlobalToken, get_id: i32) -> (http::Status, Result<Json<DisplayProduct>, Json<Value>>) {
    match Products::from_id(get_id) {
        Ok(value) => {
            (http::Status::Ok, Ok(Json(value)))
        },
        Err(error) =>{
            (error.status_code, Err(Json(get_json(error.message))))
        }
    }
}

#[post("/create", format="application/json", data="<data>")]
fn create_product(_token1: GlobalToken, _token2: UserToken, data: String) -> (http::Status, Result<Json<Products>, Json<Value>>){
    match Products::add_product(data){
        Ok(value) => {
            (http::Status::Created, Ok(Json(value)))
        },
        Err(error) => {
            (error.status_code, Err(Json(get_json(error.message))))
        }
    }
}

#[post("/update/<get_id>", format="application/json", data="<data>")]
fn update_product(_token1: GlobalToken, _token2: UserToken, get_id: i32, data: String) -> (http::Status, Result<Json<Products>, Json<Value>>){
    match Products::update_product(data, get_id){
        Ok(value) => {
            (http::Status::Created, Ok(Json(value)))
        },
        Err(error) => {
            (error.status_code, Err(Json(get_json(error.message))))
        }
    }
}

#[get("/delete/<get_id>")]
fn delete_product(_token1: GlobalToken, _token2: UserToken, get_id: i32) -> http::Status{
    match Products::delete_product(get_id) {
        Ok(_value) => {
            http::Status::NoContent
        },
        Err(error) => {
            println!("{}", error);
            http::Status::BadRequest
        }
    }
}

#[get("/search/<query>")]
fn search_products_name(_token1: GlobalToken, query: String) -> (http::Status, Result<Json<Vec<DisplayProduct>>, Json<Value>>){
    match Products::search_by_name(query) {
        Ok(value) => {
            (http::Status::Ok, Ok(Json(value)))
        },
        Err(error) =>{
            (error.status_code, Err(Json(get_json(error.message))))
        }
    }
}

#[get("/search/creator/<get_id>")]
fn search_products_creator_id(_token1: GlobalToken, get_id: i32) -> (http::Status, Result<Json<Vec<DisplayProduct>>, Json<Value>>){
    match Products::filter_by_creator(get_id) {
        Ok(value) => {
            (http::Status::Ok, Ok(Json(value)))
        },
        Err(error) =>{
            (error.status_code, Err(Json(get_json(error.message))))
        }
    }
}

#[get("/search/creator/name/<query>")]
fn search_products_creator_name(_token1: GlobalToken, query: String) -> (http::Status, Result<Json<Vec<DisplayProduct>>, Json<Value>>){
    match Products::filter_by_creator_name(query) {
        Ok(value) => {
            (http::Status::Ok, Ok(Json(value)))
        },
        Err(error) =>{
            (error.status_code, Err(Json(get_json(error.message))))
        }
    }
}

#[options("/<_..>")]
fn all_options() {
    /* Intentionally left empty */
}

#[launch]
fn rocket() -> Rocket<Build> {

    let default_port = 8000;
    let port: u64 = env::var("PORT")
        .and_then(|port| Ok(port.parse::<u64>().unwrap_or(default_port)))
        .unwrap_or(default_port);

    let config = Config::figment().merge(("port", port));

    rocket::custom(config)
        .mount(
            "/",
            routes![all_options]
        )
        .mount(
            "/users",
            routes![get_all, create_user, get_user_from_id, update_user, delete_user]
        )
        .mount(
            "/products",
            routes![get_all_products, create_product, get_product_from_id, update_product, delete_product, search_products_name, search_products_creator_id, search_products_creator_name]
        )
        .mount(
            "/auth",
            routes![login, logout]
        )
        .register(
            "/",
            catchers![internal_error, unauthorized_error, invalid_route]
        )
        .attach(CORS)
        .attach(shield::Shield::new())
}