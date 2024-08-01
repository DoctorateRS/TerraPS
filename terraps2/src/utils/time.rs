use chrono::Utc;

use crate::USER_CONFIG;

pub fn time() -> u64 {
    let faketime = USER_CONFIG.fake_time;
    if faketime < 0 {
        Utc::now().timestamp() as u64
    } else {
        faketime as u64
    }
}
