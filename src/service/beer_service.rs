use chrono::naive::NaiveTime;
use chrono::offset::Local;
use serde::{Deserialize, Serialize};

pub fn time_for_beer() -> TimeForBeerResponse {
    let time_format = "%H:%M";

    let now = Local::now().naive_local().time();
    println!("Now it is {} o'clock", &now.format(time_format));

    let beer_starting_time = NaiveTime::from_hms_opt(16, 00, 00).unwrap();

    let message = receive_beer_message(now > beer_starting_time);

    TimeForBeerResponse {
        actual_time: now.format(time_format).to_string(),
        beer_starting_time: beer_starting_time.format(time_format).to_string(),
        message: message,
    }
}

fn receive_beer_message(time_for_beer: bool) -> String {
    if time_for_beer {
        "Oh yes, it is after 4 o'clock".to_string()
    } else {
        "Actually somewhere it is after 4 o'clock".to_string()
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TimeForBeerResponse {
    actual_time: String,
    beer_starting_time: String,
    pub message: String, // TODO nicht public
}
