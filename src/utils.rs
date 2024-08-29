use std::{str::Split, time::SystemTime};

pub fn get_sys_time_in_secs() -> i64 {
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => n.as_secs().try_into().unwrap_or(0_i64),
        Err(_) => 0_i64,
    }
}

pub const PARAGRAPH_DELIMITER: &str = ";";

pub fn paragraphs_to_vec(paragraphs: &str) -> Split<&str> {
    paragraphs.split(PARAGRAPH_DELIMITER)
}

pub fn append_paragraph(paragraphs: &str, new_para_id: i64) -> String {
    format!("{paragraphs}{new_para_id};")
}
