/*
File name: Staking supercontract
Version: 1.5
Patch notes: made a new string split terminator and integrated it with the system. 

Notes for other developers:
- MODIFY, means for u to modify when want to use
- PENDING, means awaiting confirmation 
- UPDATE, means for future updates

main account 
SD2L2LRSBZUMYV2T34C4UXOIAAWX4TWQSQGBPMQO

second account address
SD635GAAXIS6EHCEBLYDHJIRIHMMYAJMMMZH3YVC

third account address
SAONE2UIW6DIH6BXKAW4OTF44XMJSQ23OUES6YBB

empty accoutn address 
SBA3E4YPFXSDB4I6TXSRVDG6TZAOLP244AQSE3QA
address of empty wallet; message( can be anything ); mosaic

Use this for new staking supercontract with linear search 
main account
SD2L2LRSBZUMYV2T34C4UXOIAAWX4TWQSQGBPMQO;SBA3E4YPFXSDB4I6TXSRVDG6TZAOLP244AQSE3QA;Here you go;1000
third account
SAONE2UIW6DIH6BXKAW4OTF44XMJSQ23OUES6YBB;SBA3E4YPFXSDB4I6TXSRVDG6TZAOLP244AQSE3QA;Bye Bye money;1000
forth account
SBE7PCA4TVVM5NAN5RU7HJSTZ7PLJGSRIYH4IP3Q;SBA3E4YPFXSDB4I6TXSRVDG6TZAOLP244AQSE3QA;Ho Ho money go go;1000

SDBS66UKIK667MCYE6UKQ4ZHTFZTJ6AMFB4ONTUA;SBA3E4YPFXSDB4I6TXSRVDG6TZAOLP244AQSE3QA;XXXXXXXXXXXXXXXX;1000
*/
pub mod blockchain;
pub mod dir_iterator;
pub mod file;
pub mod filesystem;
pub mod internet;

use blockchain::get_block_height;
use blockchain::get_call_params;
use blockchain::{AggregateTransaction, EmbeddedTransaction};

use file::{FileReader, FileWriter};
use std::io::{ Read, Write};

/// # Objective
/// This function is used to deploy the supercontract onto the blockchain and write a file 
/// 
/// # Notes
/// This is the function used in the function name, along with the assignee, file name, function name, parameters
/// 
#[no_mangle]
pub unsafe extern "C" fn run() -> u32 {

    // creates file
    {
		let mut file = FileWriter::new("StakingSupercontractClientInformation.txt").unwrap();

		file.flush().unwrap();
	}

    return 0;
}

