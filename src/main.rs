use std::io::{BufRead, BufReader, Write, Read}; // Importing necessary traits for reading and writing streams.
use std::net::TcpStream; // Importing TcpStream for connecting to the server.
use std::str; // Importing str for converting byte slices to strings.
use std::time::Duration; // Importing Duration for setting the read timeout on the stream.

fn main() {
    let host = "mp3.rtvslo.si";
    let port = 80;
    let path = "/rakp";
    
    // Connecting to the server and creating a TCP stream.
    // The expect function is used to panic if the connection fails, with a descriptive message.
    let mut stream = TcpStream::connect((host, port)).expect("Could not connect to server");
    println!("Connected to server");  
    
    // Setting a read timeout to avoid hanging indefinitely if the server stops sending data.
    stream.set_read_timeout(Some(Duration::from_secs(10))).expect("Failed to set read timeout");
    
    // Constructing and sending the HTTP GET request to the server, with Icy-MetaData header to request metadata.
    let request = format!("GET {} HTTP/1.0\r\nIcy-MetaData:1\r\n\r\n", path);
    stream.write_all(request.as_bytes()).expect("Failed to write to stream");
    println!("Headers sent");
    
    // Wrapping the stream in a BufReader allows reading line by line.
    let reader = BufReader::new(&stream);
    
    let mut meta_int = 0;
    // Reading the server's response headers line by line to find the icy-metaint value.
    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        println!("Received: {}", line); 
        
        // icy-metaint header indicates the interval at which metadata is sent.
        if line.starts_with("icy-metaint:") {
            meta_int = line[12..].parse::<usize>().expect("Failed to parse meta_int");
            break;
        }
        
        // An empty line signals the end of the headers.
        if line.is_empty() {
            break;
        }
    }
    
    // If icy-metaint header was not found, we cannot proceed.
    if meta_int == 0 {
        println!("Failed to get meta_int");
        return;
    }
    println!("Meta Interval: {}", meta_int);
    
    // Allocating a buffer to read the MP3 data sent before every metadata chunk.
    let mut mp3_data = vec![0; meta_int];
    let mut meta_length_buf = [0; 1];

    loop {
        // Reading and discarding mp3 data, we are interested in metadata only.
        stream.read_exact(&mut mp3_data).expect("Failed to read mp3 data");
        
        // Reading the byte that tells the length of the upcoming metadata.
        stream.read_exact(&mut meta_length_buf).expect("Failed to read metadata length");
        
        let meta_length = meta_length_buf[0] as usize * 16; // Calculating metadata length.

        if meta_length > 0 {
            // If there is metadata, read it into a buffer.
            let mut metadata_buf = vec![0; meta_length];
            stream.read_exact(&mut metadata_buf).expect("Failed to read metadata");
            
            // Converting metadata buffer to a string and parsing to extract the StreamTitle.
            if let Ok(metadata) = str::from_utf8(&metadata_buf) {
                let trimmed_meta = metadata.trim_end_matches('\0'); // Trimming null characters at the end of metadata.
                if let Some(start) = trimmed_meta.find("StreamTitle='") {
                    let remaining = &trimmed_meta[start + 13..];
                    if let Some(end) = remaining.find("';") {
                        let title = &remaining[..end];
                        println!("StreamTitle: {}", title); // Logging the extracted StreamTitle.
                    }
                }
            }
        }
    }
}
