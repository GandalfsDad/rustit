use crate::hash::rhash::RHash;
use std::io::Error;
use std::fs;
pub struct Blob {
    data: Vec<u8>,
    hash: RHash,
    name: String,
}

impl Blob {
    pub fn try_new(file_path: &str) -> Result<Self, Error> {
        let name = file_path.split("/").last().unwrap().to_string();
        let content = fs::read(file_path)?;
        let header = format!("blob {}\0", content.len());
        let data = [header.as_bytes(), &content].concat();
        let hash = RHash::new(&data);

        Ok(Blob { data, hash, name })
    }

    pub fn save(&mut self) -> () {
        let (first_two, last_38) = self.hash.split_hash();
        let dir = format!(".rustit/objects/{}", first_two);
        let file = format!("{}/{}", dir, last_38);
        fs::create_dir_all(dir).expect("Error creating directory");
        fs::write(file, &self.data).expect("Error writing file");
    }

    pub fn get_hash(&self) -> &RHash {
        &self.hash
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }
}