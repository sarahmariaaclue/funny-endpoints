use chrono::naive::NaiveTime;
use chrono::offset::Local;
use serde::{Deserialize, Serialize};

pub fn time_for_beer() -> TimeForBeerInformation {
    let time_format = "%H:%M";
    let now = Local::now().naive_local().time();
    let beer_starting_time = NaiveTime::from_hms_opt(16, 00, 00).unwrap();
    let message = receive_beer_message(now > beer_starting_time);

    TimeForBeerInformation {
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

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct TimeForBeerInformation {
    actual_time: String,
    beer_starting_time: String,
    message: String,
}

// TODO remove, just for test
impl TimeForBeerInformation {
    pub fn new_epty() -> TimeForBeerInformation {
        TimeForBeerInformation {
            actual_time: "".to_string(),
            beer_starting_time: "".to_string(),
            message: "".to_string(),
        }
    }
}
