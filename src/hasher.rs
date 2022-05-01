use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

// Implement a `str_hash` method for converting a string into an hash string in base64.
// E.g. 'foobar' => 'uf6oYwoEKVQ='
//
// Hint: this is the code for creating an string hash:
//
// let mut s = DefaultHasher::new();
// "foobar".hash(&mut s);
// let n = s.finish();
// let buf = n.to_be_bytes();
// base64::encode_config(&buf, base64::URL_SAFE)
//

#[cfg(test)]
mod tests {
    use super::*;
    use url::Url;

    #[test]
    fn string_hasher() {
        let s = String::from("foobar");
        let h = s.str_hash();
        assert_eq!(h, "uf6oYwoEKVQ=");
    }
}
