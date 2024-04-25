use rustit::Blob;

fn main() {
    let mut blob = Blob::try_new("Cargo.toml").expect("Error reading file");
    blob.save();
}
