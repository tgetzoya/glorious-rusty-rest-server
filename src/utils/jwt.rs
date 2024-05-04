use std::time::SystemTime;

use jsonwebtoken::{Algorithm, decode, DecodingKey, encode, EncodingKey, Header, Validation};

use crate::models::claim::Claim;

pub fn generate_token(username: &str) -> String {
    let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();

    let claim: Claim = Claim {
        exp: (now + 300) as usize,
        iat: now as usize,
        nbf: now as usize,
        sub: username.trim().parse().unwrap(),
    };

    encode(&Header::new(Algorithm::HS512), &claim, &EncodingKey::from_secret("todoreplacemeplease".as_ref())).unwrap()
}

pub fn update_token(key: &str) -> String {
    let mut decoded = decode::<Claim>(key, &DecodingKey::from_secret("todoreplacemeplease".as_ref()), &Validation::new(Algorithm::HS512)).unwrap();

    let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();

    decoded.claims.exp = (now + 300) as usize;
    decoded.claims.iat = now as usize;
    decoded.claims.nbf = now as usize;

    encode(&decoded.header, &decoded.claims, &EncodingKey::from_secret("todoreplacemeplease".as_ref())).unwrap()
}

pub fn validate_token(token: &str) -> bool {
    let decoded = decode::<Claim>(token, &DecodingKey::from_secret("todoreplacemeplease".as_ref()), &Validation::new(Algorithm::HS512))
        .map_err(|e| {
            println!("ERROR: {}", e);
            return "Nope";
        });

    return decoded.is_ok()
}