/*
File name: Staking supercontract
Version: 1.8
Patch notes: can now handle multiple deposits made by the same person. Also fixed logic error, where can deposit earlier when there is multiple deposits

Notes for other developers:
- MODIFY, means for u to modify when want to use, potential areas with bugs 
- PENDING, means awaiting confirmation 
- UPDATE, means for future updates

main account 
SD2L2LRSBZUMYV2T34C4UXOIAAWX4TWQSQGBPMQO

second account address
SD635GAAXIS6EHCEBLYDHJIRIHMMYAJMMMZH3YVC

third account address
SAONE2UIW6DIH6BXKAW4OTF44XMJSQ23OUES6YBB

empty acount address 
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
use blockchain::get_caller_public_key;
use blockchain::{AggregateTransaction, EmbeddedTransaction};

use file::{FileReader, FileWriter};
use std::io::{ Read, Write};
use std::vec;

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
    let current_block_height = get_block_height() + 30;
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

    // create the aggregate transaction
    let status_create_transaction = create_aggregate_transaction(1, combined_bytes, 16724, 3);

    // PENDING
    if status_create_transaction == 1{

        //panic!();
        return 1;
        }
    /////////////////////// transaction creation ends ///////////////////////
    


    /////////////////////// get caller public starts ///////////////////////
    // get the caller public key
    let caller_public_key_vector_u8: [u8; 32] = get_caller_public_key();

    // this variable stores the data of the public key in an array with only one element
    let mut caller_public_key_vector_string: Vec<String> = Vec::new();

    // convert the public key to a string to use as a key
    let status_convert_public_key_to_string = convert_vector_u8_to_string(caller_public_key_vector_u8, &mut 
    caller_public_key_vector_string);

    // PENDING, want to use panic!(); or return 1;?
    if status_convert_public_key_to_string == 1 {

        // can choose to also panic!();
        return 1;
    }

    // this variable holds the information in string
    let caller_public_key = &caller_public_key_vector_string[0];
    /////////////////////// get caller public ends ///////////////////////
    

    
    /////////////////////// write into file to check if return deposit is satisfied starts ///////////////////////
    // MODIFY
    // if the way to store the key is changed, for example: instead of key001, it becomes alpha001, change here
    // has the normal file write and save the height so that can be checked here to see if condition is fulfilled or not
    // amount to transfer ||| the block height that will allow the withrawal of deposit ||| the withrawal status
    let string_to_save = format!("A;key{};{};{};{};{};{};0;", caller_public_key , sender_address , address , message , mosaic , current_block_height);

    // store the string inside an array of len 1
    let mut input_array = vec![string_to_save];

    // store the contents of the file here
    let mut data: Vec<String> = Vec::new();

    // read the file
    let status_file_read = file_read("StakingSupercontractClientInformation.txt", &mut data);

    // PENDING
    if status_file_read == 1{
        return 1;
    }

    // appends the contents of the original file with the desired contents
    let status_append_existing = file_append_with_existing("StakingSupercontractClientInformation.txt", &mut data, &mut input_array);

    // PENDING
    if status_append_existing == 1 {
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
#[no_mangle]
pub unsafe extern "C" fn withdraw() -> u32 {

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
    
    // stores the withdrawal amount 
    let call_param_withdrawal_amount = input_parts[0]; 
    /////////////////////// inputs from storage tool "parameters tab" ends ///////////////////////



    /////////////////////// get caller public starts ///////////////////////
    // get the caller public key
    let caller_public_key_vector_u8: [u8; 32] = get_caller_public_key();

    // this variable stores the data of the public key in an array with only one element
    let mut caller_public_key_vector_string: Vec<String> = Vec::new();

    // convert the public key to a string to use as a key
    let status_convert_public_key_to_string = convert_vector_u8_to_string(caller_public_key_vector_u8, &mut 
    caller_public_key_vector_string);

    // PENDING, want to use panic!(); or return 1;?
    if status_convert_public_key_to_string == 1 {

        // can choose to also panic!();
        return 1;
    }

    // this variable holds the information in string
    let caller_public_key = &caller_public_key_vector_string[0];
    /////////////////////// get caller public ends ///////////////////////
    

    
    /////////////////////// read and search for the supercontract caller starts ///////////////////////
    // this Vector will store the information about the target
    let mut result: Vec<String> = Vec::new();

    // this Vector is used to store data that exist within the file to avoid re-reading and lower the time complexity 
    let mut data: Vec<String> = Vec::new();

    // this Vector is used to store the contents of the file which is in the data variable above, but without target data
    let mut read_buffer : Vec<String> = Vec::new();

    // this Vector stores the number of times the data appeared
    let mut number_of_targets: Vec<u8> = Vec::new();

    // MODIFY
    // if the number changes
    // this variable stores the number of data per client in the text file
    // for example: A;2;harlow = 3 
    let number_of_data_per_client = 8;

    // read the contents of the file and convert them into a Vector of String
    let read_status = file_read("StakingSupercontractClientInformation.txt", &mut data);

    // if the file dosent exist, immediately fail
    // PENDING, want to use panic!(); or return 1;?
    if read_status == 1 {

        // can choose to also panic!();
        return 1;
    }

    // MODIFY
    // if the way the key is stored is changed, modify here
    // to format how key will look like
    let target = format!("key{}",caller_public_key);

    // to search for the data 
    let linear_search_status = linear_search(&mut data, &target , number_of_data_per_client,&mut read_buffer, &mut result, &mut number_of_targets);

    // if the key is not valid, immediately fail
    // PENDING, want to use panic!(); or return 1;?
    if linear_search_status == 1 {

        // can choose to also panic!();
        return 1;
    }
    /////////////////////// read and search for the supercontract caller ends ///////////////////////


    
    /////////////////////// find the intended withdrawal starts ///////////////////////
    // extract the number of deposits made
    let number_of_deposits = number_of_targets[0];

    let mut is_withdrawal = false;



    let mut read_placeholder = "".to_string();

    let mut read_caller_public_key = "".to_string();

    // get the caller's public key
    let mut read_sender_address =  "".to_string();

    // get the sender's address 
    let mut read_sender_address =  "".to_string();

    // get the receiver's address 
    let mut read_receiver_address =  "".to_string();

    // get the message 
    let mut read_message =  "".to_string();

    // get the mosaic from the text file
    let mut mosaic =  "".to_string();

    // take the height
    let mut data_block_height =   "".to_string();

    // get the withrawal status 
    let mut withdrawal_status =  "".to_string();

    // get current block height
    let current_block_height: u64 = get_block_height();

    // convert to i64
    let current_block_height_i64: i64 = current_block_height as i64;

    // loop through 
    for x in 0..number_of_deposits{

        // MODIFY
        // its currently -2 because thats how it includes the ";" 
        // this is the variable that stores the index of the status
        // eg: 
        // iteration 0, index of status = 11 
        let retrieved_status_index = (x * number_of_data_per_client * 2 ) + ( number_of_data_per_client  * 2 ) - ( 1 * 2 );
        let retrieved_status = &result[ retrieved_status_index as usize ];

        let retrieved_height_index = (x * number_of_data_per_client * 2 ) + ( number_of_data_per_client  * 2 ) - ( 2 * 2 );
        let retrieved_height = &result[ retrieved_height_index as usize ];

        let retrieved_amount_index = (x * number_of_data_per_client * 2 ) + ( number_of_data_per_client  * 2 ) - ( 3 * 2 );
        let retrieved_amount = &result[ retrieved_amount_index as usize ];

        // convert the height to int 
        let retrieved_block_height_in_i64: i64 = match retrieved_height.trim().parse() {
            Ok(integer) => integer,
            Err(_) => return 99,
        };

        // PENDING CHANGE HERE START
        // add code here to check the retrieved height and current height 
        // PENDING CHANGE HERE ENDS

        // PENDING
        // MAYBE here will have issues due to conversion types
        if  call_param_withdrawal_amount == retrieved_amount.as_str() && retrieved_status.as_str() == "0" && current_block_height_i64 >= retrieved_block_height_in_i64
        {

            
            // allow withdrawal
            is_withdrawal = true;

            // MODIFY 
            // if the number of data here changed, please add here
            // UPDATE, can make here more robust
            // all the indexing must be 0,2,4..2n cause of the ";"
            // get placeholder / extra character
            let read_placeholder_index = (x * number_of_data_per_client * 2 ) + ( 0 * 2 );
            read_placeholder = result[read_placeholder_index as usize ].clone();

            // get the caller's public key
            let read_caller_public_key_index = (x * number_of_data_per_client * 2 ) + ( 1 * 2 );
            read_caller_public_key = result[read_caller_public_key_index as usize ].clone();

            // get the sender's address 
            let read_sender_address_index = (x * number_of_data_per_client * 2 ) + ( 2 * 2 );
            read_sender_address = result[read_sender_address_index as usize ].clone();

            // get the receiver's address 
            let read_receiver_address_index = (x * number_of_data_per_client * 2 ) + ( 3 * 2 );
            read_receiver_address = result[read_receiver_address_index as usize ].clone();

            // get the message 
            let read_message_index = (x * number_of_data_per_client * 2 ) + ( 4 * 2 );
            read_message = result[read_message_index as usize ].clone();

            // get the mosaic from the text file
            let mosaic_index = (x * number_of_data_per_client * 2 ) + ( 5 * 2 );
            mosaic = result[mosaic_index as usize ].clone();

            // take the height
            let data_block_height_index = (x * number_of_data_per_client * 2 ) + ( 6 * 2 );
            data_block_height = result[data_block_height_index as usize ].clone();

            // get the withrawal status 
            let withdrawal_status_index = (x * number_of_data_per_client * 2 ) + ( 7 * 2 );
            withdrawal_status = result[withdrawal_status_index as usize ].clone();
        }
    }

    /////////////////////// find the intended withdrawal ends ///////////////////////

    
    /* 
    /////////////////////// supercontract caller data extraction starts ///////////////////////
    // convert the height allowed to withdraw to i64
    let data_block_height_i64: i64 = match data_block_height.trim().parse() {
        Ok(integer) => integer,
        Err(_) => return 99,
    };
    /////////////////////// supercontract caller data extraction ends ///////////////////////
    
    */

    
    /////////////////////// transaction starts ///////////////////////
    
    /* 
    // PENDING CHANGE HERE START
    // get current block height
    let current_block_height: u64 = get_block_height();
    
    // convert to i64
    let current_block_height_i64: i64 = current_block_height as i64;
    */

    // check if the condition has been met
    if is_withdrawal == true { 

    // PENDING CHANGE HERE ENDS
        
        // method 2, use the input from users
        let send_address = read_sender_address.as_str();
        

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

       
        let status_file_append = file_append_with_existing("StakingSupercontractClientInformation.txt", &mut read_buffer, &mut result);

        // PENDING
        if status_file_append == 1 {
            return 1;
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

    // this variable stores the data after being split 
    let mut data_parts = Vec::new();

    // this is the method that splits the string using a delimiter while including the delimiter
    let status_split = string_spliter_terminator(file_content_in_string.to_string(), ";",&mut data_parts);

    // PENDING
    if status_split == 1{
        return 1;
    }

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
/// - number_of_targets: the number of times the target appeared
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
/// let mut number_of_targets : Vec<u8> = Vec::new();
/// file_read("try.txt", &mut data);
/// linear_search(&mut data, "key1", 3, &mut read_buffer, &mut result, &mut number_of_targets);
/// 
/// // OR
/// 
/// let data = vec!["A" , ";" , "key1388714119416524012310222157236205981062271959714114215349145660116225218158157769" , ";" , "SAONE2UIW6DIH6BXKAW4OTF44XMJSQ23OUES6YBB" , ";" , "SBA3E4YPFXSDB4I6TXSRVDG6TZAOLP244AQSE3QA" , ";" , "Bye Bye money" , ";" , "1000" , ";" , "127" , ";" , "0" , ";" , "A", ";" , "key4852455010992561212281862281822354423979131195229196811161481912253185841992247" , ";" , "SD2L2LRSBZUMYV2T34C4UXOIAAWX4TWQSQGBPMQO" ,";", "SBA3E4YPFXSDB4I6TXSRVDG6TZAOLP244AQSE3QA" , ";" , "Here you go" , ";" , "1000" , ";" , "118" , ";" , "1" , ";", "A" , ";" , "key1388714119416524012310222157236205981062271959714114215349145660116225218158157769" , ";" , "SAONE2UIW6DIH6BXKAW4OTF44XMJSQ23OUES6YBB" , ";" , "SBA3E4YPFXSDB4I6TXSRVDG6TZAOLP244AQSE3QA" , ";" , "SAYONARA" , ";" , "1000" , ";" , "127" , ";" , "0" , ";"];
/// let mut data_array : Vec<String> = Vec::new();
/// 
/// for x in data{
///    data_array.push(x.to_string());
/// }
/// 
/// let target = "key1388714119416524012310222157236205981062271959714114215349145660116225218158157769";
/// let number_of_data_per_client = 8;
/// let mut read_buffer_modified: Vec<String> = Vec::new();
/// let mut wanted_data: Vec<String> = Vec::new();
/// let mut number_of_targets: Vec<u8> = Vec::new();
/// 
/// linear_search(&mut data_array, target, number_of_data_per_client, &mut read_buffer_modified, &mut wanted_data , &mut number_of_targets );
/// 
/// ```
/// 
/// # Notes
fn linear_search( data_array:&mut Vec<String>, target:&str, number_of_data_per_client:u8, read_buffer_modified:&mut Vec<String>, wanted_data:&mut Vec<String>, number_of_targets: &mut Vec<u8>) -> u8 {
    // needs to return the result / status of this function
    // set 1 by default, will change to 0 if target is found
    let mut status = 1;

    // this is the variable that will store the number of times that the target appeared
    let mut number_of_targets_in_u8:u8 = 0;

    // MODIFY
    // this indicates the number jumps to perform 
    let number_of_jumps: u8 = 2;

    // MODIFY
    // if there is like elements infront of the key besides the extra character, need to remove this
    // while loop counter 
    // set the initial counter value to the first key which is at index what ever the number of jumps is.
    // For example if the number of jumps = 2
    // example string: A;key1
    // key1 is at index 2
    let mut counter = number_of_jumps as usize;

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

            // loop though the entire data for 1 client
            while number_of_repetitions_now < number_of_data_per_client {

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

            // increment the number of times the data appeared
            number_of_targets_in_u8 += 1;
            
        }

        // if its other keys 
        else if data_array[counter].starts_with(other_keys)
        {
            // this is the variable that will store the data of the other clients
            let mut read_buffer_temp :Vec<String> = Vec::new();

            // this is the counter used to extract the information
            // -1 because of the extra character
            let mut word_index = counter as u8 - number_of_jumps;

            // keeps track of how many data per customer, we have added to the list
            let mut number_of_repetitions_now = 0;

            // -1 because if lets say we have 7 words, we start from -1, so lets say we found the target at index 1,
            // it will be like this 0,1,2,3,4,5,6 ( inclusive )
            while number_of_repetitions_now < number_of_data_per_client {

                // this is to push the data into the temp array
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
        
        // MODIFY
        // if there is bug, here could be the issue
        // can increase the counter to reduce time complexity
        // increment the counter
        counter += number_of_data_per_client as usize;
    }

    // store the number of targets inside the array
    number_of_targets.push(number_of_targets_in_u8);

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

/// # Objective
/// This function is designed to iterate through a string and seperate them into different strings based on "a" delimiter ( 1 delimiter )
/// The reason for this function's existence is due to the sdk not being able to provide this function
/// 
/// # Parameters
/// - input: the String that wants to be split
/// - delimiter: the delimiter in &str type
/// - output: the vector of String that will contain the list of Strings split into their respective
/// 
/// # Examples
/// ```
/// let mut result: Vec<String> = Vec::new();
/// let mut read_buffer_modified = Vec::new();
/// let mut wanted_data = Vec::new();
/// let mut target = "keySD2L2LRSBZUMYV2T34C4UXOIAAWX4TWQSQGBPMQO";
/// let number_of_data_per_client = 7;
/// string_spliter_terminator("A;keySD2L2LRSBZUMYV2T34C4UXOIAAWX4TWQSQGBPMQO;SBA3E4YPFXSDB4I6TXSRVDG6TZAOLP244AQSE3QA;Here you go;1000;329;0;A;keyAD2L2LRSBZUMYV2T34C4UXOIAAWX4TWQSQGBPMQO;SBA3E4YPFXSDB4I6TXSRVDG6TZAOLP244AQSE3QA;Here you go;2000;329;0;".to_string(), ";", &mut result);
/// ```
/// 
/// # Notes
fn string_spliter_terminator(input:String , delimiter:&str, output:&mut Vec<String> ) -> u8 {

    let mut status = 0;

    // this variables stores the characters that make up a string
    let mut temp: Vec<char> = Vec::new();

    // to make the iter variable peekable
    // peekable means to see the next element
    let mut char_iter = input.chars().peekable();

    // iterate over all the characters in the string
    while let Some( character ) = char_iter.next()
    {

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
            // else if the delimiter is the only content in the array
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
            // if there is no characters left
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

/// # Objective
/// This function creates a new text file, and writes the contents of the original file ( input ) and the content that wants to be appended in ( input )
/// 
/// # Parameters
/// - file_path: the file name with the .txt
/// - original_data: the original contents of the file
/// - content: the data that wants to be appended
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
/// let string_to_save = format!("A;key{};{};{};{};{};{};0;", caller_public_key , sender_address , address , message , mosaic , current_block_height);
/// // store the string inside an array of len 1
/// let mut input_array = vec![string_to_save];
/// // store the contents of the file here
/// let mut data: Vec<String> = Vec::new();
/// // read the file
/// let status_file_read = file_read("StakingSupercontractClientInformation.txt", &mut data);
/// // PENDING
/// if status_file_read == 1{
///     return 1;
/// }
/// // appends the contents of the original file with the desired contents
/// let status_append_existing = file_append_with_existing("StakingSupercontractClientInformation.txt", &mut data, &mut input_array);
/// // PENDING
/// if status_append_existing == 1 {
///     return 1;
/// }
/// ```
/// 
/// # Notes
/// - If the write function in the sdk can accomodate append, this function is not needed
fn file_append_with_existing(file_path:&str , original_data:&mut Vec<String>, content:&mut Vec<String> ) -> u32 {
    // needs to return the result / status of this function
    let mut status: u32 = 0;

	{
        // create a file
		let mut file = FileWriter::new(file_path).unwrap();

        // rewrite everything that existed already
        // file.write(file_content_in_string.as_bytes()).unwrap();

        //////////////////// work here //////////////////////////////////////

        // write the original content
        let mut temp_original = Vec::new();
        let mut temp_content = Vec::new();

        let status_convert_original = convert_string_vector_to_u8_vector(original_data, &mut temp_original);

        // PENDING
        if status_convert_original == 1 {
            return 1;
        }
        
        file.write(&temp_original).unwrap();


        let status_convert_content = convert_string_vector_to_u8_vector(content, &mut temp_content);

        // PENDING
        if status_convert_content == 1 {
            return 1;
        }

        file.write(&temp_content).unwrap();
        
        // flush out any remiaining data
		file.flush().unwrap();
	}

    return status;
}

/// # Objective
/// This function is designed to take in a Vector of String and convert it into a vector of u8
/// 
/// # Parameters
/// - in_string_vector: the Vector of String that wants to be converted
/// - out_u8_vector: the Vector of u8 that will store the converted version 
/// 
/// # Examples
/// ```
/// let mut converted_others_u8 : Vec<u8> = Vec::new();
/// let status_convert_vec_string_to_vec_u8_target = convert_string_vector_to_u8_vector(&mut result, &mut converted_target_u8);
/// //PENDING
/// if status_convert_vec_string_to_vec_u8_target == 1 {
///    return 1;
/// }
/// ```
/// 
/// # Notes
fn convert_string_vector_to_u8_vector(in_string_vector:&mut Vec<String> , out_u8_vector:&mut Vec<u8> ) -> u8{
    let mut status = 0;

    let mut temp: Vec<u8> = in_string_vector.clone().into_iter().flat_map(|s| s.into_bytes()).collect();

    out_u8_vector.extend_from_slice(&temp);

    return status;
}

/// # Objective
/// This function is designed to take in a Vector of u8 and convert it into a vector of String with only 1 element
/// 
/// # Parameters
/// - input_vector_u8: the Vector of u8 that wants to be converted
/// - output_string: the Vector of String that will store the converted version 
/// 
/// # Examples
/// ```
/// let data = [4, 85, 245, 50, 109, 92, 56, 1, 21, 228, 186, 228, 182, 235, 44, 239, 79, 131, 195, 229, 196, 81, 116, 148, 191, 225, 31, 85, 84, 19, 92, 247];
/// let mut result3 = Vec::new();
/// let status = convert_vector_u8_to_string(data, &mut result3);
/// ```
/// 
/// # Notes
/// - the output parameter is a vector for consistency 
fn convert_vector_u8_to_string( input_vector_u8: [u8;32] , output_string:&mut Vec<String> ) -> u8 {
    let mut status = 0;

    // this array will store the String 
    let mut converted_to_string: Vec<String> = Vec::new();

    for x in input_vector_u8.iter(){
        converted_to_string.push(x.to_string());
    }

    // this variable will hold the String
    let mut temp = converted_to_string.concat();

    // push the content back to the array
    output_string.push(temp);

    return status;
}