use super::blob::Blob;
use crate::hash::rhash::RHash;
use std::{f32::consts::E, fs, io::Error};

pub struct Tree {
    pub entries: Vec<TreeEntry>,
    pub content: Vec<u8>,
    pub hash: RHash,
    pub name: String,
}

pub enum TreeEntry {
    Tree(Tree),
    Blob(Blob),
}

impl Tree {
    pub fn try_new(dir_path: &str) -> Result<Self, Error> {
        let name = dir_path.split("/").last().unwrap().to_string();
        let entries = Tree::get_entries_full_dir(dir_path)?;
        let content = Tree::gen_content(&entries);
        let hash = Tree::gen_hash(&content)?;
        Ok(Tree { entries, content,  hash, name })
    }

    pub fn from_paths(paths: Vec<String>, name: String) -> Result<Self, Error> {
        let entries = Tree::get_entries_from_paths(&paths)?;
        let content = Tree::gen_content(&entries);
        let hash = Tree::gen_hash(&content)?;
        Ok(Tree { entries, content,  hash, name })
    }

    pub fn get_hash(&self) -> &RHash {
        &self.hash
    }

    pub fn save(&mut self) -> () {
        let (first_two, last_38) = self.hash.split_hash();
        let dir = format!(".rustit/objects/{}", first_two);
        let file = format!("{}/{}", dir, last_38);
        fs::create_dir_all(dir).expect("Error creating directory");
        fs::write(file, &self.content).expect("Error writing file");
    }

    fn gen_hash( content: &Vec<u8>) -> Result<RHash, Error> {
        Ok(RHash::new(&content))
    }

    fn get_entries_full_dir(dir_path: &str) -> Result<Vec<TreeEntry>, Error> {
        let path_entries = fs::read_dir(dir_path)?;

        //Convert the path_entries to a Vec of string slice paths
        let paths: Vec<String> = path_entries
            .map(|entry| entry.unwrap().path().to_str().unwrap().to_string())
            .collect();

        Ok(Tree::get_entries_from_paths(&paths)?)
    }

    fn get_entries_from_paths(paths: &[String]) -> Result<Vec<TreeEntry>, Error> {
        let mut entries = Vec::new();

        for path in paths {
            let metadata = fs::metadata(path)?;

            if metadata.is_dir() {
                let tree = Tree::try_new(&path)?;
                entries.push(TreeEntry::Tree(tree));
            } else {
                let blob = Blob::try_new(&path)?;
                entries.push(TreeEntry::Blob(blob));
            }
        }

        Ok(entries)
    }

    fn gen_content(entries: &Vec<TreeEntry>) -> Vec<u8> {
        let mut content = String::new();

        for entry in entries {
            match entry {
                TreeEntry::Tree(tree) => {
                    content.push_str("tree ");
                    content.push_str(&tree.get_hash().as_string());
                    content.push_str(format!(" {}\n", &tree.name).as_str());
                },
                TreeEntry::Blob(blob) => {
                    content.push_str("blob ");
                    content.push_str(&blob.get_hash().as_string());
                    content.push_str(format!(" {}\n", &blob.get_name()).as_str());
                },
            }
        }
        
        let header = format!("tree {}\0", content.len());
        let data = [header.as_bytes(), content.as_bytes()].concat();

        data
    }
}
