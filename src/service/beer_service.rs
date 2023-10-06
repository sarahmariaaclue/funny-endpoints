use chrono::naive::NaiveTime;
use chrono::offset::Local;

pub fn time_for_beer() -> String {
    let time_format = "%H:%M";

    let now = Local::now().naive_local().time();
    println!("Now it is {} o'clock", &now.format(time_format));

    let happy_hour = NaiveTime::from_hms_opt(16, 00, 00).unwrap();

    if now > happy_hour {
        "Oh yes, it is after 4 o'clock".to_string()
    } else {
        "Actually it is somewhere after 4 o'clock".to_string()
    }
}
