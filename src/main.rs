#[macro_use]
extern crate rocket;

mod controller;
mod persistence;
mod service;

use rocket::fairing;
use rocket::fairing::AdHoc;
use rocket::Build;
use rocket::Rocket;
use rocket_db_pools::sqlx;
use rocket_db_pools::Database;

use sqlx::Postgres;

#[derive(Database)]
#[database("funny_endpoints")]
pub struct FunnyEndpointsDB(sqlx::Pool<Postgres>);

#[launch]
async fn rocket() -> _ {
    rocket::build()
        .attach(FunnyEndpointsDB::init())
        .attach(AdHoc::try_on_ignite("SQLx Migrations", run_migrations))
        .mount(
            "/",
            routes![
                controller::beer_controller::root,
                controller::beer_controller::is_time_for_beer,
                controller::beer_controller::get_beer_brands,
                controller::beer_controller::get_beer_brand,
                controller::beer_controller::create_beer_brand,
                controller::beer_controller::update_beer_brand,
                controller::beer_controller::delete_beer_brand,
            ],
        )
        .mount(
            "/drinks",
            routes![controller::drink_controller::get_all_drinks],
        )
}

async fn run_migrations(rocket: Rocket<Build>) -> fairing::Result {
    match FunnyEndpointsDB::fetch(&rocket) {
        Some(db) => match sqlx::migrate!("db/migrations").run(&**db).await {
            Ok(_) => Ok(rocket),
            Err(e) => {
                error!("Failed to initialize SQLx database: {}", e);
                Err(rocket)
            }
        },
        None => Err(rocket),
    }
}

/*
TODO

- Tests
- database, evtl verschiedene postgres, mongodb
- readme
- ...

docker run --name some-postgres -e POSTGRES_PASSWORD=mysecretpassword -d postgres
docker run --name funny-endpoints-postgres -e POSTGRES_PASSWORD=c1Rt3X66rGi5flJypblB -d postgres -p 5555:5432

*/
