pub mod blockchain;
pub mod dir_iterator;
pub mod file;
pub mod filesystem;
pub mod internet;

use crate::file::{FileReader, FileWriter};
use crate::filesystem::*;
use std::io::{BufReader, BufWriter, Read, Write};

#[no_mangle]
pub unsafe extern "C" fn run() -> u32 {
    let test_case = "Hi wasmer.\nGood to see you.\n".as_bytes();

    {
        let file = FileWriter::new("test.txt").unwrap();
        let mut writer = BufWriter::new(file);
        writer.write(test_case).unwrap();
        // flush to ensure the buffer wrapped by the BufWriter is all written in the file
        writer.flush().unwrap(); // https://stackoverflow.com/questions/69819990/whats-the-difference-between-flush-and-sync-all#:~:text=So%20what%20is,%27s%20documentation).
    }

    let mut big_buffer = Vec::new();
    {
        let file = FileReader::new("test.txt").unwrap();
        let mut reader = BufReader::new(file);
        reader.read_to_end(&mut big_buffer).unwrap();
    }

    if test_case != big_buffer {
        return 0;
    }
}
