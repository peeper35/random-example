use sha2::Sha512;
use hmac::{Hmac, Mac, NewMac};
type HmacSHA512 = Hmac<Sha512>;

//utilities
pub fn to_hex_string(bytes: &Vec<u8>) -> String {
    let strs: Vec<String> = bytes.iter()
        .map(|b| format!("{:02x}", b))
        .collect();
    strs.join("")
}

pub fn hash_to_number(slice : &str) -> i64{
    let z = i64::from_str_radix(slice, 16);
    z.unwrap()
}

//main fns
pub fn convert_to_hmacsha512(server_seed : String, input: String, nonce: i64) -> String{
    let mut mac = HmacSHA512::new_from_slice(server_seed.as_bytes()).expect("some error occured");
    mac.update(&format!("{} - {}", input.trim().to_string(), nonce).into_bytes());
    let result = mac.finalize();
    let byte_arr = result.into_bytes();
    to_hex_string(&byte_arr.to_vec())
}

pub fn get_non_999_no(hmac: &String) -> Option<i64> {
    let mut start: usize = 0;
    let mut end: usize = 5;
    let mut slice1 = &hmac[start..end];

    loop {
        let no = hash_to_number(slice1);

        if no < 999999{
            return Some(no);
        } else {
           println!("Need for increment");
           start += 5;
           end += 5;
           if end > 127 {
                println!("Hash length exceeds {}", end);
                return None;
           }
           slice1 = &hmac[start..end];
        }
    }
}

pub fn index(range: usize, rndm: f64) -> usize{
    // range (max - min) * (n, 1) + min;
    let idx = (range - 1) as f64 * rndm;
    idx as usize
}

pub fn get_random_number(server_seed: &String, client_seed: &String, nonce: i64) -> usize {
    let hmachash: String = convert_to_hmacsha512(server_seed.to_string(), client_seed.to_string(), nonce);
    let rnd = get_non_999_no(&hmachash).unwrap()%(10000)/100;

    let mut frnd = rnd as f64;
    if frnd < 10.0{
        frnd = frnd + 10.0;
    }
    frnd = frnd/100.0;
    frnd = (frnd * 10.0).round() / 10.0;
    let idx = index(10, frnd);

    return idx;
}