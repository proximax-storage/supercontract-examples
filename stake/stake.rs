use core::str;
use std::io::{Read, Write};

use blockchain::{
    get_block_height, get_call_params, get_service_payments, print_log, AggregateTransaction,
    EmbeddedTransaction,
};
use file::{FileReader, FileWriter};

pub mod blockchain;
pub mod dir_iterator;
pub mod file;
pub mod filesystem;
pub mod internet;

#[no_mangle]
pub unsafe extern "C" fn deposit_interest() -> u32 {
    {
        let mut file = match FileWriter::new("stake_list") {
            Ok(f) => f,
            Err(_) => panic!(),
        };

        if file.write(&[]).is_err() {
            panic!();
        }

        if file.flush().is_err() {
            panic!();
        }
    }
    return 0;
}

#[no_mangle]
pub unsafe extern "C" fn join() -> u32 {
    let mosaics = get_service_payments();

    // minimum stake value is 100
    if mosaics[0].amount < 100 {
        panic!();
    }
    let amount = mosaics[0].amount;
    let height = get_block_height();

    let mut input = get_call_params();
    input.extend_from_slice(&amount.to_le_bytes());
    input.extend_from_slice(&height.to_le_bytes());
    append_stake_list("participant", &input);

    return 0;
}

#[no_mangle]
pub unsafe extern "C" fn withdraw() -> u32 {
    let input = get_call_params();
    let address = str::from_utf8_unchecked(&input);
    let current_height = get_block_height();
    let stake_list = get_stake_list("stake_list");
    let duration = 50; // stake duration n blocks
    let mut reward = 0_u64;
    let interest_rate = 10_u64;

    let mut new_stake_list: Vec<u8> = Vec::new();

    for (address, amount, height) in stake_list {
        new_stake_list.extend_from_slice(address.as_bytes());
        new_stake_list.extend_from_slice(&amount.to_le_bytes());

        if address.as_bytes() != input {
            new_stake_list.extend_from_slice(&height.to_le_bytes());
            continue;
        }

        if current_height >= height + duration {
            reward += amount;
            new_stake_list.extend_from_slice(&u64::MAX.to_le_bytes()); // ardy claim
        }
    }

    // Update stake list
    {
        let mut file = match FileWriter::new("stake_list") {
            Ok(f) => f,
            Err(_) => panic!(),
        };

        if file.write(&new_stake_list).is_err() {
            panic!();
        }

        if file.flush().is_err() {
            panic!();
        }
    }

    // aggregate txn
    let mut aggregate = AggregateTransaction::default();
    aggregate.set_max_fee(1);

    // generate payload
    let mut address_byte = encode_address(address);
    let mosaic_id = 992621222383397347_u64.to_le_bytes();
    reward = reward * interest_rate / 100 + reward;
    let mosaic_amount = reward.to_le_bytes();
    address_byte.extend_from_slice(&vec![1, 0, 1, 0]);
    address_byte.extend_from_slice(&mosaic_id);
    address_byte.extend_from_slice(&mosaic_amount);

    // embedded txn
    let mut embedded = EmbeddedTransaction::default();
    embedded.set_entity_type(16724);
    embedded.set_version(3);
    embedded.set_payload(address_byte);
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

// get all stake accounts
pub unsafe fn get_stake_list(filename: &str) -> Vec<(String, u64, u64)> {
    let mut stake = Vec::new();
    let mut byte: Vec<u8> = Vec::new();

    {
        let mut file = match FileReader::new(filename) {
            Ok(f) => f,
            Err(_) => panic!(),
        };

        if file.read_to_end(&mut byte).is_err() {
            print_log("failed to read participant file");
            panic!();
        }
    }

    let mut index = 0;
    let pair_size = 40 + 8 + 8; // address, amount, height

    while index + pair_size <= byte.len() {
        // Decode the address (first 40 bytes of the pair)
        let address_bytes = &byte[index..index + 40];
        let address = match String::from_utf8((&address_bytes).to_vec()) {
            Ok(addr) => addr,
            Err(_) => panic!(),
        };

        // Decode the u64 number (next 8 bytes of the pair)
        let amount_bytes = &byte[index + 40..index + 48];
        let amount = match amount_bytes.try_into() {
            Ok(bytes) => u64::from_le_bytes(bytes),
            Err(_) => panic!(),
        };

        // Decode the u64 number (next 8 bytes of the pair)
        let height_bytes = &byte[index + 48..index + 56];
        let height = match height_bytes.try_into() {
            Ok(bytes) => u64::from_le_bytes(bytes),
            Err(_) => panic!(),
        };

        stake.push((address, amount, height));

        index += pair_size;
    }

    stake
}
