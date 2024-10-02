pub mod blockchain;
pub mod dir_iterator;
pub mod file;
pub mod filesystem;
pub mod internet;
pub mod btc_price;

use btc_price::read_btc_price;

#[no_mangle]
pub extern "C" fn run() -> i32 {
    // approach 1:
    if blockchain::get_block_height() % 60 == 0 {
        // call read btc price function
        return read_btc_price();
    }
    return 0;
}