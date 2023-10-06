#[macro_use]
extern crate rocket;
mod controller;
mod service;

// fn main() {
// }
#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![controller::beer_controller::is_time_for_beer])
}
