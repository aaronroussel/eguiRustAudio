use std::thread;
use std::sync::mpsc;
use super::file_handling::file_handling::*;
use super::file_handling::AudioPlayer::*;
use egui::*;
use std::sync::{Arc, Mutex};

pub struct TemplateApp {
    music_library: Vec<music_file>,
    sender: mpsc::Sender<String>,  // Sender to send filepath to audio handler thread
}

impl Default for TemplateApp {
    fn default() -> Self {
        let (tx, rx) = mpsc::channel::<String>();  // Create a channel

        // Create and start audio handler thread
        let audio_handler = Arc::new(Mutex::new(AudioHandler::new()));
        let audio_handler_clone = audio_handler.clone();
        thread::spawn(move || {
            loop {
                let filepath = rx.recv().unwrap();  // Blocking wait for a message
                let path = std::path::Path::new(&filepath);  // Convert String to Path
                let mut audio_handler_locked = audio_handler.lock().unwrap();
                audio_handler_locked.load_file(&path);
                audio_handler_locked.play_file();
            }
        });

        Self {
            music_library: get_library(),
            sender: tx,
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.

        Default::default()
    }
}


impl eframe::App for TemplateApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self { music_library, sender, .. } = self;

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.vertical_centered(|ui|{
                if ui.button("PLAY").clicked() {
                    _frame.close();
                }
            })   
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                for x in music_library {
                    if ui.add(Label::new(&x.name).sense(Sense::click())).double_clicked() {
                        // Send the filepath to the audio handler thread
                        let _ = sender.send(x.file_path.to_str().unwrap().to_string());
                    }
                }
            });
        });
    }
}
