use diesel::{insert_into, RunQueryDsl};
use ii_raw::{establish_connection, models::User, schema::users::dsl::*};

use rocket::serde::json::Json;

#[macro_use] extern crate rocket;

#[get("/", format="application/json", data="<data>")]
fn index(data: Json<User>) -> Json<Vec<User>> {
    let conn = &mut establish_connection();

    let _ = insert_into(users).values(User {
        id: data.id,
        nickname: data.nickname.clone(),
        password: data.password.clone()
    }).execute(conn);

    let qtd = users.load(conn).unwrap() as Vec<User>;

    Json(qtd)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
