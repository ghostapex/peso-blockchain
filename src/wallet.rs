use ed25519_dalek::{Signature as EdSignature, Signer, SigningKey, VerifyingKey};
use rand::rngs::OsRng;

use crate::types::{AccountId, Signature};

pub struct Wallet {
    secret: SigningKey,
    pub account_id: AccountId,
}

impl Wallet {
    pub fn new() -> Self {
        let secret = SigningKey::generate(&mut OsRng);
        let verifying = secret.verifying_key();
        let account_id = hex::encode(verifying.as_bytes());

        Self {
            secret,
            account_id,
        }
    }

    pub fn sign(&self, message: &[u8]) -> Signature {
        let sig = self.secret.sign(message);
        hex::encode(sig.to_bytes())
    }

    pub fn verify(account_id: &str, message: &[u8], signature_hex: &str) -> bool {
        let account_bytes = match hex::decode(account_id) {
            Ok(v) => v,
            Err(_) => return false,
        };

        if account_bytes.len() != 32 {
            return false;
        }

        let verifying_key = match VerifyingKey::from_bytes(
            account_bytes
                .as_slice()
                .try_into()
                .expect("32-byte account key required"),
        ) {
            Ok(vk) => vk,
            Err(_) => return false,
        };

        let signature_bytes = match hex::decode(signature_hex) {
            Ok(v) => v,
            Err(_) => return false,
        };

        let signature = match EdSignature::try_from(signature_bytes.as_slice()) {
            Ok(sig) => sig,
            Err(_) => return false,
        };

        verifying_key.verify(message, &signature).is_ok()
    }
}
