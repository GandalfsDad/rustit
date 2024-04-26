use super::tree::Tree;
use crate::hash::rhash::RHash;
use std::rc::Rc;
use std::fs;

pub struct Commit {
    tree: Tree,
    message: String,
    hash: RHash,
    content: Vec<u8>,
    parent: Option<Rc<CommitEntry>>,
}

pub enum CommitEntry {
    Commit(Commit),
    CommitHash(RHash),
}

impl Commit {
    pub fn new(tree: Tree, message: String, parent: Option<CommitEntry>) -> Self {
        let content = Commit::gen_content(&tree, &message, &parent);
        let hash = Commit::gen_hash(&content);

        if let Some(parent) = parent {
            Commit { tree, message, hash, content, parent: Some(Rc::new(parent)) }
        } else {
            Commit { tree, message, hash, content, parent: None }
        }
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

    fn gen_content(tree: &Tree, message: &String, parent: &Option<CommitEntry>) -> Vec<u8> {
        let mut content = String::new();
        content.push_str(&format!("tree {}\n", tree.hash.as_string()));
        if let Some(parent) = parent {
            match parent {
                CommitEntry::Commit(commit) => {
                    content.push_str(&format!("parent {}\n\n", commit.hash.as_string()));
                },
                CommitEntry::CommitHash(hash) => {
                    content.push_str(&format!("parent {}\n\n", hash.as_string()));
                }
            }
        }
        content.push_str(&format!("message {}", message));
        content.as_bytes().to_vec()
    }

    fn gen_hash(content: &Vec<u8>) -> RHash {
        RHash::new(content)
    }
}