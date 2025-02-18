use std::io::{BufReader, Read};
use blockchain::print_log;
use serde::{Deserialize, Serialize};

pub mod blockchain;
pub mod dir_iterator;
pub mod file;
pub mod filesystem;
pub mod internet;

// Json deserialize
#[derive(Debug, Serialize, Deserialize)]
struct Mosaic {
    id: Vec<u32>,
    amount: Vec<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Account {
    address: String,
    mosaics: Vec<Mosaic>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Root {
    account: Account,
}

#[no_mangle]
pub unsafe extern "C" fn run() -> u32 {

    // Sirius chain Rest Endpoint
    let baseurl = "http://109.123.232.101:3000/account/";

    // Account to fetch
    let address = "SD2L2LRSBZUMYV2T34C4UXOIAAWX4TWQSQGBPMQO";
    {
        // Fetch the url "http://109.123.232.101:3000/account/SD2L2LRSBZUMYV2T34C4UXOIAAWX4TWQSQGBPMQO"
        let internet = internet::Internet::new(&format!("{}{}", baseurl, address), true).unwrap();
        let mut reader = BufReader::with_capacity(1024, internet);
        let mut internet_buffer = Vec::new();
        reader.read_to_end(&mut internet_buffer).unwrap();
        let result: Root = serde_json::from_slice(&internet_buffer).unwrap();

        print_log(&result.account.address);
    }

    return 0;
}