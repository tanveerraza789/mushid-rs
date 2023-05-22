//! # Mush ID
//! A Unique ID generator based on time, network state and OS randomness of distributing PIDs.
//!
//! # Usage
//! ```rust
//! use mushid::mushid;
//! let id = mushid();
//! println!("{id}"); // a6c66fc1cbf9151f
//! ```
//!
use std::time::{SystemTime, UNIX_EPOCH};

fn calculate_crc32_hex(data: &[u8]) -> String {
    format!("{:08x}", crc32c::crc32c(data))
}

fn get_sys_time() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos()
}

fn get_mac_address() -> String {
    if let Ok(Some(mac)) = mac_address::get_mac_address() {
        mac.to_string()
    } else {
        String::from("00:00:00:00:00:00")
    }
}

fn get_pid() -> u32 {
    std::process::id()
}

fn mushid() -> String {
    let time = calculate_crc32_hex(&get_sys_time().to_ne_bytes());
    let buff = get_pid().to_string() + "-" + get_mac_address().as_str();
    let diffrentiator = calculate_crc32_hex(buff.as_bytes());
    time + diffrentiator.as_str()
}

#[cfg(test)]
mod tests {
    use crate::mushid;

    #[test]
    fn it_works() {
        let id = mushid();
        println!("{}", id);
    }
}
