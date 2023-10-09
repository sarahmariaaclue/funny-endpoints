use chrono::naive::NaiveTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug)]
pub struct DrinkEntity {
    id: Uuid, // TODO ?? macht man das so?
    name: String,
    description: String,
    alcohol_content_in_percent: u16,
    starting_day_time: NaiveTime,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct DrinkDto {
    name: String,
    description: String,
    alcohol_content_in_percent: u16,
    starting_day_time_hour: u8,
    starting_day_time_minutes: u8,
}

impl DrinkDto {
    pub fn is_alcoholic(&self) -> bool {
        &self.alcohol_content_in_percent > &0
    }
    pub fn beer() -> DrinkDto {
        DrinkDto {
            name: "beer".into(),
            description: "barley juice".into(),
            alcohol_content_in_percent: 5,
            starting_day_time_hour: 16,
            starting_day_time_minutes: 0,
        }
    }

    // fn new(
    //     name: String,
    //     description: String,
    //     alcohol_content_in_percent: u16,
    //     starting_day_hour: u8,
    // ) -> DrinkEntity {
    //     DrinkEntity { name }
    // }
}
