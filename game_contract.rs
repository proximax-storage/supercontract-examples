pub mod blockchain;
pub mod dir_iterator;
pub mod file;
pub mod filesystem;
pub mod internet;

use blockchain::get_block_height;
use blockchain::get_call_params;
use blockchain::{AggregateTransaction, EmbeddedTransaction};

use std::io::Write;
use file::FileWriter;
use std::fs;

// Locking game developer's money (event prize) : to lock it we transfer the fund to another account, and send it back after certain duration
#[no_mangle]
pub unsafe extern "C" fn locked_prize() -> u32 {
    let input_from_executer = get_call_params();
    let converted_u8_to_string = String::from_utf8(input_from_executer).expect("Failed to convert to String");
    let input_parts: Vec<&str> = converted_u8_to_string.split(';').collect();

    let address = input_parts[0]; //Address to lokced place
    let message = input_parts[1]; //Message
    let mosaic = input_parts[2]; //Total mosaics to be send
    let current_block_height = get_block_height() + 40;

    let string_to_save = format!("{};{}", mosaic, current_block_height);

	{
		let mut file = FileWriter::new("DataGameDeveloper.txt").unwrap();
		file.write(&string_to_save.as_bytes()).unwrap();
		file.flush().unwrap();
	}

    // Your Base32 encoded string
    let encoded_address = address;

    // Define the Base32 alphabet as a static array
    static BASE32_ALPHABET: [u8; 32] = *b"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";​
    let mut decoded_address = Vec::new();
    let mut bits = 0u32;
    let mut bit_count = 0;

    for c in encoded_address.chars() {
        let value = BASE32_ALPHABET.iter().position(|&x| x as char == c);​
        match value {
            Some(index) => {
                bits = (bits << 5) | index as u32;
                bit_count += 5;

                if bit_count >= 8 {
                    decoded_address.push((bits >> (bit_count - 8)) as u8);
                    bit_count -= 8;
                }
            }
            None => {
                println!("Invalid character in input: {}", c);
                return 1;
            }
        }
    }​
    // If there are any remaining bits, add them to the result
    if bit_count > 0 {
        decoded_address.push((bits >> (bit_count - 8)) as u8);
    }

    //Message convert method
    let result: Vec<u8> = message.chars().map(|c| c as u8).collect();

    let custom_byte: u8 = message.len() as u8 + 1;

    let values_to_append_at_start: Vec<u8> = vec![custom_byte, 0, 1, 0];

    let mut message_in_bytes = values_to_append_at_start;
    message_in_bytes.extend(result);

    // Types of Mosaic (XPX, Storage, etc...)
    let type_of_mosaic: Vec<u8> = vec![246, 189, 22, 145, 161, 66, 251, 191];

    message_in_bytes.extend(type_of_mosaic);

    //Mosaic convert method
    // Convert the string to a u32 value
    let value = mosaic.parse::<u32>().unwrap();

    // Convert the u32 value to a Vec<u8> with 8 bytes
    let mut mosaic_in_bytes = vec![0; 8];
    mosaic_in_bytes[0..4].copy_from_slice(&value.to_le_bytes());

    // Combine the bytes from decoded_address, message_in_bytes, and mosaic_in_bytes
    let mut combined_bytes = Vec::new();
    combined_bytes.extend_from_slice(&decoded_address);
    combined_bytes.extend_from_slice(&message_in_bytes);
    combined_bytes.extend_from_slice(&mosaic_in_bytes);

    // Put the converted payload into transaction
    let mut aggregate = AggregateTransaction::default();
    aggregate.set_max_fee(1);

    let mut embedded = EmbeddedTransaction::default();
    embedded.set_entity_type(16724); // Transfer Transaction
    embedded.set_version(3);
    embedded.set_payload(combined_bytes);
    // SBA3E4YPFXSDB4I6TXSRVDG6TZAOLP244AQSE3QA;Here you go;1000

    aggregate.add_embedded_transaction(embedded);
    blockchain::set_transaction(&aggregate);

    return 0;
}

