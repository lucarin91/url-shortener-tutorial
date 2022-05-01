use serde::{Deserialize, Serialize};

// Create a set of structs for serialize/deserialize the following json:
// {
//     "urls": [
//         {
//             "short": "foo",
//             "long": "bar"
//         }
//     ]
// }
//

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn storage_json() {
        let storage = Storage {
            urls: vec![
                URLPair {
                    short: "key1".to_string(),
                    long: "val1".to_string(),
                },
                URLPair {
                    short: "key2".to_string(),
                    long: "val2".to_string(),
                },
                URLPair {
                    short: "key3".to_string(),
                    long: "val3".to_string(),
                },
            ],
        };

        let encoded: String = serde_json::to_string(&storage).unwrap();
        let decoded: Storage = serde_json::from_str(&encoded).unwrap();
        assert_eq!(storage, decoded);
    }
}
