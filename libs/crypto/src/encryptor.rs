use aes_gcm::{
    AeadCore, Aes256Gcm, Key, KeyInit,
    aead::{Aead, OsRng, generic_array::GenericArray},
};
use anyhow::{Result, anyhow};

pub trait FieldEncryptor: Send + Sync {
    fn encrypt(&self, plaintext: &str) -> Result<String>;
    fn decrypt(&self, ciphertext: &str) -> Result<String>;
}

pub struct AesGcmEncryptor {
    cipher: Aes256Gcm,
}

impl AesGcmEncryptor {
    pub fn new(key: &[u8]) -> Result<Self> {
        if key.len() != 32 {
            return Err(anyhow!(
                "AES-256 key must be exactly 32 bytes, got {}",
                key.len()
            ));
        }
        let key = Key::<Aes256Gcm>::from_slice(key);
        Ok(Self {
            cipher: Aes256Gcm::new(key),
        })
    }
}

impl FieldEncryptor for AesGcmEncryptor {
    fn encrypt(&self, plaintext: &str) -> Result<String> {
        let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

        let ciphertext = self
            .cipher
            .encrypt(&nonce, plaintext.to_lowercase().as_bytes())
            .map_err(|e| anyhow!("encrypt failed: {}", e))?;

        let mut result = nonce.to_vec();
        result.extend_from_slice(&ciphertext);

        Ok(hex::encode(result))
    }

    fn decrypt(&self, encoded: &str) -> Result<String> {
        let bytes = hex::decode(encoded)?;

        // nonce (12 bytes) + ciphertext + tag (16 bytes)
        if bytes.len() < 12 + 16 {
            return Err(anyhow!("invalid ciphertext: too short"));
        }

        let nonce = GenericArray::from_slice(&bytes[..12]);
        let ciphertext = &bytes[12..];

        let plaintext = self
            .cipher
            .decrypt(nonce, ciphertext)
            .map_err(|e| anyhow!("decrypt failed: {}", e))?;

        Ok(String::from_utf8(plaintext)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_key() -> Vec<u8> {
        vec![0u8; 32]
    }

    #[test]
    fn test_encrypt_decrypt_roundtrip() {
        let enc = AesGcmEncryptor::new(&test_key()).unwrap();
        let plaintext = "John";

        let encrypted = enc.encrypt(plaintext).unwrap();
        let decrypted = enc.decrypt(&encrypted).unwrap();

        assert_eq!(decrypted, "john"); // lowercase
    }

    #[test]
    fn test_encrypt_produces_different_ciphertext_each_time() {
        let enc = AesGcmEncryptor::new(&test_key()).unwrap();

        let a = enc.encrypt("john").unwrap();
        let b = enc.encrypt("john").unwrap();

        // random nonce ทำให้ต่างกันทุกครั้ง
        assert_ne!(a, b);
    }

    #[test]
    fn test_decrypt_wrong_key_fails() {
        let enc1 = AesGcmEncryptor::new(&vec![0u8; 32]).unwrap();
        let enc2 = AesGcmEncryptor::new(&vec![1u8; 32]).unwrap();

        let encrypted = enc1.encrypt("john").unwrap();
        let result = enc2.decrypt(&encrypted);

        assert!(result.is_err());
    }

    #[test]
    fn test_decrypt_tampered_ciphertext_fails() {
        let enc = AesGcmEncryptor::new(&test_key()).unwrap();
        let mut encrypted = enc.encrypt("john").unwrap();

        // แก้ตัวสุดท้าย
        let last = encrypted.pop().unwrap();
        encrypted.push(if last == 'a' { 'b' } else { 'a' });

        let result = enc.decrypt(&encrypted);
        assert!(result.is_err());
    }

    #[test]
    fn test_decrypt_too_short_fails() {
        let enc = AesGcmEncryptor::new(&test_key()).unwrap();
        let result = enc.decrypt("deadbeef");
        assert!(result.is_err());
    }

    #[test]
    fn test_new_wrong_key_length_fails() {
        let result = AesGcmEncryptor::new(&vec![0u8; 16]);
        assert!(result.is_err());
    }
}
