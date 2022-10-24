use std::{fs::read_to_string, fs::File, io::Write};

fn main() {
    let body: &str = "Lorem Ipsum";
    let path_file_name: &str = "file.txt";

    let mut file_item = File::create(path_file_name).unwrap();
    file_item.write_all(body.as_bytes()).unwrap();
    println!("file created");

    let body = read_to_string(path_file_name).unwrap();
    println!("content file {}", body);
}
