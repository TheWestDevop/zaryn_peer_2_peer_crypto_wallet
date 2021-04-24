use blake2::{Blake2s, Digest};
use rustc_serialize::hex::ToHex;

pub async fn encode(key:&str)  -> String {
    let mut hasher = Blake2s::new();
    hasher.update(key.as_bytes());
    let result = hasher.finalize(); 
    let x = result.to_hex().to_string();
    return x;
}