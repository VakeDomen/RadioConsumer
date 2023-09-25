use shoutcast::shoutcast_listen;
use rockradio::listen_rockradio;

mod shoutcast;
mod rockradio;
mod error;


#[tokio::main]
async fn main() {
    listen_rockradio("wss://web1.nextmedia.si/ws/websocket/stream/9").await;
    // let host = "mp3.rtvslo.si";
    // let port = 80;
    // let path = "/rakp";


    // shoutcast_listen(host, port, path)
}

