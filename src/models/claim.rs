use serde::{Serialize, Deserialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct Claim {
    // pub aud: String,    // Optional. Audience /* Add this back when on a real service */
    pub exp: usize,     // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    pub iat: usize,     // Optional. Issued at (as UTC timestamp)
    // pub iss: String,    // Optional. Issuer /* Add this back when on a real service */
    pub nbf: usize,     // Optional. Not Before (as UTC timestamp)
    pub sub: String,    // Optional. Subject (whom token refers to)
}