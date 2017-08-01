extern crate rodio;
use std::fs::File;
use std::io::BufReader;
use rodio::Source;


fn main() {
    println!("Hello!");
    let endpoint = rodio::get_default_endpoint().unwrap();

    let file = File::open("../wav/amg.wav").unwrap();
    let mut player = rodio::play_once(&endpoint, BufReader::new(file)).unwrap();
    player.set_volume(0.1);
    player.play();
}
