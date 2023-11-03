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

// TODO in Repo / persistance/ config? auslagern?
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
                controller::start_controller::root,
                controller::start_controller::is_time_for_beer,
                controller::beer_brand_controller::get_beer_brands,
                controller::beer_brand_controller::get_beer_brand,
                controller::beer_brand_controller::create_beer_brand,
                controller::beer_brand_controller::update_beer_brand,
                controller::beer_brand_controller::delete_beer_brand,
            ],
        ) // /beer_brand
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
