use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;

pub fn rand(height: u64, nanos: u64, nonce: u32) -> u64 {
    let mut hasher = DefaultHasher::new();

    hasher.write_u64(height);
    hasher.write_u64(nanos);
    hasher.write_u32(nonce);

    hasher.finish()
}
