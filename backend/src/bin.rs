extern crate rocket;
extern crate lib;

use rocket::*;
use rocket::serde::json::Json;
use lib::*;
use self::model::*;
use diesel::prelude::*;


#[get("/")]
fn hello_world() -> Json<Vec<User>> {
    use self::schema::users::dsl::*;

    let connection = establish_connection();
    let results = users.load::<User>(&connection).expect("Error Loading users");
    // for user in results {
    //     println!("{}", user.name);
    // }
    Json(results)
}

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build()
        .mount(
            "/",
            routes![hello_world]
        )
}