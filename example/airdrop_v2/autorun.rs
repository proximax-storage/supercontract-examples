pub mod blockchain;
pub mod dir_iterator;
pub mod file;
pub mod filesystem;
pub mod internet;

#[no_mangle]
pub extern "C" fn run() -> i32 {
    return 0;
}