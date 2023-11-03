use crate::service::beer_service::TimeForBeerInformation;
use rocket::serde::json::Json;

#[get("/")]
pub fn root() -> &'static str {
    "Moin!"
}

#[get("/beer-time")]
pub fn is_time_for_beer() -> Json<TimeForBeerInformation> {
    Json(crate::service::beer_service::time_for_beer())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rocket::{local::asynchronous::Client, serde::json::Json};

    use crate::{
        controller::start_controller::is_time_for_beer,
        service::beer_service::TimeForBeerInformation,
    };

    use crate::rocket;

    #[test]
    fn test_root_in_controller_file() {
        let result = root();
        let should_be = "Moin!".to_string();
        assert_eq!(result, should_be);
    }

    #[rocket::async_test]
    async fn test_root_endpoint_in_controller_file() {
        let should_be = "Moin!";
        let rocket = rocket().await;
        let client = Client::tracked(rocket).await.unwrap();
        let request = client.get("/");
        let response = request.dispatch().await;
        assert_eq!(response.status(), rocket::http::Status::Ok);
        assert_eq!(response.into_string().await.unwrap(), should_be);
    }

    #[test]
    fn test_is_it_time_for_beer() {
        // TODO recherchieren, Zeitmanipulation für Testing

        // for the test, manipulates the system time to 15:00

        let result = is_time_for_beer();
        let empty_result = Json(TimeForBeerInformation::new_epty());
        assert_ne!(result, empty_result);
    }
}
