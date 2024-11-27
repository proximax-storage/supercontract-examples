pub mod blockchain;
pub mod dir_iterator;
pub mod file;
pub mod filesystem;
pub mod internet;

#[no_mangle]
pub extern "C" fn run() -> i32 {
    if get_block_height > 1000 {  // example crowdfunding expired height is 1000 block height
        return 0;
    } else {
        panic!();
    }
}
