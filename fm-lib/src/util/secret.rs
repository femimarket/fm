use std::cell::OnceCell;
use hmac::digest::MacError;
use hmac::{Hmac, Mac};
use sha2::Sha256;

const SECRET: OnceCell<String> = OnceCell::new();

fn secret() -> String {
    SECRET.get_or_init(|| {
        std::env::var("SECRET").unwrap()
    }).to_string()
}

pub fn verify_signed_message(
    data: &[u8],
)->Result<(),MacError>{
    let mut mac = Hmac::<Sha256>::new_from_slice(secret().as_ref())
        .expect("HMAC can take key of any size");
    Ok(mac.verify_slice(data)?)
}

pub fn sign_message(
    data: &[u8],
)->Vec<u8>{
    let mut mac = Hmac::<Sha256>::new_from_slice(secret().as_ref())
        .expect("HMAC can take key of any size");
    mac.update(&data);
    let result = mac.finalize();
    let code_bytes = result.into_bytes();
    code_bytes.to_vec()
}