/// # Objective
/// This function aims to send xpx to a seperate account as a deposit, so how this function works is such that, 
/// B deploys file and supercontract to the blockchain using run()
/// A calls the supercontract to deposit money 
/// After a set time, A calls the supercontract again to withraw the deposit
/// 
/// # Notes 
/// - Sometimes it does not work because of the absence of the mosaic type and amount on the left box when deploying,
///     - Therefore, re-deploy and fill in that information, for example: Mosaic ID = 992621222383397347
/// - It could also be, because there is no parameter / incorrect parameter
///     - Therefore, re-deploy and fill in the correct parameter information
#[no_mangle]
pub unsafe extern "C" fn deposit() -> u32 {

    /////////////////////// inputs from storage tool "parameters tab" starts ///////////////////////
    // temp holds the cleaned up get_call_params
    let mut temp: Vec<String> = Vec::new();
    
    let status_get_clean_params = get_clean_call_params(&mut temp);

    // if the address is not valid, immediately fail
    // PENDING, want to use panic!(); or return 1;?
    if status_get_clean_params == 1 {

        // can choose to also panic!();
        return 1;
    }

    // to convert to pointers because rest of the code relies on &str not String
    // .iter() is to create an iterator so that can go through all of the items in the vectr
    // .map() is to convert the contents that is presented by the iterator
    // .collect() is to collect all the converted components
    let input_parts: Vec<&str> = temp.iter().map(|s| s.as_str()).collect();
    
    // address of the sender
    let sender_address = input_parts[0]; 

    // address of the receiver
    let address = input_parts[1]; 

    // message
    let message = input_parts[2]; 

    // total mosaics to be send
    let mosaic = input_parts[3]; 

    // this height is used to check condition for deposit return
    // why +80? cause its only an estimation, 1 block takes approx 15seconds to build, 
    // 1 block = 15s, required block heigh = wanted time taken/ needed before release of deposit
    // +80 is approx 5 mins, 15s * 80 , 
    // also lets say if we put 80, then best is to have the autorun run 80 times + 20
    // MODIFY
    let current_block_height = get_block_height() + 5;
    /////////////////////// inputs from storage tool "parameters tab" ends ///////////////////////
    
    

    /////////////////////// address conversion to base 32 starts ///////////////////////
    // this is the array that will store the information for the decoded address
    let mut decoded_address: Vec<u8> = Vec::new();

    // converts the string address into a Vec<u8>
    let status_decode_address: u8 = decode_address_string_to_u8(address, &mut decoded_address);

    // if the address is not valid, immediately fail
    // PENDING, want to use panic!(); or return 1;?
    if status_decode_address == 1 {

        // can choose to also panic!();
        return 1;
    }
    /////////////////////// address conversion to base 32 ends ///////////////////////
    

    
    /////////////////////// message conversion to base 32 starts ///////////////////////
    // this is the array that will store the message in bytes
    let mut message_in_bytes: Vec<u8> = Vec::new();

    // converts the message from String into Vec<u8> and attaches the type of mosaic we are transferring 
    let status_decode_message: u8 = decode_message_to_u8(message, &mut message_in_bytes, 0 );

    // PENDING
    if status_decode_message == 1{

        //panic!();
        return 1;
    }
    /////////////////////// message conversion to base 32 ends ///////////////////////
    

    
    /////////////////////// mosaic conversion to base 32 starts ///////////////////////
    // this is the array that will store the mosaic in bytes
    let mut mosaic_in_bytes = vec![0; 8];

    // changes the amount of mosaic from String to Vec<u8>
    let status_decode_mosaic_amount = decode_mosaic_amount_to_u8(mosaic, &mut mosaic_in_bytes);

    // PENDING
    if status_decode_mosaic_amount == 1{

        //panic!();
        return 1;
    }
    /////////////////////// mosaic conversion to base 32 ends ///////////////////////
    
    
    
    /////////////////////// transaction creation starts ///////////////////////
    // Combine the bytes from decoded_address, message_in_bytes, and mosaic_in_bytes
    let mut combined_bytes = Vec::new();
    combined_bytes.extend_from_slice(&decoded_address);
    combined_bytes.extend_from_slice(&message_in_bytes);
    combined_bytes.extend_from_slice(&mosaic_in_bytes);

    let status_create_transaction = create_aggregate_transaction(1, combined_bytes, 16724, 3);

    // PENDING
    if status_create_transaction == 1{

        //panic!();
        return 1;
        }
    /////////////////////// transaction creation starts ///////////////////////
    

    
    /////////////////////// write into file to check if return deposit is satisfied starts ///////////////////////
    // has the normal file write and save the height so that can be checked here to see if condition is fulfilled or not
    // amount to transfer ||| the block height that will allow the withrawal of deposit ||| the withrawal status
    let string_to_save = format!("A;key{};{};{};{};{};0;", sender_address,address,message,mosaic,current_block_height);

    // write the contents of this sender into the file
    let status_append = file_append("StakingSupercontractClientInformation.txt", &string_to_save );

    // if the address is not valid, immediately fail
    // PENDING, want to use panic!(); or return 1;?
    if status_append == 1 {

        // can choose to also panic!();
        return 1;
    }
    /////////////////////// write into file to check if return deposit is satisfied starts ///////////////////////
    
    return 0;
}

