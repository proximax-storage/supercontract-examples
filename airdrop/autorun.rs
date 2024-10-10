pub mod blockchain;
pub mod dir_iterator;
pub mod file;
pub mod filesystem;
pub mod internet;

#[no_mangle]
pub extern "C" fn run() -> i32 {
    if blockchain::get_block_height() == 20000 {
        return 0;
    }
    return 2;
}