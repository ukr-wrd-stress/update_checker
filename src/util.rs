use std::time::{Duration, SystemTime, UNIX_EPOCH};
use chrono::{DateTime, Utc};
use chrono_tz::America::New_York;

#[derive(PartialEq)]
pub enum AccessType { WRITE, READ }

///Returns either the String contents of the url, or an error
pub async fn html(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let response = reqwest::get(url).await?;
    Ok(response.text().await?)
}

/// Format the system time
pub fn time() -> String {
    let datetime: DateTime<Utc> = (UNIX_EPOCH + Duration::from_nanos(sys_time() as u64)).into();
    datetime.with_timezone(&New_York).format("%m_%d_%y %H:%M:%S").to_string()
}

///Returns the system time
fn sys_time() -> u128 {
    return match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(duration) => duration.as_nanos(),
        Err(_) => 0
    };
}