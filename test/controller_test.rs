#[cfg(test)]
mod tests {
    use super::*;
    use rocket::local::blocking::Client;

    #[test]
    fn test_beer_time_endpoint() {
        let rocket = rocket::ignite().mount("/", routes![beer_time]);
        let client = Client::tracked(rocket).expect("valid rocket instance");

        let response = client.get("/beer-time").dispatch();

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string().unwrap(), "It's beer time!");

        // Additional assertions or checks can be added as needed
    }
}
