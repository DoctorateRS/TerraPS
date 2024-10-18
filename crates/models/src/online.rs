use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OnlinePing {
    alert_time: u16,
    interval: u16,
    message: &'static str,
    result: u8,
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
#[serde(rename_all = "camelCase")]
pub struct OnlineLoginOut {
    error: &'static str,
    message: &'static str,
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
