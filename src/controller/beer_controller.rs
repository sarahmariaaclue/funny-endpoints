use std::{
    fs::OpenOptions,
    io::{BufRead, BufReader, Write},
};

use crate::{service::beer_service::TimeForBeerResponse, BeerBrandEntity, FunnyEndpointsDatabase};
use rocket::serde::json::Json;
use rocket_db_pools::sqlx::Row;
use rocket_db_pools::Connection;
use serde::{Deserialize, Serialize};

const FILE_PATH: &str = "brands.txt";

#[get("/beer-time")]
pub fn is_time_for_beer() -> Json<TimeForBeerResponse> {
    Json(crate::service::beer_service::time_for_beer())
}

#[get("/")] // TODO reorganize
pub fn root() -> String {
    "Moin!\n\navailable endpoints: \n/drinks  \n/beer-time".into()
}

#[post("/beer-brand", data = "<new_beer_brand>")]
pub fn create_beer_brand(new_beer_brand: Json<BeerBrand>) -> &'static str {
    println!(
        "save beer {} from {}",
        &new_beer_brand.name, &new_beer_brand.city
    );

    "saved!"
}

#[get("/beer-brands")]
pub async fn get_beer_brands(db: Connection<FunnyEndpointsDatabase>) -> Json<Vec<BeerBrand>> {
    let res = sqlx::query("SELECT content FROM beer_brand").fetch_all(&mut **db);
    let karlsberg = BeerBrand {
        id: 56,
        name: "Karlsberg".into(),
        city: "Homburg".into(),
    };
    Json(vec![karlsberg])
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct BeerBrand {
    id: i64,
    name: String,
    city: String,
}
