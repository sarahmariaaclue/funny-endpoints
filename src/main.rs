use controller::beer_controller::BeerBrand;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPoolOptions;

#[macro_use]
extern crate rocket;

mod controller;
mod persistence;
mod service;

// #[launch]
// fn rocket() -> _ {
//     rocket::build()
//         .mount(
//             "/",
//             routes![
//                 controller::beer_controller::root,
//                 controller::beer_controller::is_time_for_beer,
//                 controller::beer_controller::create_beer_brand,
//                 controller::beer_controller::get_beer_brands
//             ],
//         )
//         .mount(
//             "/drinks",
//             routes![controller::drink_controller::get_all_drinks],
//         )
// }

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Unable to load environment variables from .env file");

    let db_url = std::env::var("DATABASE_URL").expect("Unable to read DATABASE_URL env var");

    let pool = PgPoolOptions::new()
        .max_connections(100)
        .connect(&db_url)
        .await
        .expect("Unable to connect to Postgres");

    let inserted: Vec<BeerBrandEntity> = sqlx::query_as!(
        BeerBrandEntity,
        "insert into beer_brand(id, name, city) values ($1, $2, $3) returning *",
        3, // TODO auto id??
        "Ratsherrn",
        "Hamburg"
    )
    .fetch_all(&pool)
    .await
    .expect("Unable to insert a domain");

    println!("inserted: {:?}", inserted);

    let beer_brands: Vec<BeerBrandEntity> =
        sqlx::query_as!(BeerBrandEntity, "select * from beer_brand")
            .fetch_all(&pool)
            .await
            .expect("Unable to query beer brand table");

    println!("There are in db: {} beer_brands.", beer_brands.len());
    println!("found in db: {:?}", beer_brands);
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
- use Json in Rest
- Tests
- database, evtl verschiedene postgres, mongodb
- ...



rustup default nightly --> wird für rocket gebraucht?!?!??!!!!
rustup default stable

docker run --name some-postgres -e POSTGRES_PASSWORD=mysecretpassword -d postgres
docker run --name funny-endpoints-postgres -e POSTGRES_PASSWORD=c1Rt3X66rGi5flJypblB -d postgres -p 5555:5432


*/
