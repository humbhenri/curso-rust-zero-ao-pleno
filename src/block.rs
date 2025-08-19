use std::time::{SystemTime, UNIX_EPOCH};

pub struct Block {
    index: u64,
    timestamp: u64,
    data: String,
    hash: String,
    prev_hash: String,
}

fn get_system_time() -> u64 {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    since_the_epoch.as_secs()
}

impl Block {
    pub fn new() -> Self {
        Block {
            index: 0,
            timestamp: get_system_time(),
            data: String::from(""),
            hash: String::from(""),
            prev_hash: String::from(""),
        }
    }

    pub fn data_size(&self) -> usize {
        self.data.len()
    }

    pub fn creation_time(&self) -> u64 {
        self.timestamp
    }
}
