
use super::file_handling::file_handling::*;
use super::file_handling::audio_player::*;
use std::collections::VecDeque;
use std::fs::File;
use std::path::Path;
use std::sync::atomic::Ordering;

use egui_modal;
use egui::*;
use egui::Color32;
use egui::WidgetType::ComboBox;


pub struct TemplateApp {
    music_library: Vec<MusicFile>,
    audio_player: AudioHandler,
    seek: f32,
    fp: String,
    visualizer_parameters: VisualizerParameters,
}

impl Default for TemplateApp {
    fn default() -> Self {


        Self {
            music_library: get_library().unwrap(),
            audio_player: AudioHandler::new(),
            seek: 1.0,
            fp: "".to_owned(),
            visualizer_parameters: VisualizerParameters::new(),
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
                });
                ui.menu_button("View",|ui| {
                    if ui.button("Visualizer").clicked() {
                        self.visualizer_parameters.is_active = true;
                    }
                });
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
                    .fill(Color32::LIGHT_BLUE)
                    .desired_width(300.0)
                    );
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
            if self.visualizer_parameters.is_active {

                if ui.button("EXIT").clicked() {
                    self.visualizer_parameters.is_active =  false
                }

                if self.visualizer_parameters.style == 1 {
                    if ui.button(if self.visualizer_parameters.lines_active { "LINES OFF" } else { "LINES ON" }).clicked() {
                        if self.visualizer_parameters.lines_active {
                            self.visualizer_parameters.lines_active = false
                        } else {
                            self.visualizer_parameters.lines_active = true
                        }
                    }
                }

                let selected_size = format!("{}", self.visualizer_parameters.buffer_size);
                let selected_style = if self.visualizer_parameters.style == 0 {
                    "Waveform"
                }
                else if self.visualizer_parameters.style == 1 {
                    "Lissajous"
                }
                else {
                    "Stereo Spread"
                };

                // Declare the available options for the combo box
                let buffer_options = ["128","256", "512", "1024", "2048"];
                let style_options = ["Waveform", "Lissajous", "Stereo Spread"];


                egui::ComboBox::from_label("BUFFER SIZE")
                    .selected_text(&selected_size)
                    .show_ui(ui, |ui| {
                        for &option in buffer_options.iter() {
                            if ui.selectable_label(self.visualizer_parameters.buffer_size == option.parse::<usize>().unwrap(), option).clicked() {
                                self.visualizer_parameters.buffer_size = option.parse::<usize>().unwrap();
                            }
                        }
                    });
                egui::ComboBox::from_label("STYLE")
                    .selected_text(selected_style)
                    .show_ui(ui, |ui| {
                        for &option1 in style_options.iter() {
                            let is_selected = match option1 {
                                "Waveform" => self.visualizer_parameters.style == 0,
                                "Lissajous" => self.visualizer_parameters.style == 1,
                                "Stereo Spread" => self.visualizer_parameters.style == 2,
                                _ => false,
                            };

                            if ui.selectable_label(is_selected, option1).clicked() {
                                if option1 == "Waveform" {
                                    self.visualizer_parameters.style = 0;
                                } else if option1 == "Lissajous" {
                                    self.visualizer_parameters.style = 1;
                                }
                                else if option1 == "Stereo Spread" {
                                    self.visualizer_parameters.style = 2;
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

                    let origin = rect.center();

// Set the start and end points for the horizontal and vertical lines to the edges of the drawing area
                    let x_axis = [
                        pos2(rect.left(), origin.y),
                        pos2(rect.right(), origin.y),
                    ];
                    let y_axis = [
                        pos2(origin.x, rect.top()),
                        pos2(origin.x, rect.bottom()),
                    ];

                    let origin_color = Color32::WHITE;  // or any other color you prefer for the origin

// Convert these lines into shapes and add them to the shapes vector
                    let mut origin_paint = vec![
                        epaint::Shape::line(x_axis.to_vec(), Stroke::new(0.5, origin_color)),
                        epaint::Shape::line(y_axis.to_vec(), Stroke::new(0.5, origin_color)),
                    ];


                    let mut shapes = vec![];
                    let n = self.visualizer_parameters.buffer_size;  // visualize the last 500 pairs of samples.
                    let samples_to_fetch = n;

                    if self.visualizer_parameters.style == 0 {

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

                    if self.visualizer_parameters.style == 1 {
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
                            shapes.push(epaint::Shape::circle_filled(point, 2.0, color));


                            if self.visualizer_parameters.lines_active {
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

                    if self.visualizer_parameters.style == 2 {
                        let samples_to_visualize = &self.audio_player.samples_for_viz[idx.saturating_sub(samples_to_fetch)..=idx];
                        let angle_rad = 45.0f32.to_radians();  // 45 degrees in radians

                        for i in (0..samples_to_visualize.len()).step_by(2) {
                            if i + 1 >= samples_to_visualize.len() {
                                break;
                            }

                            let mut x_sample = samples_to_visualize[i].abs();  // Convert negative to positive
                            let mut y_sample = samples_to_visualize[i + 1].abs();  // Convert negative to positive

                            // Apply rotation transformation
                            let rotated_x = x_sample * angle_rad.cos() - y_sample * angle_rad.sin();
                            let rotated_y = x_sample * angle_rad.sin() + y_sample * angle_rad.cos();

                            let point_x = rect.center().x + rotated_x * rect.width() / -2.0;
                            let point_y = (rect.max.y + rotated_y * rect.height() / -2.0 );

                            let point = pos2(point_x, point_y);
                            shapes.push(epaint::Shape::circle_filled(point, 0.7, color));

                        }
                    }
                    ui.painter().extend(shapes);
                    if self.visualizer_parameters.style == 1 {
                        ui.painter().extend(origin_paint)
                    }
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

pub struct VisualizerParameters {
    pub is_active: bool,
    pub lines_active: bool,
    pub style: i8,
    pub buffer_size: usize,
    pub playback_speed: f32,
}

impl VisualizerParameters {
    fn new() -> VisualizerParameters {
        VisualizerParameters {
            is_active: false,
            lines_active: false,
            style: 0,
            buffer_size: 512,
            playback_speed: 1.0,
        }
    }
}
