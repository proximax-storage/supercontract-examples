pub mod blockchain;
pub mod dir_iterator;
pub mod file;
pub mod filesystem;
pub mod internet;

use blockchain::{get_call_params, get_service_payments, AggregateTransaction, EmbeddedTransaction};
use std::io::{Write, Read};
use file::{FileReader, FileWriter};

// Change name of function to a better one
#[no_mangle]
pub unsafe extern "C" fn deposit() -> u32 {
    let input = get_call_params();
    let mut file = FileWriter::new("price.txt").unwrap();
    file.write(&input).unwrap();
    file.flush().unwrap();
    return 0;
}

// Read data from file and check if value matches that of input
#[no_mangle]
pub unsafe extern "C" fn buy() -> u32 {
    let mosaic = get_service_payments();

    if mosaic.len() != 2 {
        panic!();
    }

    let mut file_contents: Vec<u8> = Vec::new();
    let mut file = FileReader::new("price.txt").unwrap();
    file.read_to_end(&mut file_contents).unwrap();

    if file_contents > mosaic[1].amount.to_le_bytes().to_vec(){
        panic!();
    }

    // seller transfer nft to buyer
    let mut buyer_address = encode_address("SB3Z7PF6P5WFNSJLLCPZAENN32XR7BHINFWJR6L6");
    let nft_id = mosaic.get(0).unwrap().mosaic_id.to_le_bytes();
    let nft_amount = mosaic.get(0).unwrap().amount.to_le_bytes();

    let mut aggregate = AggregateTransaction::default();
    aggregate.set_max_fee(1);

    buyer_address.extend_from_slice(&vec![1, 0, 1, 0]);
    buyer_address.extend_from_slice(&nft_id);
    buyer_address.extend_from_slice(&nft_amount);
    
    let mut embedded = EmbeddedTransaction::default();
    embedded.set_entity_type(16724);
    embedded.set_version(3);
    embedded.set_payload(buyer_address);
    aggregate.add_embedded_transaction(embedded);

    // buyer transfer xpx to seller
    let mut seller_address = encode_address("SD2L2LRSBZUMYV2T34C4UXOIAAWX4TWQSQGBPMQO");
    let mosaic_id = mosaic.get(1).unwrap().mosaic_id.to_le_bytes();
    let mosaic_amount = mosaic.get(1).unwrap().amount.to_le_bytes();
    seller_address.extend_from_slice(&vec![1, 0, 1, 0]);
    seller_address.extend_from_slice(&mosaic_id);
    seller_address.extend_from_slice(&mosaic_amount);

    // send the transactions
    let mut embedded = EmbeddedTransaction::default();
    embedded.set_entity_type(16724);
    embedded.set_version(3);
    embedded.set_payload(seller_address);
    aggregate.add_embedded_transaction(embedded);

    blockchain::set_transaction(&aggregate);
    return 0;
}

// function to convert address to bytes
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
