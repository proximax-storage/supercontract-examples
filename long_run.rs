pub mod blockchain;
pub mod dir_iterator;
pub mod file;
pub mod filesystem;
pub mod internet;

use std::io::BufReader;
use std::io::Read;

#[no_mangle]
pub unsafe extern "C" fn run() -> u32 {
    let internet = internet::Internet::new("example.com", true).unwrap();
    let mut reader = BufReader::with_capacity(16 * 1024, internet);
    // let identifier = 123456789i64;
    let mut buf = Vec::new();
    // buf.resize(20 * 1048576, 0);
    let res = reader.read_to_end(&mut buf).unwrap();
    let mut i = 0;
    let length = buf.len();
    while i < (res * 10000) {
        buf[i % length] = 255;
        i += 1;
    }
    return 1;
}
