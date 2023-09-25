use std::io::{BufRead, BufReader, Write, Read};
use std::net::TcpStream;
use std::str;

pub fn connect_to_server(host: &str, port: u16) -> TcpStream {
    TcpStream::connect((host, port)).expect("Could not connect to server")
}

pub fn send_request(stream: &mut TcpStream, path: &str) {
    let request = format!("GET {} HTTP/1.0\r\nIcy-MetaData:1\r\n\r\n", path);
    stream.write_all(request.as_bytes()).expect("Failed to write to stream");
}

pub fn read_headers(reader: BufReader<&TcpStream>) -> usize {
    let mut meta_int = 0;
    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        println!("Received: {}", line);

        if line.starts_with("icy-metaint:") {
            meta_int = line[12..].parse::<usize>().expect("Failed to parse meta_int");
            break;
        }

        if line.is_empty() {
            break;
        }
    }
    meta_int
}

pub fn read_stream(stream: &mut TcpStream, meta_int: usize) {
    let mut mp3_data = vec![0; meta_int];
    let mut meta_length_buf = [0; 1];

    loop {
        stream.read_exact(&mut mp3_data).expect("Failed to read mp3 data");
        stream.read_exact(&mut meta_length_buf).expect("Failed to read metadata length");

        let meta_length = meta_length_buf[0] as usize * 16;

        if meta_length > 0 {
            let mut metadata_buf = vec![0; meta_length];
            stream.read_exact(&mut metadata_buf).expect("Failed to read metadata");

            if let Ok(metadata) = str::from_utf8(&metadata_buf) {
                let trimmed_meta = metadata.trim_end_matches('\0');
                if let Some(start) = trimmed_meta.find("StreamTitle='") {
                    let remaining = &trimmed_meta[start + 13..];
                    if let Some(end) = remaining.find("';") {
                        let title = &remaining[..end];
                        println!("StreamTitle: {}", title);
                    }
                }
            }
        }
    }
}
