use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct OnlinePing {
    #[serde(rename = "alertTime")]
    alert_time: u16,
    interval: u16,
    message: &'static str,
    result: u8,
    #[serde(rename = "timeLeft")]
    time_left: i8,
}

impl OnlinePing {
    pub const fn default() -> Self {
        Self {
            alert_time: 600,
            interval: 3590,
            message: "OK",
            result: 0,
            time_left: -1,
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct OnlineLoginOut {
    error: &'static str,
    message: &'static str,
    #[serde(rename = "statusCode")]
    status_code: u16,
}

impl OnlineLoginOut {
    pub const fn default() -> Self {
        Self {
            error: "Not Found",
            message: "Not Found",
            status_code: 404,
        }
    }
}
