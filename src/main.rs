#[macro_use]
extern crate rocket;
mod controller;
mod persistence;
mod service;

// fn main() {
// }
#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![controller::beer_controller::is_time_for_beer])
        .mount("/", routes![controller::beer_controller::root])
        .mount(
            "/drinks",
            routes![controller::drink_controller::get_all_drinks],
        )
}

/*
TODO
- use Json in Rest
- Tests
- database, evtl verschiedene postgres, mongodb
- ...



rustup default nightly --> wird für rocket gebraucht?!?!??!!!!
rustup default stable

docker run --name some-postgres -e POSTGRES_PASSWORD=mysecretpassword -d postgres
docker run --name funny-endpoints-postgres -e POSTGRES_PASSWORD=c1Rt3X66rGi5flJypblB -d postgres


*/
