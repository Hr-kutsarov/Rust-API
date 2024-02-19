#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, this is me, rust!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
