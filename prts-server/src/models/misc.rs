use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct EventResponse<'a> {
    code: u16,
    msg: &'a str,
    next: u16,
}

pub const EVENT: EventResponse = EventResponse { code: 200, msg: "OK", next: 180 };

#[derive(Deserialize, Serialize)]
pub struct MiscResponse<'a> {
    code: u16,
    msg: &'a str,
}

pub const BATCH_EVENT: MiscResponse = MiscResponse { code: 200, msg: "OK" };
