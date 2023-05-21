use crc::crc32::{Digest, Hasher32};
use std::hash::Hasher;
use std::time::{SystemTime, UNIX_EPOCH};

fn calculate_crc32_hex(data: &[u8]) -> String {
    let mut digest = Digest::new(crc32::IEEE);
    digest.write(data);
    format!("{:08x}", digest.sum32())
}

fn get_sys_time() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos()
}

fn get_mac_address() -> String {
    if let Ok(Some(mac)) = mac_address::get_mac_address() {
        return mac.to_string();
    } else {
        return String::from("00:00:00:00:00:00");
    }
}

fn get_pid() -> u32 {
    std::process::id()
}

fn mushid() -> String {
    let time = calculate_crc32_hex(&get_sys_time().to_ne_bytes());
    let buff = get_pid().to_string() + "-" + get_mac_address().as_str();
    let diffrentiator = calculate_crc32_hex(&buff);
    time.to_string() + diffrentiator.to_str()
}

#[cfg(test)]
mod tests {
    use crate::{get_mac_address, mushid};

    #[test]
    fn it_works() {
        let id = mushid();
        println!("{:?}", id);
    }
}
