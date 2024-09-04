use chrono::Utc;

pub fn time(faketime: i64) -> u64 {
    if faketime < 0 {
        Utc::now().timestamp() as u64
    } else {
        faketime as u64
    }
}
