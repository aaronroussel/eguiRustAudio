use std::fs::File;
use std::io::BufReader;
use std::path::*;

use rodio::{Decoder, OutputStream, Sink};

pub struct AudioHandler {
    pub sink: Sink,
    pub stream: OutputStream,
}

impl AudioHandler {
    pub fn new() -> AudioHandler {
        let (stream, stream_handle) = OutputStream::try_default().unwrap();
        AudioHandler {
            sink: Sink::try_new(&stream_handle).unwrap(),
            stream,  // Store the stream to keep it alive
        }
    }

    pub fn load_file(&mut self, path: &Path) {
        let file = File::open(path).unwrap();
        let buf_reader = BufReader::new(file);
        let source = Decoder::new(buf_reader).unwrap();
        self.sink.append(source);
        self.sink.play();
        println!("audio file playing");
    }

    pub fn pause_playback(&mut self) {
        self.sink.pause();
        println!("audio paused");
    }

    pub fn stop_playback(&mut self) {
        self.sink.stop();
        println!("audio stopped");
    }

    pub fn resume_playback(&mut self) {
        self.sink.play();
    }
}



