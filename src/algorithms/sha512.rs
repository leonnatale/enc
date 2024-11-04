use sha2::{Digest, Sha512};

pub fn encode(input: String) -> String {
    let mut hasher = Sha512::new();
    hasher.update(input);
    let result = hasher.finalize();
    hex::encode(result)
}