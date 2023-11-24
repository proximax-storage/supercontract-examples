use std::io::{BufReader, Read, BufWriter, Write};

use crate::{internet, file::{FileWriter, FileReader}};

pub fn read_btc_price()-> i32 {
    let internet = internet::Internet::new("https://api.coingecko.com/api/v3/simple/price?ids=bitcoin&vs_currencies=usd", true).unwrap();
    let mut reader = BufReader::with_capacity(1024, internet);
    let mut internet_buffer = Vec::new();
    let res = reader.read_to_end(&mut internet_buffer).unwrap();
    if res == 0 {
        return 0;
    }

    {
        let file = FileWriter::new("text.txt").unwrap();
        let mut writer = BufWriter::new(file);
        writer.write(&internet_buffer).unwrap();
        writer.flush().unwrap();
    }

    let mut read_buffer = Vec::new();
    {
        let file = FileReader::new("text.txt").unwrap();
        let mut reader = BufReader::new(file);
        reader.read_to_end(&mut read_buffer).unwrap();
    }

    if internet_buffer != read_buffer {
        return 0;
    }

    return 1;
}