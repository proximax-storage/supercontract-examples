pub mod blockchain;
pub mod dir_iterator;
pub mod file;
pub mod filesystem;
pub mod internet;

use blockchain::{get_block_height, get_call_params, get_service_payments, print_log, AggregateTransaction, EmbeddedTransaction};
use file::{FileReader, FileWriter};

use core::{panic, str};
use std::io::{Read, Write};

#[no_mangle]
pub unsafe extern "C" fn init() -> u32 {
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
    {
        let mut file = match FileWriter::new("prejoin") {
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
pub unsafe extern "C" fn set_airdrop_divisibility() -> u32 {
    // if get_caller_public_key() != "".as_bytes() {
    //     return 2;
    // }

    let input = get_call_params();
    {
        let mut file = match FileWriter::new("airdrop_div") {
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
pub unsafe extern "C" fn set_end_height() -> u32 {
    // if get_caller_public_key() != "".as_bytes() {
    //     return 2;
    // }

    let input = get_call_params();
    {
        let mut file = match FileWriter::new("airdrop_end") {
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
pub unsafe extern "C" fn set_participate_fee() -> u32 {
    // if get_caller_public_key() != "".as_bytes() {
    //     return 2;
    // }

    let input = get_call_params();
    let mut file = FileWriter::new("participate_fee").unwrap();
    file.write(&input).unwrap();
    file.flush().unwrap();
    return 0;
}

#[no_mangle]
pub unsafe extern "C" fn deposit() -> u32 {
    // if get_caller_public_key() != "".as_bytes() {
    //     return 2;
    // }

    let mosaic = &get_service_payments()[0];
    {
        let mut file = match FileWriter::new("airdrop_id") {
            Ok(f) => f,
            Err(_) => return 2,
        };

        if file.write(&mosaic.mosaic_id.to_le_bytes()).is_err() {
            return 2;
        }

        if file.flush().is_err() {
            return 2;
        }
    }
    {
        let mut file = match FileWriter::new("airdrop_amount") {
            Ok(f) => f,
            Err(_) => return 2,
        };

        if file.write(&mosaic.amount.to_le_bytes()).is_err() {
            return 2;
        }

        if file.flush().is_err() {
            return 2;
        }
    }
    return 0;
}

#[no_mangle]
pub unsafe extern "C" fn prejoin() -> u32 {
    // if get_caller_public_key() != "".as_bytes() {
    //     return 2;
    // }

    let input = get_call_params();

    if input.len() < 40 {
        print_log("no 40 len");
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

    append_participant("prejoin", &data);
    return 0;
}

#[no_mangle]
pub unsafe extern "C" fn join() -> u32 {
    if get_block_height() >= get_end_height() {
        return 2;
    }
    
    let mosaics = get_service_payments();
    let participate_fee = get_participate_fee();
    let mut amount = mosaics[0].amount;

    if amount < participate_fee + 1 {
        return 2;
    }

    amount -= participate_fee;
    
    let mut input = get_call_params();
    input.extend_from_slice(&amount.to_le_bytes());
    append_participant("participant", &input);

    // aggregate txn
    let mut aggregate = AggregateTransaction::default();
    aggregate.set_max_fee(1);

    // generate payload
    let mosaic = get_service_payments();
    let address = "SB3Z7PF6P5WFNSJLLCPZAENN32XR7BHINFWJR6L6";
    let mut address_byte = encode_address(address);
    let mosaic_id = mosaic.get(0).unwrap().mosaic_id.to_le_bytes();
    let mosaic_amount = mosaic.get(0).unwrap().amount.to_le_bytes();
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

#[no_mangle]
pub unsafe extern "C" fn distribute() -> u32 {
    // if get_caller_public_key() != "".as_bytes() {
    //     return 2;
    // }

    const SLICE: [u8; 4] = [1, 0, 1, 0];
    let mosaic_id: u64 = get_airdrop_id();
    let mut total_airdrop = get_airdrop_amount();

    if total_airdrop <= 0 {
        return 2;
    }
    
    let mut aggregate = AggregateTransaction::default();
    aggregate.set_max_fee(100);
    let prejoin = get_participant("prejoin");
    for (address, amount) in &prejoin{
        print_log(&total_airdrop.to_string());
        let mut payload: Vec<u8> = Vec::with_capacity(45);
        let address_byte = encode_address(address);
        
        payload.extend_from_slice(&address_byte);
        payload.extend_from_slice(&SLICE);
        payload.extend_from_slice(&mosaic_id.to_le_bytes());
        payload.extend_from_slice(&amount.to_le_bytes());
        
        // embedded txn
        let mut embedded = EmbeddedTransaction::default();
        embedded.set_entity_type(16724);
        embedded.set_version(3);
        embedded.set_payload(payload);
        aggregate.add_embedded_transaction(embedded);

        total_airdrop = total_airdrop - amount;
        print_log(&total_airdrop.to_string());
    }

    // read participant file
    let participant = get_participant("participant");
    let total_xpx: u64 = participant.iter().map(|(_, amount)| amount).sum();
    
    if total_xpx <= 0 {
        return 2;
    }
    let cake = total_airdrop/total_xpx;
    if cake == 0 {
        return 2; // Handle cases where airdrop distribution is too small
    }
    
    for (address, amount) in &participant{
        print_log(&address);
        print_log(&amount.to_string());
        let mut payload: Vec<u8> = Vec::with_capacity(45);
        let address_byte = encode_address(address);
        let reward: u64 = amount*cake;

        payload.extend_from_slice(&address_byte);
        payload.extend_from_slice(&SLICE);
        payload.extend_from_slice(&mosaic_id.to_le_bytes());
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
pub unsafe fn get_participant(filename: &str) -> Vec<(String, u64)> {
    let mut participant = Vec::new();
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
    let pair_size = 48;

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
        
        participant.push((address, amount));

        index += pair_size;
    }

    participant
}

pub unsafe fn get_participate_fee() -> u64 {
    let mut byte: Vec<u8> = Vec::new();
    
    {
        let mut file = match FileReader::new("participate_fee") {
            Ok(f) => f,
            Err(_) => panic!(),
        };

        if file.read_to_end(&mut byte).is_err() {
            print_log("failed to read participant file");
            panic!();
        }
    }

    let fee_str = match str::from_utf8(&byte) {
        Ok(s) => s,
        Err(_) => panic!(),
    };

    let fee_u64: u64 = match fee_str.parse() {
        Ok(n) => n,
        Err(_) => panic!(),
    };
    
    fee_u64
}

pub unsafe fn get_end_height() -> u64 {
    let mut byte: Vec<u8> = Vec::new();
    
    {
        let mut file = match FileReader::new("airdrop_end") {
            Ok(f) => f,
            Err(_) => panic!(),
        };

        if file.read_to_end(&mut byte).is_err() {
            print_log("failed to read participant file");
            panic!();
        }
    }

    let height_str = match str::from_utf8(&byte) {
        Ok(s) => s,
        Err(_) => panic!(),
    };

    let height_u64: u64 = match height_str.parse() {
        Ok(n) => n,
        Err(_) => panic!(),
    };
    
    height_u64
}

pub unsafe fn get_airdrop_amount() -> u64 {
    let mut byte: Vec<u8> = Vec::new();
    
    {
        let mut file = match FileReader::new("airdrop_amount") {
            Ok(f) => f,
            Err(_) => panic!(),
        };

        if file.read_to_end(&mut byte).is_err() {
            print_log("failed to read airdrop_amount file");
            panic!();
        }
    }

    if byte.len() == 8 {
        let arr: [u8; 8] = byte.try_into().expect("Vector has incorrect length");
        return u64::from_le_bytes(arr);
    }else {
        return 0;
    }
}

pub unsafe fn get_airdrop_id() -> u64 {
    let mut byte: Vec<u8> = Vec::new();
    
    {
        let mut file = match FileReader::new("airdrop_id") {
            Ok(f) => f,
            Err(_) => panic!(),
        };

        if file.read_to_end(&mut byte).is_err() {
            print_log("failed to read airdrop_id file");
            panic!();
        }
    }

    if byte.len() == 8 {
        let arr: [u8; 8] = byte.try_into().expect("Vector has incorrect length");
        return u64::from_le_bytes(arr);
    }else {
        return 0;
    }
}

pub unsafe fn append_participant(filename: &str, input: &Vec<u8>) {
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

pub fn get_divisibility(amount: u64, n: u32) -> u64 {
    return amount * 10_u64.pow(n);
}