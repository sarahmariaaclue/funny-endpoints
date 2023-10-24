use crate::{service::beer_service::TimeForBeerResponse, FunnyEndpointsDB};

use rocket::serde::json::Json;
use rocket_db_pools::Connection;
use serde::{Deserialize, Serialize};
use sqlx::Error;
use sqlx::{self};

use rocket::http::Status;
use rocket::request::Request;
use rocket::response::{Responder, Response};
use std::i64;
use std::io::Cursor;

#[get("/")]
pub fn root() -> String {
    "Moin!\n\navailable endpoints: \n/drinks  \n/beer-time".into()
}

#[get("/beer-time")]
pub fn is_time_for_beer() -> Json<TimeForBeerResponse> {
    Json(crate::service::beer_service::time_for_beer())
}

#[get("/beer-brands")]
pub async fn get_beer_brands(
    mut connection: Connection<FunnyEndpointsDB>, // TODO Pool
) -> Option<Json<Vec<BeerBrandEntity>>> {
    let result: Result<Vec<BeerBrandEntity>, Error> =
        sqlx::query_as::<_, BeerBrandEntity>("SELECT * FROM beer_brand")
            .fetch_all(&mut *connection)
            .await
            .and_then(|r| Ok(r));
    let result2 = result.expect("msg");

    Some(Json(result2))
}

#[get("/beer-brand/<id>")]
pub async fn get_beer_brand(
    id: i64,
    mut connection: Connection<FunnyEndpointsDB>,
) -> Option<Json<BeerBrandEntity>> {
    sqlx::query_as::<_, BeerBrandEntity>("SELECT * FROM beer_brand WHERE id = $1")
        .bind(id)
        .fetch_optional(&mut *connection)
        .await
        .ok()
        .flatten()
        .map(Json)
}

#[post("/beer-brand", data = "<beer_brand>")]
pub async fn create_beer_brand(
    mut connection: Connection<FunnyEndpointsDB>,
    beer_brand: Json<BeerBrandDto>,
) -> Result<Json<BeerBrandEntity>, DatabaseError> {
    let result = sqlx::query_as::<_, BeerBrandEntity>(
        "INSERT INTO beer_brand (name, city) VALUES ($1, $2) RETURNING *",
    )
    .bind(&beer_brand.name)
    .bind(&beer_brand.city)
    .fetch_one(&mut *connection)
    .await
    .map_err(DatabaseError)?;

    Ok(Json(result))
}

#[put("/beer-brand", data = "<beer_brand_entity>")]
pub async fn update_beer_brand(
    mut connection: Connection<FunnyEndpointsDB>,
    beer_brand_entity: Json<BeerBrandEntity>,
) -> Result<Json<BeerBrandEntity>, DatabaseError> {
    let result = sqlx::query_as::<_, BeerBrandEntity>(
        "UPDATE beer_brand SET name = $2, city = $3 WHERE id= $1 RETURNING *",
    )
    .bind(&beer_brand_entity.id)
    .bind(&beer_brand_entity.name)
    .bind(&beer_brand_entity.city)
    .fetch_one(&mut *connection) //TODO: Frage, watum hier *
    .await
    .map_err(DatabaseError)?;

    Ok(Json(result))
}

#[delete("/beer-brand/<id>")]
pub async fn delete_beer_brand(
    mut connection: Connection<FunnyEndpointsDB>,
    id: i64,
) -> Result<Json<BeerBrandEntity>, DatabaseError> {
    let result =
        sqlx::query_as::<_, BeerBrandEntity>("DELETE FROM beer_brand WHERE id= $1 RETURNING *")
            .bind(id)
            .fetch_one(&mut *connection)
            .await
            .map_err(DatabaseError)?;

    Ok(Json(result))
}

#[derive(Serialize, Deserialize, Clone, Debug, sqlx::FromRow)]
// TODO beim Hochfahren der Andendung create table if not exists
pub struct BeerBrandEntity {
    id: i64,
    name: String,
    city: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, sqlx::FromRow)]
pub struct BeerBrandDto {
    name: String,
    city: String,
}

#[derive(Debug)]
pub struct DatabaseError(sqlx::Error);

impl<'r> Responder<'r, 'static> for DatabaseError {
    fn respond_to(self, _: &'r Request<'_>) -> Result<Response<'static>, Status> {
        let message = format!("Database error: {}", self.0);
        let response = Response::build()
            .status(Status::InternalServerError)
            .sized_body(message.len(), Cursor::new(message))
            .finalize();
        Ok(response)
    }
}
