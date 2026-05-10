use pasetors::keys::SymmetricKey;
use pasetors::token::UntrustedToken;
use pasetors::claims::{Claims, ClaimsValidationRules};
use pasetors::{local, version4::V4};
use chrono::{Utc, Duration};

pub struct PasetoAuth;

impl PasetoAuth {
    const KEY: &'static [u8; 32] = b"0123456789abcdef0123456789abcdef";

    pub async fn create_token(user_id: &str, ttl_seconds: i64) -> anyhow::Result<String> {
        let sk = SymmetricKey::<V4>::from(Self::KEY)
            .map_err(|_| anyhow::anyhow!("Invalid key length"))?;

        let mut claims = Claims::new()?;
        let expiration = Utc::now() + Duration::seconds(ttl_seconds);
        claims.expiration(&expiration.to_rfc3339())?;
        
        claims.add_additional("user_id", serde_json::json!(user_id))?;

        let token = local::encrypt(&sk, &claims, None, None)
            .map_err(|e| anyhow::anyhow!("PASETO encrypt error: {}", e))?;

        Ok(token)
    }

    pub async fn verify_token(token_str: &str) -> anyhow::Result<String> {
        let sk = SymmetricKey::<V4>::from(Self::KEY)
            .map_err(|_| anyhow::anyhow!("Invalid key length"))?;

        let untrusted_token = UntrustedToken::<pasetors::token::Local, V4>::try_from(token_str)
            .map_err(|e| anyhow::anyhow!("Invalid token format: {}", e))?;

        let validation_rules = ClaimsValidationRules::new();

        let trusted_token = local::decrypt(&sk, &untrusted_token, &validation_rules, None, None)
            .map_err(|e| anyhow::anyhow!("Token invalid/expired: {}", e))?;
        
        let claims = trusted_token.payload_claims()
            .ok_or_else(|| anyhow::anyhow!("No claims found"))?;
        
        let user_id = claims.get_claim("user_id")
            .and_then(|v| v.as_str()) 
            .ok_or_else(|| anyhow::anyhow!("user_id not found or is not a string"))?;

        Ok(user_id.to_string())
    }
}
