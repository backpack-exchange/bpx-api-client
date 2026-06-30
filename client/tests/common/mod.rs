use base64ct::{Base64, Encoding};
use ed25519_dalek::SigningKey;
use rand::rngs::OsRng;

/// Returns a base64-encoded ed25519 seed for tests.
pub fn test_secret() -> String {
    let signing_key = SigningKey::generate(&mut OsRng);
    Base64::encode_string(&signing_key.to_bytes())
}
