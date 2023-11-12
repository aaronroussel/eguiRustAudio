use std::fs::File;
use std::io::BufReader;
use std::path::*;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::collections::{vec_deque, VecDeque};
use std::time::Duration;
use circular_buffer::CircularBuffer;

use rodio::{Decoder, OutputStream, source::Source, Sink, Sample};

pub struct AudioHandler {
    pub sink: Sink,
    pub stream: OutputStream,
    pub samples_for_viz: Vec<f32>,                 // Samples for visualization
    pub sample_index: Arc<AtomicUsize>,            // Atomic iterator/index
    pub duration: u64,
    pub circular_buffer: Arc<Mutex<CircularBuffer<2048, f32>>>,
}

impl AudioHandler {
    pub fn new() -> AudioHandler {
        let (stream, stream_handle) = OutputStream::try_default().unwrap();
        AudioHandler {
            sink: Sink::try_new(&stream_handle).unwrap(),
            stream,
            samples_for_viz: Vec::new(),
            sample_index: Arc::new(AtomicUsize::new(0)),
            duration: 0,
            circular_buffer: Arc::new(Mutex::new(CircularBuffer::<2048, f32>::new()))
        }
    }

    pub fn load_file(&mut self, path: &Path) {
        let file_for_playback = File::open(path).unwrap();
        let source_for_playback = Decoder::new(BufReader::new(file_for_playback)).unwrap();
        let buffer = self.circular_buffer.clone();
        let (indexed_source, sample_index) = IndexedSource::new(source_for_playback.convert_samples::<f32>(), buffer );
        self.sample_index = sample_index.clone();
        self.sink.append(indexed_source);
        self.sink.play();
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

pub struct IndexedSource<S>
    where
        S: Source<Item = f32> + Send,
{
    inner: S,
    pub index: Arc<AtomicUsize>,
    pub buf_ref: Arc<Mutex<CircularBuffer<2048, f32>>>,
}

impl<S> IndexedSource<S>
    where
        S: Source<Item = f32> + Send,
{
    pub fn new(source: S, buffer_ref: Arc<Mutex<CircularBuffer<2048, f32>>>) -> (Self, Arc<AtomicUsize>) {
        let index = Arc::new(AtomicUsize::new(0));
        let buffer_ref = buffer_ref.clone();
        (Self { inner: source, index: index.clone(), buf_ref: buffer_ref }, index)
    }
}

impl<S> Source for IndexedSource<S>
    where
        S: Source<Item = f32> + Send,
{
    fn current_frame_len(&self) -> Option<usize> {
        self.inner.current_frame_len()
    }

    fn channels(&self) -> u16 {
        self.inner.channels()
    }

    fn sample_rate(&self) -> u32 {
        self.inner.sample_rate()
    }

    fn total_duration(&self) -> Option<std::time::Duration> {
        self.inner.total_duration()
    }
}

impl<S> Iterator for IndexedSource<S>
    where
        S: Source<Item = f32> + Send,
{
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        let sample = self.inner.next();
        if let Some(sample_value) = sample {
            self.index.fetch_add(1, Ordering::Relaxed);
            let mut buffer = self.buf_ref.lock().unwrap();
            buffer.push_back(sample_value);
        }
        sample
    }
}