pub mod blockchain;
pub mod dir_iterator;
pub mod file;
pub mod filesystem;
pub mod internet;

use blockchain::print_log;

use crate::file::{FileReader, FileWriter};
use std::io::{BufReader, BufWriter, Read, Write};

#[no_mangle]
pub unsafe extern "C" fn run() -> u32 {
    let test_case = "Hi wasmer.\nGood to see you.\n".as_bytes();

    // Write file
    {
        let file = FileWriter::new("test.txt").unwrap();
        let mut writer = BufWriter::new(file);
        writer.write(test_case).unwrap();
        writer.flush().unwrap();
    }

    // Read file
    let mut buffer = Vec::new();
    {
        let file = FileReader::new("test.txt").unwrap();
        let mut reader = BufReader::new(file);
        reader.read_to_end(&mut buffer).unwrap();
    }

    // Check if write and read are the same
    if test_case != buffer {
        return 0;
    }else {
        print_log("buffer not same");
        return 2;
    }
}
