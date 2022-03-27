use std::{fs::OpenOptions, io::Write};

fn main() {
    let mut logs = OpenOptions::new()
        .append(true)
        .open("rusties/file-logger/src/logs.txt")
        .unwrap();

    logs.write_all(b"Hi I'am sos 1 \n").unwrap();
    logs.write_all(b"Hi I'am sos 2 \n").unwrap();
    logs.write_all(b"Hi I'am sos 3 \n").unwrap();
}
