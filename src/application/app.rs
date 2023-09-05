use std::thread;
use super::file_handling::file_handling::*;
use super::file_handling::AudioPlayer::*;
use egui::*;
use std::sync::{Arc, Mutex, Condvar};


/// We derive Deserialize/Serialize so we can persist app state on shutdown.
// #[derive(serde::Deserialize, serde::Serialize)]
// #[serde(default)] // if we add new fields, give them default values when deserializing old state
 
pub struct TemplateApp {
    music_library: Vec<music_file>,
    audio_handler: Arc<Mutex<AudioHandler>>,
    play_signal: Arc<(Mutex<bool>, Condvar)>,
}


impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            music_library: get_library(),
            audio_handler: Arc::new(Mutex::new(AudioHandler::new())),
            play_signal: Arc::new((Mutex::new(false), Condvar::new())),
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
    
    
    /// Called by the frame work to save state before shutdown.


    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let music_library = &self.music_library;
        let audio_handler = &mut self.audio_handler;

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
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
                    if ui.add(Label::new(&x.title).sense(Sense::click())).double_clicked() {
                        
                        let file_path = x.file_path.clone();
                        
                        let audio_handler = self.audio_handler.clone();
                        let play_signal = self.play_signal.clone();
                        
                        thread::spawn(move || {
                            let mut audio_handler = audio_handler.lock().unwrap();
                            audio_handler.load_file(&file_path);

                            // Signal that the audio is ready to be played
                            let (lock, cvar) = &*play_signal;
                            let mut play_ready = lock.lock().unwrap();
                            *play_ready = true;
                            cvar.notify_one();

                            // Play the audio
                            audio_handler.play_file();
                        });
                    }
                }
            });
        });

        if false {
            egui::Window::new("Window").show(ctx, |ui| {
                ui.label("Windows can be moved by dragging them.");
                ui.label("They are automatically sized based on contents.");
                ui.label("You can turn on resizing and scrolling if you like.");
                ui.label("You would normally choose either panels OR windows.");
            });
        }
    }
}
