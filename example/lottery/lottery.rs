use core::str;
use std::io::{Read, Write};

use blockchain::{
    get_block_time, get_call_params, get_service_payments, print_log,
    AggregateTransaction, EmbeddedTransaction,
};
use file::{FileReader, FileWriter};

pub mod blockchain;
pub mod dir_iterator;
pub mod file;
pub mod filesystem;
pub mod internet;

#[no_mangle]
pub unsafe extern "C" fn init() -> u32 {
    {
        let mut file = match FileWriter::new("players") {
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
    let players = get_account_list("players");
    if players.len() >= 20 {
        print_log("lottery sold out");
        panic!();
    } 

    let mosaics = get_service_payments();
    let xpx_id = 992621222383397347_u64;
    let min_bet = 100;
    if mosaics[0].mosaic_id != xpx_id || mosaics[0].amount < min_bet {
        panic!();
    }

    let mut address = get_call_params();
    address.extend_from_slice(&mosaics[0].amount.to_le_bytes());

    append_file("players", &address);
    return 0;
}

#[no_mangle]
pub unsafe extern "C" fn distribute() -> u32 {
    let players = get_account_list("players");
    let xpx_id = 992621222383397347_u64;
    if players.len() < 20 {
        // return ticket back to player
        let mut aggregate = AggregateTransaction::default();
        aggregate.set_max_fee(1);

        for (address, amount) in players {
            // generate payload: transfer asset to buyer
            let mut player_bytes = encode_address(&address);
            let mosaic_id = xpx_id.to_le_bytes();
            let mosaic_amount = amount.to_le_bytes();
            player_bytes.extend_from_slice(&vec![1, 0, 1, 0]);
            player_bytes.extend_from_slice(&mosaic_id);
            player_bytes.extend_from_slice(&mosaic_amount);

            let mut embedded = EmbeddedTransaction::default();
            embedded.set_entity_type(16724);
            embedded.set_version(3);
            embedded.set_payload(player_bytes);
            aggregate.add_embedded_transaction(embedded);
        }
        blockchain::set_transaction(&aggregate);
    } else {
        let prize = players.len()-1 * 100;
        let rand = get_block_time() % 20;
        let winner = &players[rand as usize].0;

        let mut aggregate = AggregateTransaction::default();
        aggregate.set_max_fee(1);
        let mut buyer_bytes = encode_address(&winner);
        let mosaic_id = xpx_id.to_le_bytes();
        let mosaic_amount = prize.to_le_bytes();
        buyer_bytes.extend_from_slice(&vec![1, 0, 1, 0]);
        buyer_bytes.extend_from_slice(&mosaic_id);
        buyer_bytes.extend_from_slice(&mosaic_amount);

        let mut embedded = EmbeddedTransaction::default();
        embedded.set_entity_type(16724);
        embedded.set_version(3);
        embedded.set_payload(buyer_bytes);
        aggregate.add_embedded_transaction(embedded);
        blockchain::set_transaction(&aggregate);
    }

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
pub unsafe fn append_file(filename: &str, input: &Vec<u8>) {
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

// get all funding accounts
pub unsafe fn get_account_list(filename: &str) -> Vec<(String, u64)> {
    let mut accounts = Vec::new();
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

        accounts.push((address, amount));

        index += pair_size;
    }

    accounts
}
