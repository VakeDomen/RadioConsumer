use std::thread;

use shoutcast::listen_shoutcast;
use rockradio::listen_wss;
use radio::Radio;
mod shoutcast;
mod rockradio;
mod error;
mod radio;

// iffy 
// radio express
// radio hit ne ve kaj vrti (https://siradio.si/hit/) glej tle za playlisto
// capris https://www.radiocapris.si/

fn main() {
    let stations = vec![
        // Radio::Shoutcast(("Hitradio Center", "stream.nextmedia.si", 443, "/proxy/center2?mp=/stream/1/")),
        // Radio::Shoutcast(("Aktual", "live.radio.si", 80, "/Aktual")),
        // Radio::Shoutcast(("Radio Ptuj", "live.radio.si", 80, "/Ptuj")),
        // Radio::Shoutcast(("Radio Veseljak", "live.radio.si", 80, "/Veseljak")),
        // Radio::Shoutcast(("Radio Maxi", "live.radio.si", 80, "/Maxi")),
        // Radio::Shoutcast(("Radio Salomon", "live.radio.si", 80, "/Salomon")),
        // Radio::Shoutcast(("Radio Antena", "live.radio.si", 80, "/Antena")),
        // Radio::Shoutcast(("Radio Ognjišče", "live.radio.si", 80, "/ognjisce.mp3")),
        // Radio::Shoutcast(("Radio1", "live.radio.si", 80, "/Radio1")),
        // Radio::Shoutcast(("Radio Net FM (maribor)", "reflector.radionet.si", 8000, "/stream.ogg")),
        // Radio::Shoutcast(("Radio Prvi", "mp3.rtvslo.si", 80, "/ra1")),
        // Radio::Shoutcast(("Radio SI", "mp3.rtvslo.si", 80, "/rsi")),
        // Radio::Shoutcast(("Radio Celje", "live.radio.si", 80, "/Celje")),
        // Radio::Shoutcast(("Center YU", "stream3.radiocenter.si", 8100, "/stream/1/")),
        // Radio::Shoutcast(("Radio BOB", "live.radio.si", 80, "/BOB")),
        // Radio::Shoutcast(("Radio Robin", "live.radio.si", 80, "/Robin")),
        // Radio::Shoutcast(("Radio Triglav", "live.radio.si", 80, "/Triglav")),
        // Radio::Shoutcast(("Radio Maribor", "mp3.rtvslo.si", 80, "/rmb")),
        // Radio::Shoutcast(("ARS", "mp3.rtvslo.si", 80, "/ars")),
        // Radio::Shoutcast(("Radio Prlek", "86.61.68.81", 8000, "/")), // reverse song-artist in ni caps
        // Radio::Shoutcast(("Radio94", "77.38.12.198", 8000, "/radio94")),
        // Radio::Shoutcast(("Radio Kranj", "live.radio.si", 80, "/Kranj")),
        // Radio::Shoutcast(("Radio Sraka", "193.105.67.24", 8006, "/")),
        // Radio::Shoutcast(("Radio Koper", "mp3.rtvslo.si", 80, "/rakp")),
        // Radio::Shoutcast(("Val 202", "mp3.rtvslo.si", 80, "/val202")),
        // Radio::Shoutcast(("Radio City", "82.149.22.34", 8000, "/CityMp364mono.mp3")),
        Radio::WssMetadataRadio(("Rock Radio", "wss://web1.nextmedia.si/ws/websocket/stream/9")), 
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

