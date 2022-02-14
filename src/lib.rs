use std::fs;

const FILENAME_INDEX:usize = 1;
const QUERY_INDEX:usize = 2;

pub struct Config {
    pub filename: String,
    pub query: String
}

impl Config {

    pub fn new(args: &[String]) -> Config {
        let filename = args[FILENAME_INDEX].clone();
        let query = args[QUERY_INDEX].clone();

        Config {filename, query}
    }
}

pub fn run(config: Config) {
    let content = fs::read_to_string(config.filename).expect("Ocurred an error. Can't read the file");
    let found = search(&config.query, &content);

    for line in found {
        println!("{}", line);
    }
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();

    for line in content.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }

    result
}