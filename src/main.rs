#[macro_use]
extern crate rocket;

use migration::MigratorTrait;
use pool::Db;
use rocket::{
    fairing::{self, AdHoc},
    Build, Rocket,
};
use sea_orm_rocket::Database;

mod controller;
mod persistence;
mod service;

mod pool;

async fn run_migrations(rocket: Rocket<Build>) -> fairing::Result {
    let conn = &Db::fetch(&rocket).unwrap().conn;
    let _ = migration::Migrator::up(conn, None).await;
    Ok(rocket)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Db::init())
        .attach(AdHoc::try_on_ignite("Migrations", run_migrations))
        .mount(
            "/",
            routes![
                controller::beer_controller::root,
                controller::beer_controller::is_time_for_beer,
                controller::beer_controller::create_beer_brand,
                controller::beer_controller::get_beer_brands,
                controller::beer_controller::update_beer_brand,
                controller::beer_controller::delete_beer_brand,
            ],
        )
        .mount(
            "/drinks",
            routes![controller::drink_controller::get_all_drinks],
        )
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