/// # Objective
/// This function aims to send xpx to the account which deposited the money, so how this function works is such that, 
/// B deploys file and supercontract to the blockchain using run()
/// A calls the supercontract to deposit money 
/// After a set time, A calls the supercontract again to withraw the deposit
/// 
/// # Notes 
/// - Sometimes it does not work because of the absence of the mosaic type and amount on the left box when deploying,
///     - Therefore, re-deploy and fill in that information, for example: Mosaic ID = 992621222383397347
/// - It could also be, because there is no parameter / incorrect parameter
///     - Therefore, re-deploy and fill in the correct parameter information
#[no_mangle]
pub unsafe extern "C" fn withdraw() -> u32 {

    /////////////////////// inputs from storage tool "parameters tab" starts ///////////////////////
    // temp holds the cleaned up get_call_params
    let mut temp: Vec<String> = Vec::new();
    
    let status_get_clean_params = get_clean_call_params(&mut temp);

    // if the param calls is not valid, immediately fail
    // PENDING, want to use panic!(); or return 1;?
    if status_get_clean_params == 1 {

        // can choose to also panic!();
        return 1;
    }

    // to convert to pointers because rest of the code relies on &str not String
    // .iter() is to create an iterator so that can go through all of the items in the vectr
    // .map() is to convert the contents that is presented by the iterator
    // .collect() is to collect all the converted components
    let input_parts: Vec<&str> = temp.iter().map(|s| s.as_str()).collect();

    // obtain information, still need to be checked
    // used as key
    let input_sender_address = input_parts[0];

    // used as verification
    let input_address = input_parts[1];
    let input_message = input_parts[2];
    let input_mosaic = input_parts[3];


    /////////////////////// inputs from storage tool "parameters tab" ends ///////////////////////
    

    
    /////////////////////// read and search for the supercontract caller starts ///////////////////////
    // this Vector will store the information about the target
    let mut result: Vec<String> = Vec::new();

    // this Vector is used to store data that exist within the file to avoid re-reading and lower the time complexity 
    let mut data: Vec<String> = Vec::new();

    // this Vector is used to store the contents of the file which is in the data variable above, but without target data
    let mut read_buffer : Vec<String> = Vec::new();

    // read the contents of the file and convert them into a Vector of String
    let read_status = file_read("StakingSupercontractClientInformation.txt", &mut data);

    // if the file dosent exist, immediately fail
    // PENDING, want to use panic!(); or return 1;?
    if read_status == 1 {

        // can choose to also panic!();
        return 1;
    }

    // to format how key will look like
    let target = format!("key{}",input_sender_address);

    // to search for the data 
    let linear_search_status = linear_search(&mut data, &target , 7,&mut read_buffer, &mut result);

    // if the key is not valid, immediately fail
    // PENDING, want to use panic!(); or return 1;?
    if linear_search_status == 1 {

        // can choose to also panic!();
        return 1;
    }
    /////////////////////// read and search for the supercontract caller ends ///////////////////////
    
    {
        // re-create the file
        let mut file = FileWriter::new("test.txt").unwrap();

        let mut converted_target_u8: Vec<u8> = result.clone().into_iter().flat_map(|s| s.into_bytes()).collect();

        let mut temp: Vec<u8> = data.clone().into_iter().flat_map(|s| s.into_bytes()).collect();

        // write target first
        file.write( &converted_target_u8).unwrap();

        file.write( &temp).unwrap();

        file.flush().unwrap();
    }


    
    /////////////////////// supercontract caller data extraction starts ///////////////////////
    // UPDATE, can make here more robust
    // all the indexing must be 0,2,4..2n cause of the ";"
    // get placeholder / extra character
    let read_placeholder = result[0].clone();

    // get the sender's address 
    let read_sender_address = result[2].clone();

    // get the receiver's address 
    let read_receiver_address = result[4].clone();

    let read_message = result[6].clone();

    // get the mosaic from the text file
    let mosaic = result[8].clone();

    // take the height
    let data_block_height = result[10].clone();

    // get the withrawal status 
    let withdrawal_status = result[12].clone();

    // convert the status to u32
    let converted_withdrawal_status = withdrawal_status.parse::<u32>().unwrap();

    // convert the height allowed to withdraw to i64
    let data_block_height_i64: i64 = match data_block_height.trim().parse() {
        Ok(integer) => integer,
        Err(_) => return 99,
    };
    /////////////////////// supercontract caller data extraction ends ///////////////////////
    


    
    /////////////////////// transaction starts ///////////////////////
    // get current block height
    let current_block_height: u64 = get_block_height();
    
    // convert to i64
    let current_block_height_i64: i64 = current_block_height as i64;

    // check if the condition has been met
    if current_block_height_i64 >= data_block_height_i64 && converted_withdrawal_status == 0{ 

        // PENDING
        
        /* 
        // which way do we want
        // method 1, use the data stored inside the text file
        let end_sender_address_index = read_sender_address.len();

        // UPDATE
        // this can be made more modular by using the function 
        let start_sender_address_index = 3;
        let send_address = &read_sender_address[start_sender_address_index..end_sender_address_index];
        */

        
        // method 2, use the input from users
        let send_address = input_sender_address;
        

        /////////////////////// address conversion to base 32 starts ///////////////////////
        // this is the array that will store the information for the decoded address
        let mut decoded_address: Vec<u8> = Vec::new();

        let status_decode_address: u8 = decode_address_string_to_u8(send_address, &mut decoded_address);

        // if the address is not valid, immediately fail
        // PENDING, want to use panic!(); or return 1;?
        if status_decode_address == 1 {

            // can choose to also panic!();
            return 1;
        }
        /////////////////////// address conversion to base 32 ends ///////////////////////



        /////////////////////// message conversion to base 32 starts ///////////////////////
        // this is the array that will store the message in bytes
        let mut message_in_bytes: Vec<u8> = Vec::new();
        let status_decode_message: u8 = decode_message_to_u8(&read_message, &mut message_in_bytes, 0 );

        // PENDING
        if status_decode_message == 1{

        //panic!();
        return 1;
        }
        /////////////////////// message conversion to base 32 ends ///////////////////////
         
        

        /////////////////////// message conversion to base 32 starts ///////////////////////
        // this is the array that will store the message in bytes
        let mut message_in_bytes: Vec<u8> = Vec::new();
        let status_decode_message: u8 = decode_message_to_u8(&read_message, &mut message_in_bytes, 0 );

        // PENDING
        if status_decode_message == 1{

        //panic!();
        return 1;
        }
        /////////////////////// message conversion to base 32 ends ///////////////////////
        


        /////////////////////// mosaic conversion to base 32 starts ///////////////////////
        // this is the array that will store the mosaic in bytes
        let mut mosaic_in_bytes = vec![0; 8];

        // get the amount of rewards for this account //
        // this is the variable that will store the mosaic that has been increased due to the staking 
        let mut rewarded_mosaic_in_string = String::from("");

        // get the clean amount of mosaic which is in String
        let status_get_clean_rewards = get_clean_rewards(&mosaic, &mut rewarded_mosaic_in_string);

        if status_get_clean_rewards == 1{
            
        // PENDING
        return 1;
        }

        // convert the String into &str for consistency
        let rewarded_mosaic_in_str: &str = rewarded_mosaic_in_string.as_str();

        // converts the string into a &[u8] 
        let status_decode_mosaic_amount = decode_mosaic_amount_to_u8(rewarded_mosaic_in_str, &mut mosaic_in_bytes);

        // PENDING
        if status_decode_mosaic_amount == 1{

        //panic!();
        return 1;
        }
        /////////////////////// mosaic conversion to base 32 ends ///////////////////////
        
        
        
        /////////////////////// transaction creation starts ///////////////////////
        // Combine the bytes from decoded_address, message_in_bytes, and mosaic_in_bytes
        let mut combined_bytes = Vec::new();
        combined_bytes.extend_from_slice(&decoded_address);
        combined_bytes.extend_from_slice(&message_in_bytes);
        combined_bytes.extend_from_slice(&mosaic_in_bytes);

        let status_create_transaction = create_aggregate_transaction(1, combined_bytes, 16724, 3);

        // PENDING
        if status_create_transaction == 1{

        //panic!();
        return 1;
        }
        /////////////////////// transaction creation ends ///////////////////////
        
        

        /////////////////////// status update starts ///////////////////////
        // change the withdrawal status to true
        // -2 because -1 is the ";"
        let mut index_status = result.len()-2;

        // set the status to true
        result[index_status] = "1".to_string();

        // convert the target information from String into [u8]
        let mut converted_target_u8: Vec<u8> = result.clone().into_iter().flat_map(|s| s.into_bytes()).collect();

        // convert the exisiting data from String into [u8]
        let mut converted_others_u8 : Vec<u8> = read_buffer.clone().into_iter().flat_map(|s| s.into_bytes()).collect();


        // PENDING, can use append?
        // can if there is a function to add ";" to the read buffer
        {
            // re-create the file
			let mut file = FileWriter::new("StakingSupercontractClientInformation.txt").unwrap();

            // write target first
            file.write(&converted_target_u8).unwrap();

            // rewrite whole content
            file.write(&converted_others_u8).unwrap();

			file.flush().unwrap();
		}
        /////////////////////// status update ends ///////////////////////

        return 0;
    } else {
        return 99;
    }
    /////////////////////// transaction ends ///////////////////////

    
}

