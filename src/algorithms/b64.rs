use base64::{prelude::BASE64_STANDARD, Engine};

pub fn encode(input: String) -> String {
    BASE64_STANDARD.encode(input)
}