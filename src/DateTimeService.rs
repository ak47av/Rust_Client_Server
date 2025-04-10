use chrono::{DateTime, Local};

// Gets the local date and time and provides a string output
pub fn getDateAndTime() -> String {
    let dt = Local::now();
    format!("{}", dt)
}