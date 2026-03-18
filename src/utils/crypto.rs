use aes_gcm::{
    aead::{Aead, KeyInit, OsRng},
    Aes256Gcm, Nonce,
};
use anyhow::Result;
use base64::{engine::general_purpose, Engine as _};
use rand::RngCore;

pub struct Crypto;

impl Crypto {
    /// Generate a random encryption key
    pub fn generate_key() -> [u8; 32] {
        let mut key = [0u8; 32];
        OsRng.fill_bytes(&mut key);
        key
    }

    /// Encrypt data using AES-256-GCM
    pub fn encrypt(data: &[u8], key: &[u8; 32]) -> Result<Vec<u8>> {
        let cipher = Aes256Gcm::new(key.into());

        // Generate random nonce
        let mut nonce_bytes = [0u8; 12];
        OsRng.fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);

        // Encrypt
        let ciphertext = cipher
            .encrypt(nonce, data)
            .map_err(|e| anyhow::anyhow!("Encryption failed: {}", e))?;

        // Prepend nonce to ciphertext
        let mut result = nonce_bytes.to_vec();
        result.extend_from_slice(&ciphertext);

        Ok(result)
    }

    /// Decrypt data using AES-256-GCM
    pub fn decrypt(encrypted_data: &[u8], key: &[u8; 32]) -> Result<Vec<u8>> {
        if encrypted_data.len() < 12 {
            return Err(anyhow::anyhow!("Invalid encrypted data"));
        }

        let cipher = Aes256Gcm::new(key.into());

        // Extract nonce and ciphertext
        let (nonce_bytes, ciphertext) = encrypted_data.split_at(12);
        let nonce = Nonce::from_slice(nonce_bytes);

        // Decrypt
        let plaintext = cipher
            .decrypt(nonce, ciphertext)
            .map_err(|e| anyhow::anyhow!("Decryption failed: {}", e))?;

        Ok(plaintext)
    }

    /// Encrypt and encode to base64
    pub fn encrypt_to_base64(data: &str, key: &[u8; 32]) -> Result<String> {
        let encrypted = Self::encrypt(data.as_bytes(), key)?;
        Ok(general_purpose::STANDARD.encode(encrypted))
    }

    /// Decode from base64 and decrypt
    pub fn decrypt_from_base64(encoded: &str, key: &[u8; 32]) -> Result<String> {
        let encrypted = general_purpose::STANDARD.decode(encoded)?;
        let decrypted = Self::decrypt(&encrypted, key)?;
        Ok(String::from_utf8(decrypted)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt() {
        let key = Crypto::generate_key();
        let data = b"Hello, World!";

        let encrypted = Crypto::encrypt(data, &key).unwrap();
        let decrypted = Crypto::decrypt(&encrypted, &key).unwrap();

        assert_eq!(data, decrypted.as_slice());
    }

    #[test]
    fn test_encrypt_decrypt_base64() {
        let key = Crypto::generate_key();
        let data = "Secret message";

        let encrypted = Crypto::encrypt_to_base64(data, &key).unwrap();
        let decrypted = Crypto::decrypt_from_base64(&encrypted, &key).unwrap();

        assert_eq!(data, decrypted);
    }
}
