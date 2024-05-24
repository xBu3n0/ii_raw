use diesel::{insert_into, ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use ii_raw::{
    establish_connection,
    models::User,
    schema::users::{self, dsl::*},
};

use rocket::{http::Status, serde::json::Json};

#[macro_use]
extern crate rocket;

#[get("/users")]
fn get_users() -> Json<Vec<User>> {
    let conn = &mut establish_connection();

    let qtd = users.load(conn).unwrap() as Vec<User>;

    Json(qtd)
}

#[post("/users", data = "<data>")]
fn create_user(data: Json<User>) -> (Status, Result<Json<User>, String>) {
    let conn = &mut establish_connection();

    let user: User = data.into();

    match insert_into(users)
        .values(user)
        .get_result::<User>(conn)
    {
        Ok(user) => (Status::Created, Ok(Json::from(user))),
        Err(e) => (Status::Conflict, Err(e.to_string()))
    }
}

#[get("/users/<index>")]
fn get_user(index: i32) -> (Status, Json<Result<User, String>>) {
    let conn = &mut establish_connection();

    match users::table.filter(id.eq(index)).select(User::as_select()).first(conn) {
        Ok(user) => (Status::Ok, Json(Ok(user))),
        Err(err) => (Status::NotFound, Json(Err(err.to_string())))
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/api/", routes![get_users, get_user, create_user])
}
