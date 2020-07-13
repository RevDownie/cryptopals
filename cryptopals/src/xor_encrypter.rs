/// Repeating key XOR to convert the text to "encrypted" text
///
pub fn xor_encrypt(text: &str, key: &str) -> Vec<u8> {
    let mut encrypted: Vec<u8> = Vec::with_capacity(text.len());
    let key_bytes = key.as_bytes();
    for (i, b) in text.bytes().enumerate() {
        encrypted.push(b ^ key_bytes[i % key_bytes.len()]);
    }
    encrypted
}
