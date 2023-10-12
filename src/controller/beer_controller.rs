use std::{
    fs::OpenOptions,
    io::{BufRead, BufReader, Write},
};

use crate::service::beer_service::TimeForBeerResponse;
use rocket::serde::json::Json;
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

    // file, delete code later:
    let mut brands = OpenOptions::new()
        .read(true)
        .append(true)
        .create(true)
        .open(FILE_PATH)
        .expect("unable to access brands.txt");
    let brand_string = format!("{} from {}\n", new_beer_brand.name, new_beer_brand.city);
    let brand_bytes = brand_string.as_bytes();
    brands
        .write(brand_bytes)
        .expect("unable to write to barnds.txt");
    "Brand added succesfully"
}

#[get("/beer-brands")]
pub fn get_beer_brands() -> Json<Vec<String>> {
    let file = OpenOptions::new()
        .read(true)
        .append(true)
        .create(true)
        .open(FILE_PATH)
        .expect("unable to access barnds.txt");
    let reader = BufReader::new(file);
    Json(
        reader
            .lines()
            .map(|line| line.expect("could not read line"))
            .collect(),
    )

    // let karlsberg = BeerBrand {
    //     name: "Karlsberg".into(),
    //     city: "Homburg".into(),
    // };
    // Json(vec![karlsberg])
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct BeerBrand {
    id: i64,
    name: String,
    city: String,
}
