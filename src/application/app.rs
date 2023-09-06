
use super::file_handling::file_handling::*;
use super::file_handling::AudioPlayer::*;

use egui::*;


pub struct TemplateApp {
    music_library: Vec<music_file>,
    audio_player: AudioHandler,
    seek: f32,  // Sender to send filepath to audio handler thread
}

impl Default for TemplateApp {
    fn default() -> Self {


        Self {
            music_library: get_library(),
            audio_player: AudioHandler::new(),
            seek: 1.0,
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        Default::default()
    }
}


impl eframe::App for TemplateApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self { music_library,  .. } = self;

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                if self.audio_player.sink.empty() {
                    if ui.button("PLAY").clicked() {
                        // nothing
                    }   
                }
                else {
                    if self.audio_player.sink.is_paused() {
                        if ui.button("PLAY").clicked() {
                            self.audio_player.resume_playback();
                        }
                    }
                    else {
                        if ui.button("PAUSE").clicked() {
                            self.audio_player.pause_playback();
                        }
                    }
                }
                
                if ui.add(Slider::new(&mut self.seek, 0.0..=1.0)
                    .text("Volume")
                    .show_value(false)
                    .trailing_fill(true)).dragged(){
                        self.audio_player.sink.set_volume(self.seek);
                    };
                    
                
            })   
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                egui::Grid::new("some_unique_id")
                    .striped(true)
                    .max_col_width(200.0)
                    .min_col_width(200.0)
                    .show(ui, |ui|{
                        ui.label("Title:");
                        ui.label("Artist:");
                        ui.label("Album:");
                        ui.label("Duration:");
                        ui.end_row();
                        for z in music_library {
                            if &z.title == "" {
                                if ui.add(Label::new(&z.name).sense(Sense::click())).double_clicked() {
                                    self.audio_player.stop_playback();
                                    println!("double click executed");
                                    let file_path = &z.file_path;
                                    self.audio_player.load_file(file_path.as_path());
                                }
                            }
                            else {
                                if ui.add(Label::new(&z.title).sense(Sense::click())).double_clicked() {
                                    self.audio_player.stop_playback();
                                    println!("double click executed");
                                    let file_path = &z.file_path;
                                    self.audio_player.load_file(file_path.as_path());
                            }
                            }
                            ui.label(&z.artist);
                            ui.label(&z.album);
                            ui.label(&z.duration.to_string());
                            ui.end_row();
                        }
                });
            });
            
        });
        

    }
}