/// # Objective
/// This function is used to decode a string to a vector of ASCII characters of the string which contain the address
/// 
/// # Parameters
/// - mestring_to_decodessage: the message in string
/// - decoded_address: the array that will contain the mosaic in ASCII 
/// 
/// # Examples
/// ```
/// // this is the array that will store the address in bytes
/// let address = "SBA3E4YPFXSDB4I6TXSRVDG6TZAOLP244AQSE3QA";
/// let mut decoded_address: Vec<u8> = Vec::new();
/// let status_decode_address: u8 = decode_address_string_to_u8(address, &mut decoded_address);
/// ```
/// 
/// # Notes
#[no_mangle]
fn decode_address_string_to_u8( string_to_decode:&str, decoded_address:&mut Vec<u8> ) -> u8 {
    // needs to return the result / status of this function
    let mut status: u8 = 0;
    
    // this is because the transaction only allows base 32
    // Your Base32 encoded string
    let encoded_address: &str = string_to_decode;

    // Define the Base32 alphabet as a static array
    // this is an encoding method
    // i guess its something like ceaser cypher
    static BASE32_ALPHABET: [u8; 32] = *b"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";

    let mut bits = 0u32;
    let mut bit_count = 0;

    // loop through the characters in the address of the account that will hold the deposit
    for c in encoded_address.chars() 
    {

        // |&x| x as char == c means
        // |&x| means a closure to pass the x as a reference
        // x as char means conversion of x to character type
        // .iter() is trying to consume the characters every iteration 
        // .position() is trying to obtain the index of the first character that is "== c"
        let value = BASE32_ALPHABET.iter().position(|&x| x as char == c);

        match value {

            // if the value is something
            Some(index) => {

                // firstly left shift bit by 5
                // secondly "bit wise or" operation is done with the index
                bits = (bits << 5) | index as u32;
                bit_count += 5;

                // after bit count = 8 start right shift
                if bit_count >= 8 {

                    // shift the bits to the right 
                    // then append / add them to the decoded address
                    decoded_address.push((bits >> (bit_count - 8)) as u8);
                    bit_count -= 8;
                }
            }
            None => {
                println!("Invalid character in input: {}", c);
                status = 1;
            }
        }
    }

    // If there are any remaining bits, add them to the result
    if bit_count > 0 {
        decoded_address.push((bits >> (bit_count - 8)) as u8);
    }

    return status;
    
}

