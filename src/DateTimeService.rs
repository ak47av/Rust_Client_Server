use chrono::{Local};


pub fn getDateAndTime() -> String {
    let dt = Local::now();
    format!("{}", dt)
}