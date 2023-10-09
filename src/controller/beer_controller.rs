use crate::service::beer_service::TimeForBeerResponse;
use rocket::serde::json::Json;

#[get("/beer-time")]
pub fn is_time_for_beer() -> Json<TimeForBeerResponse> {
    Json(crate::service::beer_service::time_for_beer())
}

#[get("/")]
pub fn root() -> String {
    "Moin!".into()
}
