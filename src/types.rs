pub type Amount = u64;
pub type AccountId = String;
pub type Signature = String;
pub type Hash = String;

pub fn hash_bytes(data: &[u8]) -> Hash {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(data);
    hex::encode(hasher.finalize())
}
