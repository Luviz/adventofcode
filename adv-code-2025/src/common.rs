use std::fs;


pub fn read_file(path: String) -> String{
    let contents = fs::read_to_string(&path);
    if contents.is_err() {
        println!("file not found: '{}'", &path);
    }
    return contents.unwrap();
}