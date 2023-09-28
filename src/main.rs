use std::thread;

use shoutcast::listen_shoutcast;
use rockradio::listen_wss;
use radio::Radio;
mod shoutcast;
mod rockradio;
mod error;
mod radio;

fn main() {
    let stations = vec![
        // Radio::Shoutcast(("Hitradio Center", "stream.nextmedia.si", 443, "/proxy/center2?mp=/stream/1/")),
        // Radio::Shoutcast(("Aktual", "live.radio.si", 80, "/Aktual")),
        // Radio::Shoutcast(("Radio Ptuj", "live.radio.si", 80, "/Ptuj"))
        // Radio::Shoutcast(("Radio Veseljak", "live.radio.si", 80, "/Veseljak")),
        // Radio::Shoutcast(("Radio Maxi", "live.radio.si", 80, "/Maxi")),
        // Radio::Shoutcast(("Radio Salomon", "live.radio.si", 80, "/Salomon")),
        Radio::Shoutcast(("Koroški Radio", "live.radio.si", 80, "/Koroski")),
        // Radio::Shoutcast(("Radio1", "live.radio.si", 80, "/Radio1")),
        // Radio::Shoutcast(("Radio Sraka", "193.105.67.24", 8006, "/")),
        // Radio::Shoutcast(("Radio Koper", "mp3.rtvslo.si", 80, "/rakp")),
        // Radio::Shoutcast(("Val 202", "mp3.rtvslo.si", 80, "/val202")),
        // Radio::Shoutcast(("Radio City", "82.149.22.34", 8000, "/CityMp364mono.mp3")),
        // Radio::WssMetadataRadio(("Rock Radio", "wss://web1.nextmedia.si/ws/websocket/stream/9")),
    ];


    let mut running_channels = vec![];
    for station in stations.into_iter() {
        let thread_handle = match station {
            Radio::Shoutcast(conf) => thread::spawn(move || listen_shoutcast(conf)),
            Radio::WssMetadataRadio(wss) => thread::spawn(move || listen_wss(wss)),
        };
        running_channels.push(thread_handle);
    }

    // join the threads to run
    for handle in running_channels {
        handle.join().unwrap();
    }
}

