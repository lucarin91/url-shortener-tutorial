use std::collections::HashMap;
use std::sync::RwLock;

// Implement a thread safe KVS (key value store) with the following methods:
//
// pub fn store(&self, k: &str, v: &str) -> Result<(), KVSError>;
// pub fn load(&self, k: &str) -> Result<String, KVSError>;

// BONUS:
// pub fn dump(&self) -> Storage;

#[cfg(test)]
mod tests {
    use super::*;

    use crate::storage::{Storage, URLPair};

    #[test]
    fn kvs_load_store() {
        let kvs = KVS::default();

        let r = kvs.load("not-existing");
        assert_eq!(r, Err(KVSError::NotExists));

        let r = kvs.store("key1", "val1");
        assert_eq!(r, Ok(()), "should be ok");

        let r = kvs.store("key1", "already-exists");
        assert_eq!(r, Err(KVSError::AlreadyExists));
    }
}
