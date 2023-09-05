use std::fs::File;
use std::io::BufReader;
use std::path::*;

use rodio::{Decoder, OutputStream, source::Source, Sink};

pub struct AudioHandler {
    pub sink: Sink,
    pub decoder: Option<Decoder<BufReader<File>>>,
}

impl AudioHandler {
    pub fn new() -> AudioHandler {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        AudioHandler {
            sink: Sink::try_new(&stream_handle).unwrap(),
            decoder: None,
        }
    }

    pub fn load_file(&mut self, path: &Path) {
        let file = File::open(path).unwrap();
        let buf_reader = BufReader::new(file);
        self.decoder = Some(Decoder::new(buf_reader).unwrap());
        println!("audio file loaded");
    }

    pub fn play_file(&mut self, path: &Path) {
        let file = File::open(path).unwrap();
        let buf_reader = BufReader::new(file);
        let decoder = Decoder::new(buf_reader).unwrap();
    
        self.sink.append(decoder);
        self.sink.play();
        println!("audio file playing");
        // Comment out the following line if you don't want to block the thread.
        // self.sink.sleep_until_end();
    }

}



    // Play the sound directly on the device
   // sink.append(source);



    // Play the sound in a seperate audio thread
    // so we need to keep the main thread alive while it's playing
   // sink.sleep_until_end();
