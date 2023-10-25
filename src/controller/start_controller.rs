use crate::service::beer_service::TimeForBeerInformation;

use rocket::serde::json::Json;

#[get("/")]
pub fn root() -> String {
    "Moin!".into()
}

#[get("/beer-time")]
pub fn is_time_for_beer() -> Json<TimeForBeerInformation> {
    Json(crate::service::beer_service::time_for_beer())
}

#[cfg(test)]
mod tests {
    use rocket::serde::json::Json;

    use crate::{
        controller::start_controller::{is_time_for_beer, root},
        service::beer_service::TimeForBeerInformation,
    };

    #[test]
    fn test_root_in_controller_file() {
        let result = root();
        let should_be = "Moin!".to_string();
        assert_eq!(result, should_be);
    }

    #[test]
    fn test_is_it_time_for_beer() {
        // TODO recherchieren, Zeitmanipulation für Testing
        let result = is_time_for_beer();
        let empty_result = Json(TimeForBeerInformation::new_epty());
        assert_ne!(result, empty_result);
    }
}
