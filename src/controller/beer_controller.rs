use std::i64;

use crate::{service::beer_service::TimeForBeerResponse, FunnyEndpointsDB};

use rocket::serde::json::Json;
use rocket_db_pools::Connection;
use serde::{Deserialize, Serialize};
use sqlx::Error;
use sqlx::{self, postgres::PgRow};

use rocket_db_pools::sqlx::Row;

#[get("/")]
pub fn root() -> String {
    "Moin!\n\navailable endpoints: \n/drinks  \n/beer-time".into()
}

#[get("/beer-time")]
pub fn is_time_for_beer() -> Json<TimeForBeerResponse> {
    Json(crate::service::beer_service::time_for_beer())
}

#[get("/beer-brands")]
pub async fn get_beer_brands(mut connection: Connection<FunnyEndpointsDB>) -> Option<String> {
    // Json<Vec<BeerBrandEntity>>
    // TODO
    //Json([].to_vec())
    let result: Result<Vec<PgRow>, Error> = sqlx::query("SELECT * FROM beer_brand")
        .fetch_all(&mut *connection)
        .await
        .and_then(|r| Ok(r));
    let result2 = result.expect("msg");

    let result3 = result2.len();

    Some(result3.to_string())

    //result.ok().map(|s| String(s))
}

// querys the database for the beer brand with id=id
// returns the beer brand as an optional
#[get("/beer-brand/<id>")]
pub async fn get_beer_brand(
    id: i64,
    mut connection: Connection<FunnyEndpointsDB>,
) -> Option<String> {
    // Option<Json<BeerBrandEntity>>
    sqlx::query("SELECT * FROM beer_brand WHERE id = ?")
        .bind(id)
        .fetch_one(&mut *connection)
        .await
        .and_then(|r| Ok(r.try_get(0)?))
        .ok()
    // TODO: sqlx neuste version!!

    // Some(Json(BeerBrandEntity {
    //     id: id,
    //     name: "bestBeer".into(),
    //     city: "Wemmezuela".into(),
    // }))
}

// #[get("/beer-brand/<id>")]
// pub async fn get_beer_brand(
//     mut connection: Connection<FunnyEndpointsDB>,
//     id: i64,
// ) -> Option<String> {
//     print!("id is {}", id);
//     // sqlx::query("SELECT * FROM beer_brand WHERE id = ?")
//     // .bind(id)
//     // query the beer brand with id from the database
//     sqlx::query_as!(
//         BeerBrandEntity,
//         "SELECT * FROM beer_brand WHERE id = $1", // TODO : Check postgres database url
//         id
//     )
//     .fetch_optional(&mut *connection)
//     .await
//     .and_then(|r| Ok(r.try_get(0)?))
//     .ok()
// }

#[derive(Serialize, Deserialize, Clone, Debug, sqlx::FromRow)]
pub struct BeerBrandEntity {
    id: i64,
    name: String,
    city: String,
}

// #[get("/beer-brand/<id>")]
// pub async fn get_beer_brand(pool: &State<Pool<Postgres>>, id: u64) -> Result<String, Status> {
//     // &State<Pool<Postgres>>
//     let beer_brand_result = BeerBrandEntity::find_by_id(id, &pool).await;

//     match beer_brand_result {
//         Ok(beer_brand) => Ok(beer_brand.name), // TODO return entity
//         _ => Err(Status::NotFound),
//     }
// }

// impl BeerBrandEntity {
//     pub async fn find_by_id(id: u64, pool: &Pool<Postgres>) -> Result<BeerBrandEntity> {
//         let user = sqlx::query_as!(
//             BeerBrandEntity,
//             "SELECT * FROM beer_brand WHERE id = $1",
//             id
//         )
//         .fetch_one(&*pool)
//         .await?;

//         Ok(user)
//     }
// }

// #[post("/beer-brand", data = "<beer_brand_form>")]
// pub async fn create_beer_brand(
//     connection: Connection<'_, Db>,
//     beer_brand_form: Form<beer_brand::Model>, // TODO Json? nachlesen, was üblicher/besser ist
// ) -> Result<Json<beer_brand::Model>, DatabaseError> {
//     let db = connection.into_inner();
//     let beer_brand = beer_brand_form.into_inner();

//     let active_beer_brand: beer_brand::ActiveModel = beer_brand::ActiveModel {
//         id: Set(beer_brand.id), // TODO: Autoincrement oder so
//         name: Set(beer_brand.name),
//         city: Set(beer_brand.city),
//         // ..Default::default()
//     };

//     Ok(Json(active_beer_brand.insert(db).await?))
// }

// #[put("/beer-brand", data = "<beer_brand_form>")]
// pub async fn update_beer_brand(
//     connection: Connection<'_, Db>,
//     beer_brand_form: Form<beer_brand::Model>,
// ) -> Result<Json<beer_brand::Model>, DatabaseError> {
//     let db = connection.into_inner();
//     let beer_brand = beer_brand_form.into_inner();

//     let beer_brand_to_update_optional = BeerBrand::find_by_id(beer_brand.id).one(db).await?;
//     let mut beer_brand_to_update: beer_brand::ActiveModel =
//         beer_brand_to_update_optional.unwrap().into(); // TODO unwrap eretzen
//     beer_brand_to_update.name = Set(beer_brand.name);
//     beer_brand_to_update.city = Set(beer_brand.city);

//     Ok(Json(beer_brand_to_update.update(db).await?))
// }

// #[delete("/beer-brand/<id>")]
// pub async fn delete_beer_brand(
//     connection: Connection<'_, Db>,
//     id: i32,
// ) -> Result<String, DatabaseError> {
//     let db = connection.into_inner();
//     let result = BeerBrand::delete_by_id(id).exec(db).await?;

//     Ok(format!("{} beer brand(s) deleted", result.rows_affected))
// }
