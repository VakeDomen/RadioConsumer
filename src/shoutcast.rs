use std::io::{BufRead, BufReader, Write, Read};
use std::net::TcpStream;
use std::str;
use std::time::Duration;

use crate::error::ShoutcastError;
use crate::radio::ShoutcastConfig;


pub fn listen_shoutcast(radio: ShoutcastConfig) {
    loop {
        let (_, host, port, path) = radio;
        let mut stream = match connect_to_server(host, port) {
            Ok(stream) => stream,
            Err(e) => return eprintln!("Error connecting to shoutcast server: {}", e),
        };
        println!("Connected to shoutcast server");
    
    
        if let Err(e) = stream.set_read_timeout(Some(Duration::from_secs(10))) {
            eprintln!("Error setting read timeout: {}", e);
            return;
        }
    
        if let Err(e) = send_request(&mut stream, path) {
            eprintln!("Error sending request: {}", e);
            return;
        }
        
        println!("Headers sent");
    
        let reader = BufReader::new(&stream);
        
        let meta_interval = match read_headers(reader) {
            Ok(meta_interval) => meta_interval,
            Err(e) => return eprintln!("Error reading headers: {}", e),
        };

        if meta_interval == 0 {
            eprintln!("Failed to get metadata byte_interval");
            return;
        }
        
        println!("Metadata byte Interval: {}", meta_interval);
    
        if let Err(e) = read_stream(&mut stream, meta_interval) {
            eprintln!("Error reading stream: {}", e);
        }
    }
}


fn connect_to_server(host: &str, port: u16) -> Result<TcpStream, ShoutcastError> {
    TcpStream::connect((host, port)).map_err(ShoutcastError::from)
}

fn send_request(stream: &mut TcpStream, path: &str) -> Result<(), ShoutcastError> {
    let request = format!("GET {} HTTP/1.0\r\nIcy-MetaData:1\r\n\r\n", path);
    stream.write_all(request.as_bytes()).map_err(ShoutcastError::from)
}

fn read_headers(mut reader: BufReader<&TcpStream>) -> Result<usize, ShoutcastError> {
    let mut meta_int = 0;
    let mut buffer = Vec::new();
    
    loop {
        buffer.clear();
        let bytes_read = reader.read_until(b'\n', &mut buffer).map_err(ShoutcastError::from)?;
        
        if bytes_read == 0 {
            break; // End of stream
        }
        
        let line = String::from_utf8_lossy(&buffer);
        println!("{:?}", line);
        if line.starts_with("icy-metaint:") {
            meta_int = line[12..].trim().parse::<usize>().map_err(ShoutcastError::from)?;
            break;
        }
        
        if line.trim().is_empty() {
            break; // End of headers
        }
    }
    Ok(meta_int)
}


fn read_stream(stream: &mut TcpStream, meta_int: usize) -> Result<(), ShoutcastError> {
    let mut mp3_data = vec![0; meta_int];
    let mut meta_length_buf = [0; 1];

    loop {
        stream.read_exact(&mut mp3_data)?;
        stream.read_exact(&mut meta_length_buf)?;

        let meta_length = meta_length_buf[0] as usize * 16;
        if meta_length > 0 {
            let mut metadata_buf = vec![0; meta_length];
            stream.read_exact(&mut metadata_buf)?;
            let metadata = String::from_utf8_lossy(&metadata_buf);
            
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