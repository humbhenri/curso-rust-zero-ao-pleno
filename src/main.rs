use std::{fs::File, io::{Read, Write}};

fn count_lines(path: &str) -> usize {
    let mut file = File::open(path).expect("file open error");
    let mut content = String::from("");
    file.read_to_string(&mut content).expect("file read error");
    content.lines().count()
}

fn main() {
    let mut f = File::options().read(true).write(true).open("dados.txt").expect("file open error");
    let mut content = String::from("");
    f.read_to_string(&mut content).expect("file read error");
    println!("file content:\n {}", &content);
    content.push_str("\nmore lines\n1\n2\n3\nend.");
    f.write_all(content.as_bytes()).expect("file write error");
    println!("line count {}", count_lines("dados.txt"));
}
