use std::fs;

pub fn get_file_contents(path: &String) -> Result<String, &str> {
    match fs::read_to_string(path) {
        Ok(data) => Ok(data),
        Err(_) => Err("File doesn't exist or contents couldn't be read"),
    }
}
