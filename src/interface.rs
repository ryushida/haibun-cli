use chrono::NaiveDate;
use dialoguer::Input;

use crate::datetime;

/// Ask user for input and return entered integer
pub fn user_input_int(displayed_text: &str) -> i32 {
    let value: i32 = Input::new().with_prompt(displayed_text).interact().unwrap();
    value
}

pub fn user_input_float(displayed_text: &str) -> f64 {
    let value: f64 = Input::new().with_prompt(displayed_text).interact().unwrap();
    value
}

pub fn user_input_text(displayed_text: &str) -> String {
    let value: String = Input::new().with_prompt(displayed_text).interact().unwrap();
    value
}

pub fn user_input_date(displayed_text: &str) -> NaiveDate {
    let value: String = Input::new().with_prompt(displayed_text).interact().unwrap();
    let date = datetime::parse_date(&value).unwrap();
    date
}
