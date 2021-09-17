use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

// use sha2::Sha512;
// use hmac::{Hmac, Mac, NewMac};
// type HmacSHA512 = Hmac<Sha512>;

mod helpers;

/// Define the type of state stored in accounts
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct GreetingAccount {
    /// number of greetings
    pub nonce: i64,
    pub random_number: usize,
    pub server_seed: String,
    pub client_seed: String,
}

// Declare and export the program's entrypoint
entrypoint!(process_instruction);

// //utilities
// pub fn to_hex_string(bytes: &Vec<u8>) -> String {
//     let strs: Vec<String> = bytes.iter()
//         .map(|b| format!("{:02x}", b))
//         .collect();
//     strs.join("")
// }

// pub fn hash_to_number(slice : &str) -> i64{
//     let z = i64::from_str_radix(slice, 16);
//     z.unwrap()
// }

// //main fns
// pub fn convert_to_hmacsha512(server_seed : String, input: String, nonce: i64) -> String{
//     let mut mac = HmacSHA512::new_from_slice(server_seed.as_bytes()).expect("some error occured");
//     mac.update(&format!("{} - {}", input.trim().to_string(), nonce).into_bytes());
//     let result = mac.finalize();
//     let byte_arr = result.into_bytes();
//     to_hex_string(&byte_arr.to_vec())
// }

// pub fn get_non_999_no(hmac: &String) -> Option<i64> {
//     let mut start: usize = 0;
//     let mut end: usize = 5;
//     let mut slice1 = &hmac[start..end];

//     loop {
//         let no = hash_to_number(slice1);

//         if no < 999999{
//             return Some(no);
//         } else {
//            println!("Need for increment");
//            start += 5;
//            end += 5;
//            if end > 127 {
//                 println!("Hash length exceeds {}", end);
//                 return None;
//            }
//            slice1 = &hmac[start..end];
//         }
//     }
// }

// fn index(range: usize, rndm: f64) -> usize{
//     // range (max - min) * (n, 1) + min;
//     let idx = (range - 1) as f64 * rndm;
//     idx as usize
// }

// pub fn get_random_number(server_seed: &String, client_seed: &String, nonce: i64) -> usize {
//     let hmachash: String = convert_to_hmacsha512(server_seed.to_string(), client_seed.to_string(), nonce);
//     let rnd = get_non_999_no(&hmachash).unwrap()%(10000)/100;

//     let mut frnd = rnd as f64;
//     if frnd < 10.0{
//         frnd = frnd + 10.0;
//     }
//     frnd = frnd/100.0;
//     frnd = (frnd * 10.0).round() / 10.0;
//     let idx = index(10, frnd);

//     return 6;
// }

// Program entrypoint's implementation
pub fn process_instruction(
    program_id: &Pubkey, // Public key of the account the hello world program was loaded into
    accounts: &[AccountInfo], // The account to say hello to
    instruction_data: &[u8], // Ignored, all helloworld instructions are hellos
) -> ProgramResult {
    msg!("Hello World Rust program entrypoint");

    // Iterating accounts is safer then indexing
    let accounts_iter = &mut accounts.iter();

    // Get the account to say hello to
    let account = next_account_info(accounts_iter)?;

    // The account must be owned by the program in order to modify its data
    if account.owner != program_id {
        msg!("Greeted account does not have the correct program id");
        return Err(ProgramError::IncorrectProgramId);
    }
    
    // Increment and store the number of times the account has been greeted
    let mut greeting_account = GreetingAccount::try_from_slice(&account.data.borrow())?;

    greeting_account.nonce += 1;
    greeting_account.server_seed = "Some server seed".to_string();
    greeting_account.client_seed = "Some client seed".to_string();

    greeting_account.random_number = helpers::get_random_number(
        &greeting_account.server_seed, 
        &greeting_account.client_seed, 
        greeting_account.nonce
    );
    greeting_account.serialize(&mut &mut account.data.borrow_mut()[..])?;

    // msg!("Nonce: {} {} {} {}", greeting_account.server_seed, greeting_account.client_seed, greeting_account.nonce, greeting_account.random_number);

    Ok(())
}