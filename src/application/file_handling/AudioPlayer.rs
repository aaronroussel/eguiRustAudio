use std::fs::File;
use std::io::BufReader;
use std::path::*;
use rodio::{Decoder, OutputStream, source::Source, Sink};

pub struct AudioHandler {
    pub sink: Sink,
    pub file: Option<File>,
    pub source: Option<Decoder<Option<File>>>,
}

impl AudioHandler {
    pub fn new() -> AudioHandler {
        AudioHandler {
            sink: Sink::try_new(&OutputStream::try_default().unwrap()).unwrap(),
            file: None,
            source: None
        }    
    }
    
    pub fn load_file(&mut self, path: &Path) {
        self.file = BufReader::new(File::open(path).unwrap());
        self.source = Decoder::new(&self.file).unwrap();      
    } 
    
    pub fn play_file(&mut self) {
        (self.sink).append(self.source);
    }
    
}

pub fn audio_player() {
    // Get an output stream handle to the default physical sound device
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    // Create a sink to handle control of the audio playback
    let sink = Sink::try_new(&stream_handle).unwrap();
    // Load a sound from a file, using a path relative to Cargo.toml
    let file = BufReader::new(File::open("music/juicewrld-alone.mp3").unwrap());
    // Decode that sound file into a source
    let source = Decoder::new(file).unwrap();
    // Play the sound directly on the device
    sink.append(source);



    // Play the sound in a seperate audio thread
    // so we need to keep the main thread alive while it's playing
    sink.sleep_until_end();
}