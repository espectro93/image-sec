use aes_gcm::{Aes256Gcm, Key, Nonce, AeadInPlace};
use aes_gcm::aead::{Aead, NewAead, Buffer};

pub fn encrypt(to_encrypt: &[u8]) -> Vec<u8> {
    let key = Key::from_slice(b"an example very very secret key.");
    let cipher = Aes256Gcm::new(key);
    let nonce = Nonce::from_slice(b"unique nonce");

    return cipher.encrypt(nonce, to_encrypt.as_ref()).expect("Err");
}

pub fn decrypt(mut to_decrypt: &[u8]) -> Vec<u8>{
    let key = Key::from_slice(b"an example very very secret key.");
    let cipher = Aes256Gcm::new(key);
    let nonce = Nonce::from_slice(b"unique nonce");

    return cipher.decrypt(nonce, to_decrypt.as_ref()).expect("decryption failure");
}