// #[cfg(test)]
// mod beer_controller_tests {
//     use super::*;
//     use rocket::http::{ContentType, Status};
//     use rocket::local::blocking::Client;
//     use rocket_db_pools::Database;

//     #[rocket::async_test]
//     async fn test_get_beer_brands() {
//         let client = Client::tracked(rocket()).unwrap();
//         let response = client.get("/beer-brands").dispatch().await;

//         assert_eq!(response.status(), Status::Ok);
//         assert_eq!(response.content_type(), Some(ContentType::JSON));

//         let body = response.into_string().await.unwrap();
//         let beer_brands: Vec<BeerBrandEntity> = serde_json::from_str(&body).unwrap();
//         assert!(beer_brands.len() > 0);
//     }

//     #[rocket::async_test]
//     async fn test_get_beer_brand() {
//         let client = Client::tracked(rocket()).unwrap();
//         let response = client.get("/beer-brand/1").dispatch().await;

//         assert_eq!(response.status(), Status::Ok);
//         assert_eq!(response.content_type(), Some(ContentType::JSON));

//         let body = response.into_string().await.unwrap();
//         let beer_brand: BeerBrandEntity = serde_json::from_str(&body).unwrap();
//         assert_eq!(beer_brand.get_id(), 1);
//     }

//     #[rocket::async_test]
//     async fn test_create_beer_brand() {
//         let client = Client::tracked(rocket()).unwrap();
//         let beer_brand = BeerBrandDto {
//             name: "Test Beer Brand".to_string(),
//             city: "Test City".to_string(),
//         };
//         let response = client
//             .post("/beer-brand")
//             .header(ContentType::JSON)
//             .body(serde_json::to_string(&beer_brand).unwrap())
//             .dispatch()
//             .await;

//         assert_eq!(response.status(), Status::Ok);
//         assert_eq!(response.content_type(), Some(ContentType::JSON));

//         let body = response.into_string().await.unwrap();
//         let beer_brand: BeerBrandEntity = serde_json::from_str(&body).unwrap();
//         assert_eq!(beer_brand.get_name(), "Test Beer Brand");
//         assert_eq!(beer_brand.get_city(), "Test City");
//     }

//     #[rocket::async_test]
//     async fn test_update_beer_brand() {
//         let client = Client::tracked(rocket()).unwrap();
//         let beer_brand = BeerBrandEntity::new(
//             1,
//             "Updated Beer Brand".to_string(),
//             "Updated City".to_string(),
//         );
//         let response = client
//             .put("/beer-brand")
//             .header(ContentType::JSON)
//             .body(serde_json::to_string(&beer_brand).unwrap())
//             .dispatch()
//             .await;

//         assert_eq!(response.status(), Status::Ok);
//         assert_eq!(response.content_type(), Some(ContentType::JSON));

//         let body = response.into_string().await.unwrap();
//         let beer_brand: BeerBrandEntity = serde_json::from_str(&body).unwrap();
//         assert_eq!(beer_brand.get_id(), 1);
//         assert_eq!(beer_brand.get_name(), "Updated Beer Brand");
//         assert_eq!(beer_brand.get_city(), "Updated City");
//     }

//     #[rocket::async_test]
//     async fn test_delete_beer_brand() {
//         let client = Client::tracked(rocket()).unwrap();
//         let response = client.delete("/beer-brand/1").dispatch().await;

//         assert_eq!(response.status(), Status::Ok);
//         assert_eq!(response.content_type(), Some(ContentType::JSON));

//         let body = response.into_string().await.unwrap();
//         let beer_brand: BeerBrandEntity = serde_json::from_str(&body).unwrap();
//         assert_eq!(beer_brand.get_id(), 1);
//     }

//     fn rocket() -> rocket::Rocket {
//         rocket::build().attach(Database::fairing()).mount(
//             "/",
//             routes![
//                 get_beer_brands,
//                 get_beer_brand,
//                 create_beer_brand,
//                 update_beer_brand,
//                 delete_beer_brand
//             ],
//         )
//     }
// }
