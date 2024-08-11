use std::{fs::File, io::{BufRead, BufReader}, path::PathBuf};

pub fn read_file_lines(path: &PathBuf) -> Box<[String]> {
    let mut content = Vec::<String>::new();

    let file = match File::open(path) {
        Ok(file) => file,
        Err(_) => panic!("Failed to open file: {:?}", path)
    };

    let reader = BufReader::new(file);

    for line in reader.lines() {
        if let Ok(line) = line {
            if line.is_empty() {
                continue;
            } else {
                content.push(line.trim().to_string());
            }
        } else {
            panic!("Failed to read line: {}", line.unwrap());
        }
    }

    content.into_boxed_slice()
}