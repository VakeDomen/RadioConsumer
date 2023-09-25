use std::io::BufReader;
use std::time::Duration;

use crate::shoutcast::{connect_to_server, send_request, read_headers, read_stream};

mod shoutcast;
mod error;

fn main() {
    let host = "mp3.rtvslo.si";
    let port = 80;
    let path = "/rakp";

    let mut stream = match connect_to_server(host, port) {
        Ok(stream) => stream,
        Err(e) => return eprintln!("Error connecting to server: {}", e),
    };
    println!("Connected to server");


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
        eprintln!("Failed to get meta_interval");
        return;
    }
    
    println!("Meta Interval: {}", meta_interval);

    if let Err(e) = read_stream(&mut stream, meta_interval) {
        eprintln!("Error reading stream: {}", e);
    }
}