use chrono::format::ParseError;
use chrono::NaiveDate;

pub fn parse_date(date_str: &str) -> Result<NaiveDate, ParseError> {
    let date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d")?;

    Ok(date)
}
