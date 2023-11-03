use crate::persistence::beer_brand_persistence::repository::{
    delete_beer_brand_by_id, find_beer_brand_by_id, find_beer_brands, save_beer_brand,
    save_updated_beer_brand, DatabaseError,
};
use crate::persistence::beer_brand_persistence::BeerBrandEntity;
use crate::FunnyEndpointsDB;

use rocket::serde::json::Json;
use rocket_db_pools::Connection;
use serde::Deserialize;

use rocket::http::Status;
use rocket::request::Request;
use rocket::response::{Responder, Response};
use std::io::Cursor;

#[get("/beer-brands")]
pub async fn get_beer_brands(
    connection: Connection<FunnyEndpointsDB>,
) -> Result<Json<Vec<BeerBrandEntity>>, DatabaseError> {
    let result = find_beer_brands(connection).await;
    result.map(Json)
}

#[get("/beer-brand/<id>")]
pub async fn get_beer_brand(
    id: i64,
    connection: Connection<FunnyEndpointsDB>,
) -> Result<Option<Json<BeerBrandEntity>>, DatabaseError> {
    let result = find_beer_brand_by_id(connection, id).await;
    result.map(|option| option.map(Json))
}

#[post("/beer-brand", data = "<beer_brand>")]
pub async fn create_beer_brand(
    connection: Connection<FunnyEndpointsDB>,
    beer_brand: Json<BeerBrandDto>,
) -> Result<Json<BeerBrandEntity>, DatabaseError> {
    let result = save_beer_brand(connection, &beer_brand.name, &beer_brand.city).await;
    result.map(Json)
}

#[put("/beer-brand", data = "<beer_brand_entity>")]
pub async fn update_beer_brand(
    connection: Connection<FunnyEndpointsDB>,
    beer_brand_entity: Json<BeerBrandEntity>,
) -> Result<Option<Json<BeerBrandEntity>>, DatabaseError> {
    let result = save_updated_beer_brand(connection, beer_brand_entity.into_inner()).await;
    result.map(|option| option.map(Json))
}

#[delete("/beer-brand/<id>")]
pub async fn delete_beer_brand(
    connection: Connection<FunnyEndpointsDB>,
    id: i64,
) -> Result<Option<Json<BeerBrandEntity>>, DatabaseError> {
    let result: Result<Option<BeerBrandEntity>, DatabaseError> =
        delete_beer_brand_by_id(connection, id).await;
    result.map(|option| option.map(Json))
}

#[derive(Deserialize, Clone, Debug)]
pub struct BeerBrandDto {
    name: String,
    city: String,
}

impl<'r> Responder<'r, 'static> for DatabaseError {
    fn respond_to(self, _: &'r Request<'_>) -> Result<Response<'static>, Status> {
        let message = self.get_message();
        let response = Response::build()
            .status(Status::InternalServerError)
            .sized_body(message.len(), Cursor::new(message))
            .finalize();
        Ok(response)
    }
}

// Construct a Rocket instance that represents the application.

// 1
// let rocket = rocket::build();
// Construct a Client using the Rocket instance.

// 1
// let client = Client::tracked(rocket).unwrap();
// Construct requests using the Client instance.

// 1
// let req = client.get("/");
// Dispatch the request to retrieve the response.

// 1
// let response = req.dispatch();

#[cfg(test)]
mod tests {

    use super::*;
    use crate::rocket;
    use rocket::local::asynchronous::Client;

    #[rocket::async_test]
    async fn test_get_beer_brands() {
        //    let should_be = "Moin!";
        let rocket = rocket().await;
        let client = Client::tracked(rocket).await.unwrap();
        let request = client.get("/beer-brands");
        let response = request.dispatch().await;

        // let rocket = rocket::build();
        // let client = Client::tracked(rocket).unwrap();
        // let request = client.get("/beer-brands");
        // let response = request.dispatch();

        // mock the database

        // #[derive(Database)]
        // #[database("funny_endpoints")]
        // pub struct FunnyEndpointsDB(sqlx::Pool<Postgres>);

        assert_eq!(response.status(), Status::Ok);
    }
}
