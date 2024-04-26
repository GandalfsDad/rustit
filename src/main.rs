use rustit::objects::blob::Blob;
use rustit::objects::tree::Tree;


fn main() {

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
    

}
