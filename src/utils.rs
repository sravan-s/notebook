use std::time::SystemTime;

pub fn get_sys_time_in_secs() -> i64 {
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => n.as_secs().try_into().unwrap_or(0_i64),
        Err(_) => 0_i64,
    }
}
