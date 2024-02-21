/*
File name: Staking supercontract
Version: 1.0.1 
Patch notes: fixed get_clean_call_params()

PENDING: 
1) dont know if should have additional condition to determine if the person has withrawn or not written into the file
    1.1) i think need

main account 
SD2L2LRSBZUMYV2T34C4UXOIAAWX4TWQSQGBPMQO;Here you go;1000

second account address
SD635GAAXIS6EHCEBLYDHJIRIHMMYAJMMMZH3YVC;Here you go;1000

send to this empty wallet so its easier to see
SBA3E4YPFXSDB4I6TXSRVDG6TZAOLP244AQSE3QA;Here you go;1000
address of empty wallet; message( can be anything ); mosaic
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
use std::io::{BufReader, BufWriter, Read, Write};

/// # Objective
/// This function is used to deploy the supercontract onto the blockchain
/// 
/// # Notes
/// This is the function used in the function name, along with the assignee, file name, function name, parameters
/// 
#[no_mangle]
pub unsafe extern "C" fn run() -> u32 {
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

    let address = input_parts[0]; //Address to locked place
    let message = input_parts[1]; //Message
    let mosaic = input_parts[2]; //Total mosaics to be send

    // this height is used to check condition for deposit return
    // why +80? cause its only an estimation, 1 block takes approx 15seconds to build, 
    // 1 block = 15s, required block heigh = wanted time taken/ needed before release of deposit
    // +80 is approx 5 mins, 15s * 80 , 
    // also lets say if we put 80, then best is to have the autorun run 80 times + 20
    let current_block_height = get_block_height() + 5;
    /////////////////////// inputs from storage tool "parameters tab" ends ///////////////////////
    


    /////////////////////// write into file to check if return deposit is satisfied starts ///////////////////////
    // has the normal file write and save the height so that can be checked here to see if condition is fulfilled or not
    let string_to_save = format!("{};{}", mosaic, current_block_height);

	{
		let mut file = FileWriter::new("DataGameDeveloper.txt").unwrap();
		file.write(&string_to_save.as_bytes()).unwrap();
		file.flush().unwrap();
	}
    /////////////////////// write into file to check if return deposit is satisfied starts ///////////////////////
    
    

    /////////////////////// address conversion to base 32 starts ///////////////////////
    // this is the array that will store the information for the decoded address
    let mut decoded_address: Vec<u8> = Vec::new();

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
pub unsafe extern "C" fn withraw_deposit() -> u32 {

    let mut file_content = Vec::new();
    {
        // maybe here will have error
        // let file = FileReader::new("DataGameDeveloper.txt").unwrap();
        // let mut reader = BufReader::new(file);
        // reader.read_to_end(&mut file_content).unwrap();
        let mut file = FileReader::new("DataGameDeveloper.txt").unwrap();
        file.read_to_end(&mut file_content).unwrap();
    }
    let file_content_in_string = String::from_utf8_lossy(&file_content);

    // expected data
    // 1000;40
    let data_parts: Vec<&str> = file_content_in_string.split(';').collect(); 

    // take the height
    let data_block_height = data_parts[1];

    // convert to i64
    let data_block_height_i64: i64 = match data_block_height.trim().parse() {
        Ok(integer) => integer,
        Err(_) => return 99,
    };
    
    // get current block height
    let current_block_height: u64 = get_block_height();

    // convert to i64
    let current_block_height_i64: i64 = current_block_height as i64;

    // check if the condition has been met
    // PENDING, cannot use >= cause if its true, it will keep sending money right?
    if current_block_height_i64 >= data_block_height_i64 { 
        //current_block_height_i64 != data_block_height_i64

        //Address to game developer / main account 
        let address = "SD2L2LRSBZUMYV2T34C4UXOIAAWX4TWQSQGBPMQO"; 
        let message = "I sent the mosaic back"; //Message

        // PENDING change this to the mosaic that is in the folder
        let mosaic = "1000"; //Mosaic



        /////////////////////// address conversion to base 32 starts ///////////////////////
        // this is the array that will store the information for the decoded address
        let mut decoded_address: Vec<u8> = Vec::new();

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
        let status_decode_message: u8 = decode_message_to_u8(message, &mut message_in_bytes, 0 );

        // PENDING
        if status_decode_message == 1{

        //panic!();
        return 1;
        }
        /////////////////////// message conversion to base 32 ends ///////////////////////
         
        

        /////////////////////// message conversion to base 32 starts ///////////////////////
        // this is the array that will store the message in bytes
        let mut message_in_bytes: Vec<u8> = Vec::new();
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
        
        return 0;
    } else {
        return 99;
    }
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