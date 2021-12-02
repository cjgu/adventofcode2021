use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn load_file(file_path: &str) -> Vec<String> {
    let mut content = vec![];

    let f = File::open(file_path).expect("Unable to open file");

    let br = BufReader::new(f);

    for line in br.lines() {
        let l = line.unwrap();
        content.push(l);
    }

    content
}