/// # Objective
/// This function is used to decode a string to a vector of ASCII characters of the string which contain the message
/// 
/// # Parameters
/// - message: the message in string
/// - message_in_bytes: the array that will contain the mosaic in ASCII 
/// - wanted_type_of_mosaic: the wanted type of mosaic
///     - 0 = XPX = 246, 189, 22, 145, 161, 66, 251, 191
/// 
/// # Examples
/// ```
/// // this is the array that will store the message in bytes
/// let message = "hi there";
/// let mut message_in_bytes: Vec<u8> = Vec::new();
/// let status_decode_message: u8 = decode_message_to_u8(message, &mut message_in_bytes, 0 );
/// ```
/// 
/// # Notes 
#[no_mangle]
fn decode_message_to_u8( message: &str, message_in_bytes: &mut Vec<u8> , wanted_type_of_mosaic: u8 ) -> u8{
    // needs to return the result / status of this function
    let mut status = 0;

    //Message convert method
    // .chars() is an iterator 
    // .map() converts the elements to u8 
    // .collect() collects all the converted data and puts into the array 
    let result: Vec<u8> = message.chars().map(|c| c as u8).collect();

    let custom_byte: u8 = message.len() as u8 + 1;

    // this indicates how long the message is
    let values_to_append_at_start: Vec<u8> = vec![custom_byte, 0, 1, 0];

    // .extend(), appends / adds another array to the current array
    message_in_bytes.extend(values_to_append_at_start);
    message_in_bytes.extend(result);


    let mut type_of_mosaic: Vec<u8> = Vec::new(); 
    if  wanted_type_of_mosaic == 0 
    {
        // Types of Mosaic (XPX, Storage, etc...)
        // this is a fixed encoding for mosaic
        let temp: Vec<u8> = vec![246, 189, 22, 145, 161, 66, 251, 191];
        type_of_mosaic.extend(temp);
    }

    // append / add the type at the end
    message_in_bytes.extend(type_of_mosaic);

    return status;
}

/// # Objective
/// This function is used to decode a string to a vector of ASCII characters of the string which contain the amount of mosaic being transferred 
/// 
/// # Parameters
/// - mosaic: the amount in string type
/// - mosaic_in_bytes: the array that will contain the mosaic in ASCII 
/// 
/// # Examples
/// ```
/// // this is the array that will store the mosaic in bytes
/// let mosaic = "1000";
/// let mut mosaic_in_bytes = vec![0; 8];
/// let status_decode_mosaic_amount = decode_mosaic_amount_to_u8(mosaic, &mut mosaic_in_bytes);
/// ```
/// 
/// # Notes 
#[no_mangle]
fn decode_mosaic_amount_to_u8( mosaic: &str, mosaic_in_bytes: &mut Vec<u8> ) -> u8
{
    // needs to return the result / status of this function
    let mut status = 0;

    // Mosaic convert method
    // Convert the string to a u32 value
    let value = mosaic.parse::<u32>().unwrap();

    // Convert the u32 value to a Vec<u8> with 8 bytes
    mosaic_in_bytes[0..4].copy_from_slice(&value.to_le_bytes());

    return status;
}

/// # Objective
/// this function is designed to create an aggregate transaction 
/// 
/// # Parameters
/// - max_fee: The maximum fee for the supercontract
/// - combined_bytes: The Vector array that will store the payload or data, this includes:
///     - address
///     - message
///     - type of mosaic
///     - amount of mosaic
/// - entity_type: The type of transaction, for example:
///     - preparing a blockchain drive = 16738
/// - version_number: The version number 
/// 
/// # Example:
/// ```
/// // of course this Vector array needs to have proper bytes 
/// let mut combined_bytes = vec![144u8, 65u8, 178u8, 115u8, 15u8, 45u8, 228u8, 48u8, 241u8, 30u8, 157u8, 229u8, 26u8, 140u8, 222u8, 158u8, 64u8, 229u8, 191u8, 92u8, 224u8, 33u8, 34u8, 110u8, 0u8, 12u8, 0u8, 1u8, 0u8, 72u8, 101u8, 114u8, 101u8, 32u8, 121u8, 111u8, 117u8, 32u8, 103u8, 111u8, 246u8, 189u8, 22u8, 145u8, 161u8, 66u8, 251u8, 191u8, 232u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8]
/// let status_create_transaction = create_aggregate_transaction(1, combined_bytes, 16724, 3);
/// ``` 
/// 
/// # Notes
/// - If there is an issue, check the types for the parameters
/// - Check if the deployment is correct, should consist of:
///     - mosaic ID and mosaic amount in the left table
///     - parameters if needed
fn create_aggregate_transaction(  max_fee: u64 , combined_bytes: Vec<u8> , entity_type: u16 , version_number: u32 ) -> u8{
    // needs to return the result / status of this function
    let status: u8 = 0;

    let mut aggregate = AggregateTransaction::default();
    aggregate.set_max_fee(max_fee);

    let mut embedded = EmbeddedTransaction::default();

    // this value is fixed, for transfer
    // mosaic: 16724
    embedded.set_entity_type(entity_type); // Transfer Transaction
    
    embedded.set_version(version_number);
    embedded.set_payload(combined_bytes);

    aggregate.add_embedded_transaction(embedded);
    blockchain::set_transaction(&aggregate);

    return status;
}

