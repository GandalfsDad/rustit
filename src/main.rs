use rustit::objects::{blob::Blob, tree::Tree, commit::Commit, commit::CommitEntry};
use rustit::hash::rhash::RHash;



fn main() {

    let files = vec!["./src/main.rs", 
                                "./src/lib.rs", 
                                "./src/hash.rs",
                                "./src/hash/rhash.rs"];

    let folders = vec!["./src/hash"];

    for file in files {
        let mut blob: Blob = Blob::try_new(file).expect("Error reading file");
        blob.save();
    }

    for folder in folders {
        let mut tree: Tree = Tree::try_new(folder).expect("Error reading file");
        tree.save();
    }

    let initial_commit_tree_paths = vec![
        "./src/hash",
        "./src/lib.rs",
        "./src/hash.rs",
        "./src/main.rs",
    ];

    let initial_commit_tree_paths = initial_commit_tree_paths.iter().map(|s| s.to_string()).collect();

    let mut tree: Tree = Tree::from_paths(initial_commit_tree_paths, "src".to_string()).expect("Error reading file");
    tree.save();

    let mut commit = Commit::new(tree, "Initial commit".to_string(), None);
    commit.save();

    println!("Initial commit hash: {}", commit.get_hash().as_string());
    
    let files = vec!["./src/main.rs", 
                                "./src/lib.rs", 
                                "./src/objects.rs",
                                "./src/hash.rs",
                                "./src/objects/blob.rs",
                                "./src/objects/tree.rs",
                                "./src/hash/rhash.rs"];

    let folders = vec!["./src/objects", "./src/hash"];

    for file in files {
        let mut blob: Blob = Blob::try_new(file).expect("Error reading file");
        blob.save();
    }

    for folder in folders {
        let mut tree: Tree = Tree::try_new(folder).expect("Error reading file");
        tree.save();
    }

    let mut tree: Tree = Tree::try_new("./src").expect("Error reading file");
    tree.save();

    let mut second_commit = Commit::new(tree, "Second commit".to_string(), Some(CommitEntry::Commit(commit)));
    second_commit.save();

    println!("Second commit hash: {}", second_commit.get_hash().as_string());
}
