use std::{fs, io::Error, fmt::{Display, Formatter, self}};
use sha1::{digest::generic_array::GenericArray, Digest, Sha1};
use typenum::U20;


pub fn gen_hash(data: &[u8]) -> Result<BlobHash, Error> {
    let mut hasher = Sha1::new();
    hasher.update(data);
    let result = hasher.finalize();
    let result = BlobHash::new(result);
    Ok(result)
}

struct BlobHash {
    hash: GenericArray<u8, U20>,
}  

impl Display for BlobHash {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.as_string())
    }
}

impl BlobHash {
    fn new(hash: GenericArray<u8, U20>) -> Self {
        BlobHash { hash }
    }

    fn as_string(&self) -> String {
       self.hash.iter().map(|b| format!("{:02x}", b)).collect::<String>()
    }

    fn split_hash(&self) -> (String, String) {
        let hash_str = self.as_string();
        let (first_two, last_38) = hash_str.split_at(2);
        (first_two.to_string(), last_38.to_string())
    }
}

pub struct Blob {
    data: Vec<u8>,
    hash: BlobHash,
}

impl Blob {
    pub fn try_new(file_path: &str) -> Result<Self, Error> {
        let data = fs::read(file_path)?;
        let hash = gen_hash(&data)?;

        Ok(Blob { data, hash })
    }

    pub fn save(&mut self) -> () {
        let (first_two, last_38) = self.hash.split_hash();
        let dir = format!(".rustit/objects/{}", first_two);
        let file = format!("{}/{}", dir, last_38);
        fs::create_dir_all(dir).expect("Error creating directory");
        fs::write(file, &self.data).expect("Error writing file");
    }
}