/// # Objective
/// This function is used get the call parameters when calling / deploying supercontract and clean the data inputted
/// 
/// # Parameters
/// - input_parts: the vector array that will store the clean data
/// 
/// # Examples
/// ```
/// let mut temp: Vec<String> = Vec::new();
/// 
/// let status_get_clean_params = get_clean_call_params(&mut temp);
///
/// // to convert to pointers because rest of the code relies on &str not String
/// // .iter() is to create an iterator so that can go through all of the items in the vectr
/// // .map() is to convert the contents that is presented by the iterator
/// // .collect() is to collect all the converted components
/// let input_parts: Vec<&str> = temp.iter().map(|s| s.as_str()).collect();
/// ```
/// 
/// # Notes 
/// - Any form of data sanitization for input is done here
#[no_mangle]
fn get_clean_call_params( input_parts: &mut Vec<String> ) -> u8
{
    // needs to return the result / status of this function
    let mut status = 0;

    // gets the input in a Vec[u8] form
    let input_from_executer: Vec<u8>  = get_call_params();

    // convert the input to utf8 string
    // String::from_utf8() is trying to create a string from a Vec<u8>
    //.expect() is used to handle error from the above method, so if it fails, it will panic and will call this method,
    //      if no fail, will continue as normal
    let converted_u8_to_string: String = String::from_utf8(input_from_executer).expect("Failed to convert to String");

    // we .split() using ";" because thats how we want to split, returns &str
    // can also use | , : 
    // .map() to convert &str to String so that it's lifetime can be extended
    // .collect() to collect all the bits and parts of the result from the previous functions
    // eg input: SBA3E4YPFXSDB4I6TXSRVDG6TZAOLP244AQSE3QA;Here you go;1000
    let temp: Vec<String> = converted_u8_to_string.split(';').map(String::from).collect();

    // this is to add the result to the existing array
    input_parts.extend(temp);

    return status;
}

/// # Objective
/// This function is used to get the amount staked and return the amount after interest / reward from staking into the blockchain 
/// 
/// # Parameters
/// - mosaic: the amount in string type
/// - rewarded_mosaic_in_string: the string that will hold the new amount of mosaic after interest in String data type
/// 
/// # Examples
/// ```
/// let mosaic = "1000";
/// let mut rewarded_mosaic_in_string = String::from();
/// 
/// // get the clean amount of mosaic which is in String
/// get_clean_rewards(mosaic, &mut rewarded_mosaic_in_string);
/// 
/// // convert the String into &str for consistency
/// // this part is for the code above to ensure consistency, not neccessarily needed
/// let rewarded_mosaic_in_str: &str = rewarded_mosaic_in_string.as_str();
/// ```
/// 
/// # Notes 
/// This function can return String instead of the status, but during development was afraid that it would create problems
/// such as variable not living long enough.
/// This function returns the status to ensure consistency
#[no_mangle]
fn get_clean_rewards( mosaic: &str, rewarded_mosaic_in_string: &mut String ) -> u8
{
    // needs to return the result / status of this function
    let mut status = 0;

    // Convert the string to a u32 value
    let mut value = mosaic.parse::<u32>().unwrap();

    // PENDING
    // add function here that will give precise number of how many XPX to give back
    value += 100;

    // removes all element in the collection just to make sure that there is no additional data infront that
    // may or may not harm the system
    rewarded_mosaic_in_string.clear();

    // push in a reference to the string that has been converted,
    // why string? this is to keep the whole system as consistent as possible
    rewarded_mosaic_in_string.push_str(&value.to_string());

    return status;
}

