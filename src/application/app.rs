
use super::file_handling::file_handling::*;
use super::file_handling::audio_player::*;

use egui_modal;
use egui::*;
use egui::Color32;


pub struct TemplateApp {
    music_library: Vec<MusicFile>,
    audio_player: AudioHandler,
    seek: f32,
    fp: String, // Sender to send filepath to audio handler thread
}

impl Default for TemplateApp {
    fn default() -> Self {


        Self {
            music_library: get_library().unwrap(),
            audio_player: AudioHandler::new(),
            seek: 1.0,
            fp: "".to_owned(),
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        
        Default::default()
    }
}


impl eframe::App for TemplateApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // let Self { music_library,  .. } = self;

        let filepath_modal = egui_modal::Modal::new(ctx, "filepath modal")
        .with_close_on_outside_click(true);
        filepath_modal.show(|ui| {
            filepath_modal.title(ui, "Enter File Path");

            filepath_modal.frame(ui, |ui|{
                
                let fp_edit = ui.add(TextEdit::singleline(&mut self.fp)
                    .hint_text("Enter filepath"));
                if fp_edit.changed()
                {
                    
                }
            });
            filepath_modal.buttons(ui, |ui| {
                if filepath_modal.button(ui, "Add Folder").clicked() {
                    let new_music_files = get_from_path(&self.fp);
                    for x in new_music_files {
                        self.music_library.push(x);
                    }
                    filepath_modal.close();
                    self.fp = "".to_owned();
                }
                if filepath_modal.button(ui, "Cancel").clicked() {
                    filepath_modal.close();
                }        
            });
        });
        
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui|{
                ui.menu_button("File", |ui|{
                    if ui.button("Add music file").clicked() {
                        // filepath_modal.open()    this adds by folder, not file. will fix later
                    }
                    if ui.button("Add music folder").clicked() {
                        filepath_modal.open()
                    }
                });
                ui.menu_button("Edit", |ui| {
                    if ui.button("Create new Playlist").clicked() {
                        
                    }
                })   
            });
            ui.style_mut().spacing.slider_width = 100.0;
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
                    
                ui.add(ProgressBar::new(0.5)
                    .fill(Color32::GRAY)
                    .desired_width(400.0)
                    .text("1:29"));
                    
                
            })   
        });
        
        egui::SidePanel::left("left panel").show(ctx, |ui| {
            ui.label("Library");
            
            egui::CollapsingHeader::new("Playlists").open(Some(true)).show(ui, |ui|{
                ui.label("Playlist 1");
            });
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
                        for z in &self.music_library {
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

