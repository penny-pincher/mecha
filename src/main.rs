use std::{collections::HashMap, fs::read_to_string};

fn main() {
    let source = read_to_string("./README.md").unwrap();
    let mut files = HashMap::new();
    files.insert("README", source.clone());
    files.insert("README2", source);

    let files_ref = &mut files;

    print_borrowed_map(files_ref);

    let files_ref2 = &mut files;
    print_borrowed_map(files_ref2);
}

fn print_borrowed_map(map: &mut HashMap<&str, String>) {
    println!("{:?}", map)
}
