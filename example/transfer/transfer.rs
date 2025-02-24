use std::str;

use blockchain::{get_call_params, get_service_payments, AggregateTransaction, EmbeddedTransaction};

pub mod blockchain;
pub mod dir_iterator;
pub mod file;
pub mod filesystem;
pub mod internet;

#[no_mangle]
pub unsafe extern "C" fn transfer() -> u32 {
    // aggregate txn
    let mut aggregate = AggregateTransaction::default();
    aggregate.set_max_fee(1);

    // generate payload
    let mosaic = get_service_payments();
    let input = get_call_params();
    let address = str::from_utf8_unchecked(&input);
    let mut address_byte = encode_address(address);
    let mosaic_id = mosaic.get(0).unwrap().mosaic_id.to_le_bytes();
    let mosaic_amount = mosaic.get(0).unwrap().amount.to_le_bytes();
    address_byte.extend_from_slice(&vec![1, 0, 1, 0]);
    address_byte.extend_from_slice(&mosaic_id);
    address_byte.extend_from_slice(&mosaic_amount);

    // embedded txn
    let mut embedded = EmbeddedTransaction::default();
    // 16724 is the entity type for transfer transactions
    embedded.set_entity_type(16724);
    embedded.set_version(3);
    embedded.set_payload(address_byte);
    aggregate.add_embedded_transaction(embedded);
    blockchain::set_transaction(&aggregate);

    return 0;
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