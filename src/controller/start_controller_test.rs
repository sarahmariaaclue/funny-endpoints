// #[cfg(test)]
// mod tests {
//     use super::*;
//     use rocket::http::{ContentType, Status};
//     use rocket::local::blocking::Client;

//     #[test]
//     fn test_root() {
//         let client = Client::tracked(rocket()).expect("valid rocket instance");
//         let response = client.get("/").dispatch();
//         assert_eq!(response.status(), Status::Ok);
//         assert_eq!(response.into_string().unwrap(), "Moin!");
//     }

//     #[test]
//     fn test_beer_time() {
//         // TODO different times, assert answer
//         let client = Client::tracked(rocket()).expect("valid rocket instance");
//         let response = client.get("/beer-time").dispatch();
//         assert_eq!(response.status(), Status::Ok);
//         assert_eq!(response.content_type(), Some(ContentType::JSON));
//         let body = response.into_string().unwrap();
//         let time_for_beer: TimeForBeerInformation = serde_json::from_str(&body).unwrap();
//         assert!(time_for_beer.is_time_for_beer);
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use rocket::local::blocking::Client;

//     #[test]
//     fn test_beer_time_endpoint() {
//         let rocket = rocket::ignite().mount("/", routes![beer_time]);
//         let client = Client::tracked(rocket).expect("valid rocket instance");

//         let response = client.get("/beer-time").dispatch();

//         assert_eq!(response.status(), Status::Ok);
//         assert_eq!(response.into_string().unwrap(), "It's beer time!");

//         // Additional assertions or checks can be added as needed
//     }
// }
