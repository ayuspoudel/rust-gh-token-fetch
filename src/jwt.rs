use chrono::{Duration, Utc};
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use serde::Serialize;

#[derive(Debug, Serialize)]
struct Claims {
    iat: usize,
    exp: usize,
    iss: String,
}

pub fn generate_jwt(app_id: u64, private_key: &str) -> Result<String, Box<dyn std::error::Error>> {
    let encoding_key = EncodingKey::from_rsa_pem(private_key.as_bytes())?;

    let now = Utc::now();
    let claims = Claims {
        iat: now.timestamp() as usize,
        exp: (now + Duration::minutes(10)).timestamp() as usize,
        iss: app_id.to_string(),
    };

    let token = encode(&Header::new(Algorithm::RS256), &claims, &encoding_key)?;
    Ok(token)
}
