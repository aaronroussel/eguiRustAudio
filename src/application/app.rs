use std::thread;
use super::file_handling::file_handling::*;
use super::file_handling::AudioPlayer::*;
use egui::*;


/// We derive Deserialize/Serialize so we can persist app state on shutdown.
// #[derive(serde::Deserialize, serde::Serialize)]
// #[serde(default)] // if we add new fields, give them default values when deserializing old state
 
pub struct TemplateApp {
    // Example stuff:
    music_library: Vec<music_file>,
    audio_handler: AudioHandler,

    // this how you opt-out of serialization of a member
    // #[serde(skip)]

}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            music_library: get_library(),
            audio_handler: AudioHandler::new()
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
        let Self {music_library, audio_handler} = self;

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
                        let file_path = x.file_path.clone(); // Clone the file path if needed

                        let audio_handler = &mut audio_handler; // Borrow mutably for the closure

                        thread::spawn(move || {
                            audio_handler.load_file(&file_path);
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
