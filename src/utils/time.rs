use std::time::{SystemTime, UNIX_EPOCH};

pub fn get_timestamp() -> u64 {
    let duration =  SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Error occurred");
    duration.as_secs() * 1000 + duration.subsec_nanos() as u64 / 1_000_000
}