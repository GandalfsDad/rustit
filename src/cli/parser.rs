use std::collections::HashMap;
use std::fs;

use crate::objects::blob::Blob;


pub fn parse_command(args:&Vec<String>) {
    let commands = _setup_commands();

    let command = args[1].clone();

    if command != "init" {
        _check_init();
    }
    
    let command = commands.get(&command).unwrap();
    command(&args[2..].to_vec());
}

pub fn init(args: &Vec<String>) {
    println!("Initializing rustit repoisitory...");
    
    // Create the .rustit directory
    fs::create_dir_all(".rustit/objects").expect("Error creating directory");
    fs::File::create("./.rustit/HEAD").expect("Error creating file");
}

pub fn hash_object(args: &Vec<String>) {
    println!("Hashing object...");
    
    //Iterate over the args and hash each object
    for arg in args {
        let mut blob = Blob::try_new(arg).unwrap();
        blob.save();
        println!("Hash: {}", blob.get_hash().as_string());
    }

}

fn _setup_commands() -> HashMap<String, fn(&Vec<String>)> {
    let mut commands = HashMap::new();
    commands.insert("init".to_string(), init as fn(&Vec<String>));
    commands.insert("hash-object".to_string(), hash_object as fn(&Vec<String>));
    commands
}

fn _check_init() {
    if !fs::metadata(".rustit").is_ok() {
        panic!("Not a rustit repository. Please run 'rustit init' to initialize a repository");
    }
}