/// # Objective
/// This function is used to read the contents of a file and output a Vec<String>
/// 
/// # Parameters
/// - file_path: the file name with .txt
/// - out: the Vector that will store the contents of the file
/// 
/// # Examples
/// ```
/// // create a file
/// {
///     let mut file = FileWriter::new("try.txt").unwrap();
///     for i in 0..10 
///     {
///         let temp = format!("A;key{};coo{};\n",i,i);
///         file.write(temp.as_bytes()).unwrap();
///     }
///     file.flush().unwrap();
/// }
/// 
/// let mut data: Vec<String> = Vec::new();
/// file_read("try.txt", &mut data);
/// ```
/// 
/// # Notes
fn file_read(file_path:&str, out:&mut Vec<String>) -> u8 {
    // needs to return the result / status of this function
    let mut status = 0;

    // basic read
    let mut file_content = Vec::new();
    {
        // find the file
        let mut file = FileReader::new(file_path).unwrap();

        // read everything in the file
        file.read_to_end(&mut file_content).unwrap();
    }

    // convert the data to 
    let file_content_in_string = String::from_utf8_lossy(&file_content);


    // PENDING
    let mut data_parts = Vec::new();
    let status_split = string_spliter_terminator(file_content_in_string.to_string(), ";",&mut data_parts);


    /* 
    // convert the data into String
    let data_parts: Vec<String> = file_content_in_string.split(';').map(|s| s.to_string()).collect(); 
    */

    // combine the data to the output array / vector
    out.extend_from_slice(&data_parts);

    return status;
}

/// # Objective
/// This function is used to linearly search through all the contents of a file for a specific data and extract it out
/// 
/// # Parameters
/// - data_array: the contents from the file in Vec<String> 
/// - target: the String we are looking for
/// - number_of_data_per_client: how many words are there seperated by a delimiter for 1 client / customer inside the text file
/// -- for example: 3 A;key001,key008
/// - read_buffer_modified: the output Vector that will store the contents of the file without the target inside of it
/// - wanted_data: the target data
/// 
/// # Examples
/// ```
/// // create a file
/// {
///     let mut file = FileWriter::new("try.txt").unwrap();
///     for i in 0..10 
///     {
///         let temp = format!("A;key{};coo{};\n",i,i);
///         file.write(temp.as_bytes()).unwrap();
///     }
///     file.flush().unwrap();
/// }
/// 
/// let mut result: Vec<String> = Vec::new();
/// let mut data: Vec<String> = Vec::new();
/// let mut read_buffer : Vec<String> = Vec::new();
/// file_read("try.txt", &mut data);
/// linear_search(&mut data, "key1", 7, &mut read_buffer, &mut result);
/// ```
/// 
/// # Notes
fn linear_search( data_array:&mut Vec<String>, target:&str, number_of_data_per_client:u8, read_buffer_modified:&mut Vec<String>, wanted_data:&mut Vec<String>) -> u8 {
    // needs to return the result / status of this function
    // set 1 by default, will change to 0 if target is found
    let mut status = 1;

    // MODIFY
    // this indicates the number jumps to perform 
    let number_of_jumps = 2;

    // while loop counter 
    let mut counter = 0;

    // MODIFY
    // please input the string here that will represent the prefix of the keys
    let other_keys = "key";

    // to loop through every data
    while counter < data_array.len() 
    {

        // if the data is the same as the target
        if data_array[counter] == target
        {
            // means we found the target
            status = 0;

            // this variable holds the data in String type
            let mut wanted_data_parts_in_string: Vec<String> = Vec::new();

            // sammple data:
            // A;keySD2L2LRSBZUMYV2T34C4UXOIAAWX4TWQSQGBPMQO;SBA3E4YPFXSDB4I6TXSRVDG6TZAOLP244AQSE3QA;Here you go;1000;329;0

            // MODIFY
            // if there is a change in the way the data is organised in the file, need to modify here accordingly

            // this is the counter used to extract the information
            // -2 because of the extra character so lets say if we found the target at index 2
            // we want to start at the extra character, so ned to 2 - x = 0 -> x = 2 -> 2 - 2 = 0
            let mut word_index = counter as u8 - number_of_jumps;

            // this is seperated from the word index for more readability
            // this is because the number of repetitions can be done like this 
            // number_of_data_per_client + counter 
            let mut number_of_repetitions_now = 0;

            while number_of_repetitions_now < number_of_data_per_client {

                println!("data: {} | wanted_data_parts_in_string: {:?}", data_array[word_index as usize], wanted_data_parts_in_string);

                // obtains the data itself and pushes it into the vector
                wanted_data_parts_in_string.push(data_array[word_index as usize].clone());
                
                // need to append the ";" for consistency 
                wanted_data_parts_in_string.push(";".to_string());

                // MODIFY
                // need to +2 here because the data_array has ";", so needs to skip over it
                // increment the index
                word_index += number_of_jumps;

                // increment the number of times repeated
                number_of_repetitions_now += 1;
            }

            // put the data in
            wanted_data.extend_from_slice(&wanted_data_parts_in_string);
            
        }

        // if its other keys 
        else if data_array[counter].starts_with(other_keys)
        {
            let mut read_buffer_temp :Vec<String> = Vec::new();

            // this is the counter used to extract the information
            // -1 because of the extra character
            let mut word_index = counter as u8 - number_of_jumps;

            let mut number_of_repetitions_now = 0;

            // -1 because if lets say we have 7 words, we start from -1, so lets say we found the target at index 1,
            // it will be like this 0,1,2,3,4,5,6 ( inclusive )
            while number_of_repetitions_now < number_of_data_per_client {

                read_buffer_temp.push(data_array[word_index as usize].clone());
                
                // need to append the ";" for consistency 
                read_buffer_temp.push(";".to_string());

                // increment the index
                // +2, cause need to skip over the ";" 
                word_index += number_of_jumps;

                // increment the number of times repeated
                number_of_repetitions_now += 1;
            }

            // put the data in
            read_buffer_modified.extend_from_slice(&read_buffer_temp);
            
        }
        else
        {}
        
        // PENDING
        // can increase the counter to reduce time complexity
        // increment the counter
        counter += 1;
    }

    return status;

}

