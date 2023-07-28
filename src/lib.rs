mod utils;
use std::error::Error;
use utils::file_utils;

pub fn run(input: Input) -> Result<(), Box<dyn Error>> {
    let contents = file_utils::get_file_contents(&input.file_path)?;
    println!("Contents are => {}", contents);

    let res = search(&input.text, &contents);

    println!("Search result is {:?}", res);

    Ok(())
}
pub struct Input {
    pub text: String,
    pub file_path: String,
}

impl Input {
    pub fn new(args: &[String]) -> Result<Self, &str> {
        if args.len() < 2 {
            return Err("Required arguments not passed");
        }
        Ok(Input {
            text: args[1].clone(),
            file_path: args[2].clone(),
        })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results: Vec<&str> = Vec::new();

    for text in contents.lines() {
        if text.contains(query) {
            results.push(text);
        }
    }
    results
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_search() -> () {
        ()
    }
}
