//test
use diesel::{insert_into, ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use ii_raw::{
    establish_connection,
    models::User,
    schema::users::{self, dsl::*},
};

use rocket::{http::Status, serde::json::Json};

#[macro_use]
extern crate rocket;

#[catch(404)]
fn not_found() -> String {
    format!("404 - Not found")
}

#[catch(409)]
fn conflict() -> String {
    format!("409 - The user with this `id` already exists")
}

#[get("/users")]
// https://api.rocket.rs/master/rocket/response/status/
async fn get_users() -> Result<(Status, Json<Vec<User>>), Status> {
    let conn = &mut establish_connection();

    let qtd = users.load(conn).unwrap() as Vec<User>;

    Ok((Status::Ok, Json(qtd)))
}

#[post("/users", data = "<data>")]
async fn create_user(data: Json<User>) ->  Result<(Status, Json<User>), Status> {
    let conn = &mut establish_connection();

    let user: User = data.into();

    match insert_into(users)
        .values(user)
        .get_result::<User>(conn)
    {
        Ok(user) => Ok((Status::Created, Json::from(user))),
        Err(_) => Err(Status::Conflict)
    }
}

#[get("/users/<index>")]
async fn get_user(index: i32) -> Result<(Status, Json<User>), Status> {
    let conn = &mut establish_connection();

    match users::table.filter(id.eq(index)).select(User::as_select()).first(conn) {
        Ok(user) => Ok((Status::Ok, Json(user))),
        Err(_) => Err(Status::NotFound)
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
    .mount("/api/", routes![get_users, get_user, create_user])
    .register("/", catchers![not_found, conflict])
}
