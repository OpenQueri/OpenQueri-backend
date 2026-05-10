use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, PasswordVerifier, SaltString},
    Argon2
};

pub struct Argon;

impl Argon {

    pub async fn hash_pwd(password: &str) -> String {
        let salt = SaltString::generate(&mut OsRng); 
        let argon2 = Argon2::default();
        
        argon2.hash_password(password.as_bytes(), &salt)
            .expect("Failed to hash")
            .to_string()
    }

    pub async fn verify_pwd(hash: &str, attempt: &str) -> anyhow::Result<bool> {
        use argon2::password_hash::PasswordHash;
        
        let parsed_hash = PasswordHash::new(hash).map_err(|e| anyhow::anyhow!("Invalid password hash: {}", e))?;
        
        Ok(Argon2::default()
            .verify_password(attempt.as_bytes(), &parsed_hash)
            .is_ok())
    }
    
}