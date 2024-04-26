use sha1::{Sha1, Digest};
pub struct RHash {
    hash: [u8; 20]
}  

impl RHash {
    pub fn new(content: &Vec<u8>) -> Self {
        let hash = hash_content(content);
        RHash { hash }
    }

    pub fn as_string(&self) -> String {
       self.hash.iter().map(|b| format!("{:02x}", b)).collect::<String>()
    }

    pub fn split_hash(&self) -> (String, String) {
        let hash_str = self.as_string();
        let (first_two, last_38) = hash_str.split_at(2);
        (first_two.to_string(), last_38.to_string())
    }
}


pub fn hash_content(data: &Vec<u8>) -> [u8; 20] {
    let mut hasher = Sha1::new();
    hasher.update(data);
    let result = hasher.finalize();
    let mut hash = [0u8; 20];
    hash.clone_from_slice(result.as_slice());
    hash
}