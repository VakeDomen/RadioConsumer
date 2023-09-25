use std::io::BufReader;
use std::time::Duration;

use crate::shoutcast::{connect_to_server, send_request, read_headers, read_stream};


mod shoutcast;

fn main() {
    let host = "mp3.rtvslo.si";
    let port = 80;
    let path = "/rakp";
    
    let mut stream = connect_to_server(host, port);
    println!("Connected to server");

    stream.set_read_timeout(Some(Duration::from_secs(10))).expect("Failed to set read timeout");

    send_request(&mut stream, path);
    println!("Headers sent");

    let reader = BufReader::new(&stream);
    let meta_int = read_headers(reader);

    if meta_int == 0 {
        println!("Failed to get meta_int");
        return;
    }
    println!("Meta Interval: {}", meta_int);

    read_stream(&mut stream, meta_int);
}