/// # Objective
/// This function is used to append a string into an existing file by reading the contents of the original file and then re-writting the contents and the string that wants to be written
/// The reason for this, is due to the limitation of the file sdk where append was not possible
/// 
/// # Parameters
/// - file_path: the file name with the .txt
/// - content: the String in the type Vec<u8> to put inside the file
/// 
/// # Examples
/// ```
/// // create a file
/// {
///     let mut file = FileWriter::new("try.txt").unwrap();
///     for i in 0..10 
///     {
///         let temp = format!("A;key{};coo{};\n",i,i);
///         file.write(temp.as_bytes()).unwrap();
///     }
///     file.flush().unwrap();
/// }
/// 
/// // this is the text
/// let temp = Vec::from("cool\n");
/// file_append("try.txt", temp);
/// ```
/// 
/// # Notes
#[no_mangle]
fn file_append(file_path:&str , content: &String ) -> u32 {
    // needs to return the result / status of this function
    let mut status: u32 = 0;
    
    // this variable stores the information from the reader
    let mut file_content = Vec::new();
    {
        // find the file
        let mut file = FileReader::new(file_path).unwrap();

        // read everything in the file
        file.read_to_end(&mut file_content).unwrap();
    }

    // convert the information into utf8
    let file_content_in_string = String::from_utf8_lossy(&file_content);

	{
        // create a file
		let mut file = FileWriter::new(file_path).unwrap();

        // rewrite everything that existed already
        file.write(file_content_in_string.as_bytes()).unwrap();

        // write what is wanted
        file.write(content.as_bytes()).unwrap();
        
        // flush out any remiaining data
		file.flush().unwrap();
	}

    return status;
}

fn string_spliter_terminator(input:String , delimiter:&str, output:&mut Vec<String> ) -> u8 {

    let mut status = 0;

    let mut temp: Vec<char> = Vec::new();

    let mut char_iter = input.chars().peekable();

    while let Some( character ) = char_iter.next()
    {
        //println!("{}",character);

        // if the next character is "Something"
        // &next_char, & because if dont have it will be a reference to the char
        if let Some(&next_char) = char_iter.peek() 
        {

            // if the current character is not the delimiter
            if character.to_string() != delimiter
            {
                temp.push( character );
            }

            // if it is the delimiter
            else if character.to_string() == delimiter
            {
                // push the string
                output.push( temp.iter().collect() );

                // push the delimiter
                output.push( delimiter.to_string().clone() );

                temp.clear();
            }
        }

        // its the last string
        else
        {
            println!("{:?} char: {}",temp,character);

            // this is if there is a character before the last delimiter, add it to the list first then only add the delimiter outside of the if
            if temp.len() != 0 && character.to_string() != delimiter
            {

                // push the last character
                temp.push( character ) ;

                // push the last character in
                output.push( temp.iter().collect() );

                // clear the contents
                temp.clear();
            }
            else if temp.len() != 0 && character.to_string() == delimiter
            {
                // push the last character in
                output.push( temp.iter().collect() );

                // clear the contents
                temp.clear();

                // push the last character if there is no delimiter at the end
                // also used to push in the delimiter itself if its the last one
                temp.push( character ) ;

                // push the last character in
                output.push( temp.iter().collect() );

                // clear the contents
                temp.clear();
            }
            else if temp.len() == 0
            {

                // push the last character
                temp.push( character ) ;

                // push the last character in
                output.push( temp.iter().collect() );

                // clear the contents
                temp.clear();
            }
        }
    }

    return status;
}