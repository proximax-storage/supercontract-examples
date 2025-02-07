use core::str;
use std::io::{Read, Write};

use blockchain::{
    get_call_params, get_service_payments, print_log, AggregateTransaction, EmbeddedTransaction,
};
use file::{FileReader, FileWriter};

pub mod blockchain;
pub mod dir_iterator;
pub mod file;
pub mod filesystem;
pub mod internet;

#[no_mangle]
pub unsafe extern "C" fn deposit_asset() -> u32 {
    let assets = get_service_payments();
    let asset_id = assets[0].mosaic_id;
    let asset_amount = assets[0].amount;
    let price_byte = get_call_params();
    let price: u64 = str::from_utf8_unchecked(&price_byte).parse().unwrap();
    {
        let mut file = match FileWriter::new("asset_id") {
            Ok(f) => f,
            Err(_) => panic!(),
        };

        if file.write(&asset_id.to_le_bytes()).is_err() {
            panic!();
        }

        if file.flush().is_err() {
            panic!();
        }
    }
    {
        let mut file = match FileWriter::new("asset_amount") {
            Ok(f) => f,
            Err(_) => panic!(),
        };

        if file.write(&asset_amount.to_le_bytes()).is_err() {
            panic!();
        }

        if file.flush().is_err() {
            panic!();
        }
    }
    {
        let mut file = match FileWriter::new("asset_price") {
            Ok(f) => f,
            Err(_) => panic!(),
        };

        if file.write(&price.to_le_bytes()).is_err() {
            panic!();
        }

        if file.flush().is_err() {
            panic!();
        }
    }
    return 0;
}

#[no_mangle]
pub unsafe extern "C" fn buy_asset() -> u32 {
    let mosaics = get_service_payments();
    let xpx_id = 992621222383397347_u64;
    let asset_price = get_file_to_u64("asset_price");
    if mosaics[0].mosaic_id != xpx_id || mosaics[0].amount < asset_price {
        panic!();
    }

    let owner_address = "SD2L2LRSBZUMYV2T34C4UXOIAAWX4TWQSQGBPMQO"; // example owner address
    let input = get_call_params();
    let buyer_address = str::from_utf8_unchecked(&input);
    let asset_id = get_file_to_u64("asset_id");
    let asset_amount = get_file_to_u64("asset_amount");

    // aggregate txn
    let mut aggregate = AggregateTransaction::default();
    aggregate.set_max_fee(1);

    // generate payload: transfer money to owner
    let mut owner_bytes = encode_address(owner_address);
    let mosaic_id = xpx_id.to_le_bytes();
    let mosaic_amount = asset_price.to_le_bytes();
    owner_bytes.extend_from_slice(&vec![1, 0, 1, 0]);
    owner_bytes.extend_from_slice(&mosaic_id);
    owner_bytes.extend_from_slice(&mosaic_amount);

    let mut embedded = EmbeddedTransaction::default();
    embedded.set_entity_type(16724);
    embedded.set_version(3);
    embedded.set_payload(owner_bytes);
    aggregate.add_embedded_transaction(embedded);

    // generate payload: transfer asset to buyer
    let mut buyer_bytes = encode_address(buyer_address);
    let mosaic_id = asset_id.to_le_bytes();
    let mosaic_amount = asset_amount.to_le_bytes();
    buyer_bytes.extend_from_slice(&vec![1, 0, 1, 0]);
    buyer_bytes.extend_from_slice(&mosaic_id);
    buyer_bytes.extend_from_slice(&mosaic_amount);

    let mut embedded = EmbeddedTransaction::default();
    embedded.set_entity_type(16724);
    embedded.set_version(3);
    embedded.set_payload(buyer_bytes);
    aggregate.add_embedded_transaction(embedded);

    blockchain::set_transaction(&aggregate);

    return 0;
}

// Utils

// function to convert address to base32 bytes
fn encode_address(address: &str) -> Vec<u8> {
    let mut out: Vec<u8> = Vec::new();
    static BASE32_ALPHABET: [u8; 32] = *b"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";
    let mut bits = 0u32;
    let mut bit_count = 0;
    for c in address.chars() {
        let value = BASE32_ALPHABET.iter().position(|&x| x as char == c);
        match value {
            Some(index) => {
                bits = (bits << 5) | index as u32;
                bit_count += 5;
                if bit_count >= 8 {
                    out.push((bits >> (bit_count - 8)) as u8);
                    bit_count -= 8;
                }
            }
            None => {
                println!("Invalid character in input: {}", c);
            }
        }
    }
    if bit_count > 0 {
        out.push((bits >> (bit_count - 8)) as u8);
    }
    out
}

// append new stake account
pub unsafe fn append_stake_list(filename: &str, input: &Vec<u8>) {
    let mut byte: Vec<u8> = Vec::new();
    {
        let mut file = FileReader::new(filename).unwrap();
        if file.read_to_end(&mut byte).is_err() {
            print_log("failed to read file");
            panic!();
        }
    }
    byte.extend(input);
    {
        let mut file = FileWriter::new(filename).unwrap();
        if file.write(&byte).is_err() {
            print_log("failed to write participant file");
            panic!();
        }
        if file.flush().is_err() {
            print_log("failed to flush participant file");
            panic!();
        }
    }
}

pub unsafe fn get_file_to_u64(filename: &str) -> u64 {
    let mut bytes: Vec<u8> = Vec::new();

    {
        let mut file = match FileReader::new(filename) {
            Ok(f) => f,
            Err(_) => panic!(),
        };

        if file.read_to_end(&mut bytes).is_err() {
            print_log("failed to read airdrop_amount file");
            panic!();
        }
    }

    if bytes.len() == 8 {
        let arr: [u8; 8] = bytes.try_into().expect("Vector has incorrect length");
        return u64::from_le_bytes(arr);
    } else {
        panic!();
    }
}
