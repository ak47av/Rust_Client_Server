use chrono::{DateTime, Local};


pub fn getDateAndTime() -> String {
    let dt = Local::now();
    format!("{}", dt)
}