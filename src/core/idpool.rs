use std::sync::{Mutex, OnceLock};
use std::collections::HashMap;

static ID_POOL: OnceLock<Mutex<HashMap<u64, String>>> = OnceLock::new(); //Place where we store id's and id strings

/// FNV-1a [`str`]/[`String`] hashing, mainly used for id optimizations
/// Easy way to call this function is "[`id`]" macro
pub const fn hash_str(labels: &str) -> u64 {
    let mut hash: u64 = 0xcbf29ce484222325;
    let prime: u64 = 0x100000001b3;

    let bytes = labels.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        hash ^= bytes[i] as u64;
        hash = hash.wrapping_mul(prime);
        i += 1;
    }
    hash
}

/// Easy way to call [`hash_str`]
#[macro_export]
macro_rules! hsid {
    ($string:expr) => {
        $crate::core::idpool::hash_str($string)
    };
}

///Registers id into global id pool and returns hashed id
pub fn regid(name: String) -> u64 {
    let hash = hash_str(&name);
    
    let pool_mutex = ID_POOL.get_or_init(|| Mutex::new(HashMap::with_capacity(16)));
    
    if let Ok(mut pool) = pool_mutex.lock() {
        pool.insert(hash, name);
    }
    
    hash
}

///Gets id into global id pool
pub fn get_id(hash: u64) -> Option<String> {
    let pool_mutex = ID_POOL.get()?;
    let pool = pool_mutex.lock().ok()?;
    pool.get(&hash).cloned()
}