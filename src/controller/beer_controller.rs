#[get("/beer-time")]
pub fn is_time_for_beer() -> String {
    crate::service::beer_service::time_for_beer()
}
