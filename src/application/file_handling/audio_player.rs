use std::fs::File;
use std::io::BufReader;
use std::path::*;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::collections::{vec_deque, VecDeque};

use rodio::{Decoder, OutputStream, source::Source, Sink, Sample};

pub struct AudioHandler {
    pub sink: Sink,
    pub stream: OutputStream,
    pub samples_for_viz: Vec<f32>,                 // Samples for visualization
    pub sample_index: Arc<AtomicUsize>,            // Atomic iterator/index
    pub duration: u32,
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
        }
    }

    pub fn load_file(&mut self, path: &Path) {
        // Decode once for extracting samples
        let file_for_viz = File::open(path).unwrap();
        let source_for_viz = Decoder::new(BufReader::new(file_for_viz)).unwrap();
        self.samples_for_viz = source_for_viz.convert_samples::<f32>().collect();

        // Decode again for samples to be used by visualizer
        let file_for_playback = File::open(path).unwrap();
        let source_for_playback = Decoder::new(BufReader::new(file_for_playback)).unwrap();

        // Wrap the source with our indexed source
        let (indexed_source, sample_index) = IndexedSource::new(source_for_playback);

        // Save the indexed source and sample index
        self.sample_index = sample_index.clone();


        self.sink.append(indexed_source); // Assuming IndexedSource implements Clone

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

pub struct IndexedSource<S> {
    inner: S,
    pub index: Arc<AtomicUsize>,
}

impl<S> IndexedSource<S> {
    pub fn new(source: S) -> (Self, Arc<AtomicUsize>) {
        let index = Arc::new(AtomicUsize::new(0));
        (Self { inner: source, index: index.clone() }, index)
    }
}

impl<S> Source for IndexedSource<S>
    where
        S: Source,
        S::Item: Sample,
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
        S: Source,
        S::Item: Sample,
{
    type Item = S::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let sample = self.inner.next();
        if sample.is_some() {
            self.index.fetch_add(1, Ordering::Relaxed);
        }
        sample
    }
}
