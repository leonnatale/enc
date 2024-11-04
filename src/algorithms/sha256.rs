use sha2::{Digest, Sha256};

pub fn encode(input: String) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input);
    let result = hasher.finalize();
    hex::encode(result)
}