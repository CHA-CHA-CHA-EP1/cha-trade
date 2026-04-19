use anyhow::{Result, anyhow};
use argon2::{
    Argon2,
    password_hash::{
        PasswordHash, PasswordHasher as Argon2PasswordHasher, PasswordVerifier, SaltString,
        rand_core::OsRng,
    },
};

pub trait PasswordHasher: Send + Sync {
    fn hash(&self, password: &str) -> Result<String>;
    fn verify(&self, password: &str, hash: &str) -> Result<bool>;
}

pub struct Argon2Hasher;

impl Argon2Hasher {
    pub fn new() -> Self {
        Self
    }
}

impl PasswordHasher for Argon2Hasher {
    fn hash(&self, password: &str) -> Result<String> {
        let salt = SaltString::generate(&mut OsRng);
        let hash = Argon2::default()
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| anyhow!("hash failed: {}", e))?;

        Ok(hash.to_string())
    }

    fn verify(&self, password: &str, hash: &str) -> Result<bool> {
        let parsed = PasswordHash::new(hash)
            .map_err(|e| anyhow!("invalid hash: {}", e))?;

        Ok(Argon2::default()
            .verify_password(password.as_bytes(), &parsed)
            .is_ok())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_and_verify() {
        let hasher = Argon2Hasher::new();
        let hash = hasher.hash("password123").unwrap();
        assert!(hasher.verify("password123", &hash).unwrap());
    }

    #[test]
    fn test_wrong_password_fails() {
        let hasher = Argon2Hasher::new();
        let hash = hasher.hash("password123").unwrap();
        assert!(!hasher.verify("wrongpassword", &hash).unwrap());
    }

    #[test]
    fn test_hash_is_different_each_time() {
        let hasher = Argon2Hasher::new();
        let h1 = hasher.hash("password123").unwrap();
        let h2 = hasher.hash("password123").unwrap();
        assert_ne!(h1, h2); // random salt ทุกครั้ง
    }
}
