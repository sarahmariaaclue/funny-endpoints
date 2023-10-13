#[macro_use]
extern crate rocket;

use rocket_db_pools::sqlx::{self, Row};
use rocket_db_pools::{Connection, Database};
use serde::{Deserialize, Serialize};

mod controller;
mod persistence;
mod service;

#[derive(Database)]
#[database("funny_endpoints")]
pub struct FunnyEndpointsDatabase(sqlx::PgPool);

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(FunnyEndpointsDatabase::init())
        .mount(
            "/",
            routes![
                controller::beer_controller::root,
                controller::beer_controller::is_time_for_beer,
                controller::beer_controller::create_beer_brand,
                controller::beer_controller::get_beer_brands
            ],
        )
        .mount(
            "/drinks",
            routes![controller::drink_controller::get_all_drinks],
        )
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct BeerBrandEntity {
    id: i64,
    name: String,
    city: String,
}

/*
TODO

- Tests
- database, evtl verschiedene postgres, mongodb
- ...



rustup default nightly --> wird für rocket gebraucht?!?!??!!!!
rustup default stable

docker run --name some-postgres -e POSTGRES_PASSWORD=mysecretpassword -d postgres
docker run --name funny-endpoints-postgres -e POSTGRES_PASSWORD=c1Rt3X66rGi5flJypblB -d postgres -p 5555:5432


*/
