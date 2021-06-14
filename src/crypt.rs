use aes_gcm::{Aes256Gcm, Key, Nonce, AeadInPlace}; // Or `Aes128Gcm`
use aes_gcm::aead::{Aead, NewAead};

//TODO: NOCHMAL DOCS ANSCHAUEN

pub fn encrypt(to_encrypt : &[u8]){
    let key = Key::from_slice(b"an example very very secret key.");
    let cipher = Aes256Gcm::new(key);
    let nonce = Nonce::from_slice(b"unique nonce"); // 96-bits; unique per message

    let mut buffer: Vec<u8> = Vec::with_capacity(128); // Buffer needs 16-bytes overhead for GCM tag
    buffer.extend_from_slice(to_encrypt);
    cipher.encrypt_in_place(nonce, b"", &mut buffer).expect("encryption failure!");

}

pub fn decrypt(mut to_decrypt : &[u8]){
    let key = Key::from_slice(b"an example very very secret key.");
    let cipher = Aes256Gcm::new(key);
    let nonce = Nonce::from_slice(b"unique nonce"); // 96-bits; unique per message
    // Decrypt `buffer` in-place, replacing its ciphertext context with the original plaintext
    cipher.decrypt_in_place(nonce, b"", &mut to_decrypt).expect("decryption failure!");
    assert_eq!(&to_decrypt, b"plaintext message");
}