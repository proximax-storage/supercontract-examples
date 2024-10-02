pub mod blockchain;
pub mod dir_iterator;
pub mod file;
pub mod filesystem;
pub mod internet;

use blockchain::{get_call_params, get_service_payments, print_log, AggregateTransaction, EmbeddedTransaction};
use file::{FileReader, FileWriter};

use std::io::{Read, Write};

#[no_mangle]
pub unsafe extern "C" fn test() -> u32 {
    let input = get_call_params();
    print_log(&input.len().to_string());
    return 0;
}

// init supercontract (create files to store important data)
#[no_mangle]
pub unsafe extern "C" fn init() -> u32 {
    {
        let mut file = match FileWriter::new("height") {
            Ok(f) => f,
            Err(_) => return 2,
        };

        if file.write(&[]).is_err() {
            return 2;
        }

        if file.flush().is_err() {
            return 2;
        }
    }
    {
        let mut file = match FileWriter::new("participant") {
            Ok(f) => f,
            Err(_) => return 2,
        };

        if file.write(&[]).is_err() {
            return 2;
        }

        if file.flush().is_err() {
            return 2;
        }
    }
    return 0;
}

#[no_mangle]
pub unsafe extern "C" fn set_height() -> u32 {
    let input = get_call_params();
    {
        let mut file = match FileWriter::new("height") {
            Ok(f) => f,
            Err(_) => return 2,
        };

        if file.write(&input).is_err() {
            return 2;
        }
        if file.flush().is_err() {
            return 2;
        }
    }
    return 0;
}

#[no_mangle]
pub unsafe extern "C" fn deposit() -> u32 {
    return 0;
}

#[no_mangle]
pub unsafe extern "C" fn prejoin() -> u32 {
    let input = get_call_params();

    if input.len() < 40 {
        return 2; // Ensure input is at least 40 bytes long
    }

    let (address_bytes, amount_bytes) = input.split_at(40);
    let amount_u64: u64 = match std::str::from_utf8(amount_bytes) {
        Ok(amount_str) => match amount_str.parse::<u64>() {
            Ok(t) => t,
            Err(_) => return 2, // Return error if parsing fails
        },
        Err(_) => return 2, // Return error if UTF-8 conversion fails
    };
    let mut data: Vec<u8> = Vec::with_capacity(40 + 8);
    data.extend_from_slice(address_bytes);
    data.extend_from_slice(&amount_u64.to_le_bytes());

    append_participant(&data);
    print_participant();
    return 0;
}

#[no_mangle]
pub unsafe extern "C" fn join() -> u32 {
    let mosaics = get_service_payments();
    let amount = mosaics[0].amount;
    let mut input = get_call_params();
    input.extend_from_slice(&amount.to_le_bytes());
    append_participant(&input);
    print_participant();
    return 0;
}

#[no_mangle]
pub unsafe extern "C" fn distribute() -> u32 {
    let total_airdrop = 10000;
    print_participant();
    // read participant file
    let participant = get_participant();
    let total_xpx: u64 = participant.iter().map(|(_, amount)| amount).sum();
    
    if total_xpx == 0 {
        return 2;
    }
    let cake = total_airdrop/total_xpx;
    if cake == 0 {
        return 2; // Handle cases where airdrop distribution is too small
    }
    const SLICE: [u8; 4] = [1, 0, 1, 0];
    const MOSAIC_ID: u64 = 992621222383397347_u64;

    
    let mut aggregate = AggregateTransaction::default();
    aggregate.set_max_fee(100);
    for (address, amount) in &participant{
        let mut payload: Vec<u8> = Vec::with_capacity(45);
        let address_byte = encode_address(address);
        let reward: u64 = amount*cake;

        payload.extend_from_slice(&address_byte);
        payload.extend_from_slice(&SLICE);
        payload.extend_from_slice(&MOSAIC_ID.to_le_bytes());
        payload.extend_from_slice(&reward.to_le_bytes());

        // embedded txn
        let mut embedded = EmbeddedTransaction::default();
        embedded.set_entity_type(16724);
        embedded.set_version(3);
        embedded.set_payload(payload);
        aggregate.add_embedded_transaction(embedded);
    }

    blockchain::set_transaction(&aggregate);
    return 0;
}


// utils
pub unsafe fn get_participant() -> Vec<(String, u64)> {
    let mut participant = Vec::new();
    let mut byte: Vec<u8> = Vec::new();
    
    {
        let mut file = match FileReader::new("participant") {
            Ok(f) => f,
            Err(_) => panic!(),
        };

        if file.read_to_end(&mut byte).is_err() {
            print_log("failed to read participant file");
            panic!();
        }
    }

    let mut index = 0;
    let pair_size = 48;

    while index + pair_size <= byte.len() {
        // Decode the address (first 40 bytes of the pair)
        let address_bytes = &byte[index..index + 40];
        let address = match String::from_utf8(address_bytes.to_vec()) {
            Ok(addr) => addr,
            Err(_) => panic!(), 
        };

        // Decode the u64 number (next 8 bytes of the pair)
        let amount_bytes = &byte[index + 40..index + 48];
        let amount = match amount_bytes.try_into() {
            Ok(bytes) => u64::from_le_bytes(bytes),
            Err(_) => panic!(),
        };
        
        participant.push((address, amount));

        index += pair_size;
    }

    participant
}

pub unsafe fn print_participant() {
    let mut participant = Vec::new();
    let mut byte: Vec<u8> = Vec::new();
    {
        let mut file = match FileReader::new("participant") {
            Ok(f) => f,
            Err(_) => panic!(),
        };

        if file.read_to_end(&mut byte).is_err() {
            print_log("failed to read participant file");
            panic!();
        }
    }

    let mut index = 0;
    let pair_size = 48;

    while index + pair_size <= byte.len() {
        // Decode the address (first 40 bytes of the pair)
        let address_bytes = &byte[index..index + 40];
        let address = String::from_utf8(address_bytes.to_vec()).expect("Failed to decode address string");

        // Decode the u64 number (next 8 bytes of the pair)
        let amount_bytes = &byte[index + 40..index + 48];
        let amount = u64::from_le_bytes(amount_bytes.try_into().expect("Failed to decode u64 number"));
        
        print_log(&address);
        print_log(&amount.to_string());
        participant.push((address, amount));

        index += pair_size;
    }
}

pub unsafe fn append_participant(input: &Vec<u8>) {
    let mut byte: Vec<u8> = Vec::new();
    {
        let mut file = FileReader::new("participant").unwrap();
        if file.read_to_end(&mut byte).is_err() {
            print_log("failed to read participant file");
            panic!();
        }
    }
    byte.extend(input);
    {
        let mut file = FileWriter::new("participant").unwrap();
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

// // 9 define participate fee
// #[no_mangle]
// pub unsafe extern "C" fn setParticipateFee() -> u32 {
//     // read user input and write to fee file
//     let input = get_call_params();
//     let mut file = FileWriter::new("fee").unwrap();
//     file.write(&input).unwrap();
//     file.flush().unwrap();
//     return 0;
// }