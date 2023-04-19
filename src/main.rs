#[macro_use]
extern crate rocket;

use chrono::Local;

#[get("/")]
fn index() -> String {
    let local_date = Local::now();
    let iso_date = local_date.format("%Y-%m-%d %H:%M:%S").to_string();
    format!("Hello, world! The time is {}", iso_date)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
