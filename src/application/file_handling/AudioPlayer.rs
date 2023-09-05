use std::fs::File;
use std::io::BufReader;
use std::path::*;

use rodio::{Decoder, OutputStream, source::Source, Sink};

pub struct AudioHandler {
    pub sink: Sink,
    pub decoder: Option<Decoder<BufReader<File>>>,
}

impl Clone for AudioHandler {
    fn clone(&self) -> Self {
        // Create a new audio handler with a new sink and None for decoder
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        AudioHandler {
            sink: Sink::try_new(&stream_handle).unwrap(),
            decoder: None,
        }
    }
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
    } 
    
    pub fn play_file(&mut self) {
        if let Some(decoder) = self.decoder.take() {
            self.sink.append(decoder);
        }
    }
}
