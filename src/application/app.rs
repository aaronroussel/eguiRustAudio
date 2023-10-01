
use super::file_handling::file_handling::*;
use super::file_handling::AudioPlayer::*;
use std::collections::VecDeque;
use std::fs::File;
use std::path::Path;
use std::sync::atomic::Ordering;

use egui_modal;
use egui::*;
use egui::Color32;
use egui::WidgetType::ComboBox;


pub struct TemplateApp {
    music_library: Vec<music_file>,
    audio_player: AudioHandler,
    seek: f32,
    fp: String,
    visualizer_active: bool,
    visualizer_effect_lines: bool,
    visualizer_style: i8,
    buffer_size: usize,
}

impl Default for TemplateApp {
    fn default() -> Self {


        Self {
            music_library: get_library(),
            audio_player: AudioHandler::new(),
            seek: 1.0,
            fp: "".to_owned(),
            visualizer_active: false,
            visualizer_effect_lines: true,
            visualizer_style: 0,
            buffer_size: 512,
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


        // Function to check song queue and load next song
        /*
        if self.audio_player.sink.empty() {
            if !self.song_queue.is_empty() {

            }
        }
        */
        ctx.set_visuals(egui::Visuals::dark());

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
                if filepath_modal.button(ui, "close").clicked() {
                    let new_music_files = get_from_path(&self.fp);
                    for x in new_music_files {
                        self.music_library.push(x);
                    }
                    filepath_modal.close();
                    self.fp = "".to_owned();
                }        
            });
        });
        
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui|{
                ui.menu_button("File", |ui|{
                    if ui.button("Add music file").clicked() {
                        filepath_modal.open()
                    }
                    if ui.button("Add music folder").clicked() {
                        // do stuff here
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
                    .trailing_fill(true))
                    .dragged(){
                        self.audio_player.sink.set_volume(self.seek);
                    };

                let usize_val = self.audio_player.sample_index.load(Ordering::Relaxed);

                ui.add(ProgressBar::new(usize_val as f32 / self.audio_player.samples_for_viz.len() as f32 )
                    .fill(Color32::GRAY)
                    .desired_width(400.0)
                    );

                if ui.button("Viz").clicked() {
                    self.visualizer_active = true
                }
            })   
        });
        
        egui::SidePanel::left("left panel").show(ctx, |ui| {
            ui.label("Library");
            
            egui::CollapsingHeader::new("Playlists").open(Some(true)).show(ui, |ui|{
                ui.label("Playlist 1");
            });
        });

        egui::SidePanel::right("right panel").exact_width(250.0).show(ctx, |ui| {
            ui.label("Song Queue:");
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            if self.visualizer_active {

                if ui.button("EXIT").clicked() {
                    self.visualizer_active =  false
                }
                if ui.button(if self.visualizer_effect_lines {"LINES OFF"} else {"LINES ON"}).clicked() {
                    if self.visualizer_effect_lines {
                        self.visualizer_effect_lines = false
                    }
                    else {
                        self.visualizer_effect_lines = true
                    }
                }

                let selected_size = format!("{}", self.buffer_size);
                let selected_style = if self.visualizer_style == 0 {
                    "Waveform"
                }
                else {
                    "Lissajous"
                };

                // Declare the available options for the combo box
                let buffer_options = ["128","256", "512", "1024", "2048"];
                let style_options = ["Waveform", "Lissajous"];


                egui::ComboBox::from_label("BUFFER SIZE")
                    .selected_text(&selected_size)
                    .show_ui(ui, |ui| {
                        for &option in buffer_options.iter() {
                            if ui.selectable_label(self.buffer_size == option.parse::<usize>().unwrap(), option).clicked() {
                                self.buffer_size = option.parse::<usize>().unwrap();
                            }
                        }
                    });
                egui::ComboBox::from_label("STYLE")
                    .selected_text(selected_style)
                    .show_ui(ui, |ui| {
                        for &option1 in style_options.iter() {
                            let is_selected = match option1 {
                                "Waveform" => self.visualizer_style == 0,
                                "Lissajous" => self.visualizer_style == 1,
                                _ => false,
                            };

                            if ui.selectable_label(is_selected, option1).clicked() {
                                if option1 == "Waveform" {
                                    self.visualizer_style = 0;
                                } else if option1 == "Lissajous" {
                                    self.visualizer_style = 1;
                                }
                            }
                        }
                    });



                if !self.audio_player.sink.empty() {

                    ui.ctx().request_repaint();
                    let color = Color32::LIGHT_BLUE;
                    let idx = self.audio_player.sample_index.load(Ordering::Relaxed);
                    let desired_size = ui.available_width() * vec2(0.99, 0.6);
                    let (_id, rect) = ui.allocate_space(desired_size);

                    let mut shapes = vec![];
                    let n = self.buffer_size;  // visualize the last 500 pairs of samples.
                    let samples_to_fetch = n;

                    if self.visualizer_style == 0 {

                        let samples_to_visualize = &self.audio_player.samples_for_viz[idx.saturating_sub(2 * n)..=idx];
                        let middle_x = rect.center().x;


                        // Each sample will be spaced by a certain amount on the X-axis.
                        let spacing_x = rect.width() / n as f32;

                        for i in (0..samples_to_visualize.len()).step_by(2) {
                            if i + 1 >= samples_to_visualize.len() {
                                break;
                            }

                            // Average the two samples
                            let average_sample = (samples_to_visualize[i] + samples_to_visualize[i + 1]) / 2.0;

                            // Calculate the x-coordinate offset from the middle
                            let offset_x = (i/2) as f32 * spacing_x - (samples_to_visualize.len() as f32 * spacing_x / 4.0);  // divide by 4 because we're considering two samples as one

                            let start_x = middle_x + offset_x;
                            let end_x = start_x - spacing_x;

                            let start_point = pos2(start_x, rect.center().y + average_sample * rect.height() / 2.0);
                            let end_point = pos2(end_x, rect.center().y + if i > 0 { (samples_to_visualize[i - 2] + samples_to_visualize[i - 1]) / 2.0 * rect.height() / 2.0 } else { 0.0 });

                            shapes.push(epaint::Shape::line(
                                [start_point, end_point].to_vec(),
                                Stroke::new(2.5, color)
                            ));
                        }
                    }

                    if self.visualizer_style == 1 {
                    // We'll use a step of 2 since each point needs two samples.
                        let samples_to_visualize = &self.audio_player.samples_for_viz[idx.saturating_sub(samples_to_fetch)..=idx];
                        for i in (0..samples_to_visualize.len()).step_by(2) {
                            if i + 1 >= samples_to_visualize.len() {
                                break;  // If there's no pair for the last sample, break out of the loop.
                            }

                            let x_sample = samples_to_visualize[i];
                            let y_sample = samples_to_visualize[i + 1];

                            let point_x = rect.center().x + x_sample * rect.width() / 2.0;
                            let point_y = rect.center().y - y_sample * rect.height() / 2.0;

                            let point = pos2(point_x, point_y);
                            shapes.push(epaint::Shape::circle_filled(point, 1.0, color));

                            if self.visualizer_effect_lines {
                                if i > 1 {
                                    let prev_x_sample = samples_to_visualize[i - 2];
                                    let prev_y_sample = samples_to_visualize[i - 1];

                                    let prev_point_x = rect.center().x + prev_x_sample * rect.width() / 2.0;
                                    let prev_point_y = rect.center().y - prev_y_sample * rect.height() / 2.0;

                                    let prev_point = pos2(prev_point_x, prev_point_y);

                                    shapes.push(epaint::Shape::line(
                                        [prev_point, point].to_vec(),
                                        Stroke::new(1.5, color)
                                    ));
                                }
                            }
                        }
                    }
                    ui.painter().extend(shapes);
                }


            }
            else {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    egui::Grid::new("some_unique_id")
                        .striped(true)
                        .max_col_width(200.0)
                        .min_col_width(200.0)
                        .show(ui, |ui| {
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
                                } else {
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
            }
        });
    }
}

