// use crate::persistence::drink_entity::DrinkDto;
// use rocket::serde::json::Json;

#[get("/")]
pub fn get_all_drinks() -> String {
    "beer and friends, comming soon.".into()
}
// pub fn get_all_drinks() -> Json<[DrinkDto]> {
//     let drinks = [DrinkDto::beer()];
//     Json(drinks)
// }
