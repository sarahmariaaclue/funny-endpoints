use crate::persistence::beer_brand_persistence::BeerBrandEntity;
use crate::FunnyEndpointsDB;

use rocket::serde::json::Json;
use rocket_db_pools::Connection;
use serde::{Deserialize, Serialize};
use sqlx::{self};

use rocket::http::Status;
use rocket::request::Request;
use rocket::response::{Responder, Response};
use std::i64;
use std::io::Cursor;
// TODO datenbanck kram auslagern in repository
// TODO Fehlerbehandlung

#[get("/beer-brands")]
pub async fn get_beer_brands(
    mut connection: Connection<FunnyEndpointsDB>,
) -> Option<Json<Vec<BeerBrandEntity>>> {
    let result = sqlx::query_as::<_, BeerBrandEntity>("SELECT * FROM beer_brand")
        .fetch_all(&mut *connection)
        .await
        .ok()?;
    Some(Json(result))
}

#[get("/beer-brand/<id>")]
pub async fn get_beer_brand(
    id: i64,
    mut connection: Connection<FunnyEndpointsDB>,
) -> Option<Json<BeerBrandEntity>> {
    let result = sqlx::query_as::<_, BeerBrandEntity>("SELECT * FROM beer_brand WHERE id = $1")
        .bind(id)
        .fetch_optional(&mut *connection)
        .await
        .ok()?
        .map(Json)?;
    Some(result)
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
    .bind(beer_brand_entity.get_id())
    .bind(beer_brand_entity.get_name())
    .bind(beer_brand_entity.get_city())
    .fetch_one(&mut *connection)
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