// This will check the duration at every blockheight, if the duration due, it will release the game developer's money (locked prize)
// by sending it back to the game developer
#[no_mangle]
pub unsafe extern "C" fn send_to_winner() -> u32 { //run as a Automatic Execution File Name
    let file_path = "DataGameDeveloper.txt";

    let mut file_content = Vec::new();
    {
        let file = FileReader::new(file_path).unwrap();
        let mut reader = BufReader::new(file);
        reader.read_to_end(&mut file_content).unwrap();
    }
    let file_content_in_string = String::from_utf8_lossy(&file_content);

    let data_parts: Vec<&str> = file_content_in_string.split(';').collect(); //1000;40
    let data_block_height = data_parts[1];

    let data_block_height_i64: i64 = match data_block_height.trim().parse() {
        Ok(integer) => integer,
        Err(_) => return 99,
    };

    let current_block_height: u64 = get_block_height();
    let current_block_height_i64: i64 = current_block_height as i64;

    if current_block_height_i64 == data_block_height_i64 {
        let address = "819F72066B17FFD71B8B4142C5AEAE4B997B0882ABDF2C263B02869382BD93A0"; //Address to game developer
        let message = "I sent the mosaic back"; //Message
        let mosaic = data_parts[0]; //Total mosaics that has been inputed before

        // Your Base32 encoded string
        let encoded_address = address;​
        // Define the Base32 alphabet as a static array
        static BASE32_ALPHABET: [u8; 32] = *b"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";

        let mut decoded_address = Vec::new();
        let mut bits = 0u32;
        let mut bit_count = 0;​
        for c in encoded_address.chars() {
            let value = BASE32_ALPHABET.iter().position(|&x| x as char == c);

            match value {
                Some(index) => {
                    bits = (bits << 5) | index as u32;
                    bit_count += 5;​
                    if bit_count >= 8 {
                        decoded_address.push((bits >> (bit_count - 8)) as u8);
                        bit_count -= 8;
                    }
                }
                None => {
                    println!("Invalid character in input: {}", c);
                    return 1;
                }
            }
        }

        // If there are any remaining bits, add them to the result
        if bit_count > 0 {
            decoded_address.push((bits >> (bit_count - 8)) as u8);
        }​
        //Message convert method
        let result: Vec<u8> = message.chars().map(|c| c as u8).collect();

        let custom_byte: u8 = message.len() as u8 + 1;​
        let values_to_append_at_start: Vec<u8> = vec![custom_byte, 0, 1, 0];

        let mut message_in_bytes = values_to_append_at_start;
        message_in_bytes.extend(result);​
        // Values to append at the end
        let type_of_mosaic: Vec<u8> = vec![246, 189, 22, 145, 161, 66, 251, 191];

        message_in_bytes.extend(type_of_mosaic);

        //Mosaic convert method
        // Convert the string to a u32 value
        let value = mosaic.parse::<u32>().unwrap();

        // Convert the u32 value to a Vec<u8> with 8 bytes
        let mut mosaic_in_bytes = vec![0; 8];
        mosaic_in_bytes[0..4].copy_from_slice(&value.to_le_bytes());

        // Combine the bytes from decoded_address, message_in_bytes, and mosaic_in_bytes
        let mut combined_bytes = Vec::new();
        combined_bytes.extend_from_slice(&decoded_address);
        combined_bytes.extend_from_slice(&message_in_bytes);
        combined_bytes.extend_from_slice(&mosaic_in_bytes);

        let mut aggregate = AggregateTransaction::default();
        aggregate.set_max_fee(1);

        let mut embedded = EmbeddedTransaction::default();
        embedded.set_entity_type(16724); // Transfer Transaction
        embedded.set_version(3);
        embedded.set_payload(combined_bytes);

        aggregate.add_embedded_transaction(embedded);
        blockchain::set_transaction(&aggregate);

        return 0;
    } else {
        return 99;
    }
}