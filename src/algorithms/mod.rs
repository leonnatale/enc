mod b64;
mod sha256;
mod sha512;

pub fn load_cryptography(name: String, input: String) -> Option<String> {
    match name.as_str() {
        "b64" => Some(b64::encode(input)),
        "sha256" => Some(sha256::encode(input)),
        "sha512" => Some(sha512::encode(input)),
        _ => None
    }
}