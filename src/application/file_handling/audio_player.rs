use circular_buffer::CircularBuffer;
use std::collections::{vec_deque, VecDeque};
use std::fs::File;
use std::io::BufReader;
use std::path::*;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use rodio::{source::Source, Decoder, OutputStream, Sample, Sink};

// this is the audio handler, it is responsible for handling all audio related tasks

pub struct AudioHandler {
    pub sink: Sink,           // controls audio playback to the OS
    pub stream: OutputStream, // output stream for audio
    // pub samples_for_viz: Vec<f32>,      // Samples for visualization [no longer used]]
    pub sample_index: Arc<AtomicUsize>, // Atomic iterator/index [for playback position tracking]
    pub duration: u64,                  // total number of samples
    pub circular_buffer: Arc<Mutex<CircularBuffer<2048, f32>>>, // reference to cicrular buffer for audio data [used for the visualizer]
}

impl AudioHandler {
    pub fn new() -> AudioHandler {
        let (stream, stream_handle) = OutputStream::try_default().unwrap();
        AudioHandler {
            sink: Sink::try_new(&stream_handle).unwrap(),
            stream,
            sample_index: Arc::new(AtomicUsize::new(0)),
            duration: 0,
            circular_buffer: Arc::new(Mutex::new(CircularBuffer::<2048, f32>::new())),
        }
    }

    pub fn load_file(&mut self, path: &Path) {
        // -----------------------------------------------------------------------------------------------
        // load a music fine and append it to the sink
        // Path should be fetch from a music file object
        // -----------------------------------------------------------------------------------------------
        let file_for_playback = File::open(path).unwrap();
        let source_for_playback = Decoder::new(BufReader::new(file_for_playback)).unwrap();
        let buffer = self.circular_buffer.clone();
        let converted_samples = source_for_playback.convert_samples::<f32>();
        // -----------------------------------------------------------------------------------------------
        // i may have to do some kind of hack here to get the duration of the song,
        // involving extracting a raw vector of all the sample data, getting the length then slapping
        // it back into a decoder object and so i can throw it back into an indexed source
        // -----------------------------------------------------------------------------------------------
        let (indexed_source, sample_index) = IndexedSource::new(converted_samples, buffer);
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

// -----------------------------------------------------------------------------------------------
// IndexedSource is a wrapper around a rodio source that keeps track of the current index of the
// audio playback. It also keeps a reference to a circular buffer that is used for the visualizer
// -----------------------------------------------------------------------------------------------
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
    pub fn new(
        source: S,
        buffer_ref: Arc<Mutex<CircularBuffer<2048, f32>>>,
    ) -> (Self, Arc<AtomicUsize>) {
        let index = Arc::new(AtomicUsize::new(0));
        let buffer_ref = buffer_ref.clone();
        (
            Self {
                inner: source,
                index: index.clone(),
                buf_ref: buffer_ref,
            },
            index,
        )
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

// -----------------------------------------------------------------------------------------------
// This is the iterator implementation for the IndexedSource
// each time we fetch the next sample, we increment the index and push the sample into the cicular
// buffer so we can use those samples for the visualizer
//
// ** buffer sizes > 2048 will create discontinuities in the audio signal, which we don't want
// this is due to the fact the we are locking and unlocking a mutex each time we push samples
// into the buffer **
// -----------------------------------------------------------------------------------------------
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
