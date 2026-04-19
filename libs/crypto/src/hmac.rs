use anyhow::Result;
use hmac::{Hmac, Mac};
use sha2::Sha256;

pub trait FieldHmac: Send + Sync {
    fn generate(&self, plaintext: &str) -> Result<String>;
}

pub struct HmacSha256 {
    key: Vec<u8>,
}

impl HmacSha256 {
    pub fn new(key: &[u8]) -> Result<Self> {
        if key.is_empty() {
            return Err(anyhow::anyhow!("HMAC key must not be empty"));
        }
        Ok(Self { key: key.to_vec() })
    }
}

impl FieldHmac for HmacSha256 {
    fn generate(&self, plaintext: &str) -> Result<String> {
        let mut mac = Hmac::<Sha256>::new_from_slice(&self.key)
            .map_err(|e| anyhow::anyhow!("hmac init failed: {}", e))?;

        mac.update(plaintext.to_lowercase().as_bytes());

        Ok(hex::encode(mac.finalize().into_bytes()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_key() -> Vec<u8> {
        vec![1u8; 32]
    }

    #[test]
    fn test_generate_is_deterministic() {
        let h = HmacSha256::new(&test_key()).unwrap();

        let a = h.generate("John").unwrap();
        let b = h.generate("john").unwrap(); // lowercase เดียวกัน

        assert_eq!(a, b);
    }

    #[test]
    fn test_different_input_produces_different_hmac() {
        let h = HmacSha256::new(&test_key()).unwrap();

        let a = h.generate("john").unwrap();
        let b = h.generate("jane").unwrap();

        assert_ne!(a, b);
    }

    #[test]
    fn test_different_key_produces_different_hmac() {
        let h1 = HmacSha256::new(&vec![1u8; 32]).unwrap();
        let h2 = HmacSha256::new(&vec![2u8; 32]).unwrap();

        let a = h1.generate("john").unwrap();
        let b = h2.generate("john").unwrap();

        assert_ne!(a, b);
    }

    #[test]
    fn test_empty_key_fails() {
        let result = HmacSha256::new(&[]);
        assert!(result.is_err());
    }
}
