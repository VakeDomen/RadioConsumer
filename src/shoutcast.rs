use std::io::{BufRead, BufReader, Write, Read};
use std::net::TcpStream;
use std::str;

use crate::error::ShoutcastError;

pub fn connect_to_server(host: &str, port: u16) -> Result<TcpStream, ShoutcastError> {
    TcpStream::connect((host, port)).map_err(ShoutcastError::from)
}

pub fn send_request(stream: &mut TcpStream, path: &str) -> Result<(), ShoutcastError> {
    let request = format!("GET {} HTTP/1.0\r\nIcy-MetaData:1\r\n\r\n", path);
    stream.write_all(request.as_bytes()).map_err(ShoutcastError::from)
}

pub fn read_headers(reader: BufReader<&TcpStream>) -> Result<usize, ShoutcastError> {
    let mut meta_int = 0;
    for line in reader.lines() {
        let line = line.map_err(ShoutcastError::from)?;
        if line.starts_with("icy-metaint:") {
            meta_int = line[12..].parse::<usize>().map_err(ShoutcastError::from)?;
            break;
        }
        if line.is_empty() {
            break;
        }
    }
    Ok(meta_int)
}

pub fn read_stream(stream: &mut TcpStream, meta_int: usize) -> Result<(), ShoutcastError> {
    let mut mp3_data = vec![0; meta_int];
    let mut meta_length_buf = [0; 1];

    loop {
        stream.read_exact(&mut mp3_data)?;
        stream.read_exact(&mut meta_length_buf)?;

        let meta_length = meta_length_buf[0] as usize * 16;
        if meta_length > 0 {
            let mut metadata_buf = vec![0; meta_length];
            stream.read_exact(&mut metadata_buf)?;

            let metadata = str::from_utf8(&metadata_buf)?;
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