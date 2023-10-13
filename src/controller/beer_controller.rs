use crate::{pool::Db, service::beer_service::TimeForBeerResponse};
use entity::beer_brand;
use entity::beer_brand::Entity as BeerBrand;
use rocket::{
    form::Form,
    http::Status,
    response::{self, Responder},
    serde::json::Json,
    Request,
};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, EntityTrait, QueryOrder};
use sea_orm_rocket::Connection;

pub struct DatabaseError(sea_orm::DbErr);

impl<'r> Responder<'r, 'r> for DatabaseError {
    fn respond_to(self, _request: &Request) -> response::Result<'r> {
        Err(Status::InternalServerError)
    }
}

impl From<sea_orm::DbErr> for DatabaseError {
    fn from(error: sea_orm::DbErr) -> Self {
        DatabaseError(error)
    }
}

#[get("/beer-time")]
pub fn is_time_for_beer() -> Json<TimeForBeerResponse> {
    Json(crate::service::beer_service::time_for_beer())
}

#[get("/")] // TODO reorganize
pub fn root() -> String {
    "Moin!\n\navailable endpoints: \n/drinks  \n/beer-time".into()
}

#[post("/beer-brand", data = "<beer_brand_form>")]
pub async fn create_beer_brand(
    connection: Connection<'_, Db>,
    beer_brand_form: Form<beer_brand::Model>, // TODO Json? nachlesen, was üblicher/besser ist
) -> Result<Json<beer_brand::Model>, DatabaseError> {
    let db = connection.into_inner();
    let beer_brand = beer_brand_form.into_inner();

    let active_beer_brand: beer_brand::ActiveModel = beer_brand::ActiveModel {
        id: Set(beer_brand.id), // TODO: Autoincrement oder so
        name: Set(beer_brand.name),
        city: Set(beer_brand.city),
        // ..Default::default()
    };

    Ok(Json(active_beer_brand.insert(db).await?))
}

#[get("/beer-brands")]
pub async fn get_beer_brands(
    connection: Connection<'_, Db>,
) -> Result<Json<Vec<beer_brand::Model>>, DatabaseError> {
    let db = connection.into_inner();

    let beers = BeerBrand::find()
        .order_by_asc(beer_brand::Column::Id)
        .all(db)
        .await?;
    println!("Found {} beer brands.", beers.len());
    Ok(Json(beers))

    // Ok(Json(
    //     BeerBrand::find()
    //         .order_by_asc(beer_brand::Column::Id)
    //         .all(db)
    //         .await?,
    // ))
}

#[put("/beer-brand", data = "<beer_brand_form>")]
pub async fn update_beer_brand(
    connection: Connection<'_, Db>,
    beer_brand_form: Form<beer_brand::Model>,
) -> Result<Json<beer_brand::Model>, DatabaseError> {
    let db = connection.into_inner();
    let beer_brand = beer_brand_form.into_inner();

    let beer_brand_to_update_optional = BeerBrand::find_by_id(beer_brand.id).one(db).await?;
    let mut beer_brand_to_update: beer_brand::ActiveModel =
        beer_brand_to_update_optional.unwrap().into(); // TODO unwrap eretzen
    beer_brand_to_update.name = Set(beer_brand.name);
    beer_brand_to_update.city = Set(beer_brand.city);

    Ok(Json(beer_brand_to_update.update(db).await?))
}

#[delete("/beer-brand/<id>")]
pub async fn delete_beer_brand(
    connection: Connection<'_, Db>,
    id: i32,
) -> Result<String, DatabaseError> {
    let db = connection.into_inner();
    let result = BeerBrand::delete_by_id(id).exec(db).await?;

    Ok(format!("{} beer brand(s) deleted", result.rows_affected))
}
