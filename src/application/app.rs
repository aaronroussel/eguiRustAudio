
use super::file_handling::file_handling::*;
use super::file_handling::audio_player::*;
use std::collections::VecDeque;
use std::fs::File;
use std::path::Path;
use std::rc::Rc;
use std::sync::atomic::Ordering;
use realfft::RealFftPlanner;
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
    song_queue: VecDeque<MusicFile>,
    playlists: Vec<MusicCollection>,
    current_collection: Vec<MusicFile>,
    current_song: String,
    playlist_state: usize,
    song_holder: Option<MusicFile>,
}

impl Default for TemplateApp {
    fn default() -> Self {


        Self {
            music_library: new_library(),
            audio_player: AudioHandler::new(),
            seek: 1.0,
            fp: "".to_owned(),
            visualizer_parameters: VisualizerParameters::new(),
            song_queue: VecDeque::new(),
            playlists: Vec::new(),
            current_collection: Vec::new(),
            current_song: String::new(),
            playlist_state: 0,
            song_holder: None,
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


        if self.audio_player.sink.empty() {
            if !self.song_queue.is_empty() {
                let song: MusicFile = self.song_queue.pop_front().unwrap();
                self.audio_player.load_file(&song.file_path);
            }
        }

        // 1345x775

        // _frame.set_window_size(vec2(1345.0, 775.0));

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
                if filepath_modal.button(ui, "Add to Library").clicked() {
                    let new_music_files = get_from_path(&self.fp);
                    for x in new_music_files {
                        self.music_library.push(x);
                    }
                    filepath_modal.close();
                    self.fp = "".to_owned();
                }        
            });
        });

        let playlist_modal = egui_modal::Modal::new(ctx, "playlist modal")
            .with_close_on_outside_click(true);
        playlist_modal.show(|ui| {
            playlist_modal.title(ui, "Create Playlist");
            playlist_modal.frame(ui, |ui| {
                let playlist_name = ui.add(TextEdit::singleline(&mut self.fp)
                    .hint_text("Enter Playlist Name"));
                if playlist_name.changed(){

                }
            });
            playlist_modal.buttons(ui, |ui| {
                if playlist_modal.button(ui, "Create").clicked() {
                    let playlist = MusicCollection::new(String::from(&self.fp), self.playlists.len() as i32);
                    self.playlists.push(playlist);
                    playlist_modal.close();
                    self.fp = "".to_owned();
                }
            })
        });

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui|{
                ui.menu_button("File", |ui|{
                    if ui.button("Add music file").clicked() {
                        filepath_modal.open();
                        ui.close_menu();
                    }
                    if ui.button("Add music folder").clicked() {
                       filepath_modal.open();
                        ui.close_menu();
                    }
                });
                ui.menu_button("Edit", |ui| {
                    if ui.button("Create new Playlist").clicked() {
                        playlist_modal.open();
                        ui.close_menu();
                    }
                });
                ui.menu_button("View",|ui| {
                    if ui.button("Visualizer").clicked() {
                        if self.visualizer_parameters.is_active == false {
                            self.visualizer_parameters.is_active = true;
                            ui.close_menu();
                        }
                        else {
                            self.visualizer_parameters.is_active = false;
                            ui.close_menu();
                        }
                    }
                    println!("{}", self.visualizer_parameters.is_active);
                });
            });
            ui.style_mut().spacing.slider_width = 100.0;
            ui.vertical_centered(|ui| {
                if !self.audio_player.sink.empty() {
                    ui.label(format!("Now playing: {}", self.current_song));
                }
                if self.audio_player.sink.empty() {
                    if ui.button("PLAY").clicked() {
                        if !self.song_queue.is_empty() {
                            let song = self.song_queue.pop_front().unwrap();
                            self.current_song = song.name.to_string();
                            self.audio_player.load_file(&song.file_path);
                        }
                        // nothing
                    }
                }
                else {
                    ui.vertical_centered(|ui| {
                        if self.audio_player.sink.is_paused() {
                            if ui.button("PLAY").clicked() {
                                self.audio_player.resume_playback();
                            }
                        } else {
                            if ui.button("PAUSE").clicked() {
                                self.audio_player.pause_playback();
                            }
                        }
                        if !self.song_queue.is_empty() {
                            if ui.button("Next").clicked() {
                                self.audio_player.stop_playback();

                                // Check if there are songs in the queue
                                if let Some(next_song) = self.song_queue.pop_front() {
                                    // Load and play the next song
                                    self.current_song = next_song.name.to_string();
                                    self.audio_player.load_file(next_song.file_path.as_path());
                                    // Add this line
                                }
                            }
                        }
                    });
                }

                if ui.add(
                        Slider::new(&mut self.seek, 0.0..=1.0)
                            .text("Volume")
                            .show_value(false)
                            .trailing_fill(true),
                    )
                    .dragged()
                {
                    self.audio_player.sink.set_volume(self.seek);
                };



                let usize_val = self.audio_player.sample_index.load(Ordering::Relaxed);


                let desired_size = ui.available_width() * vec2(0.2, 0.05);
                let (_id, rect) = ui.allocate_space(desired_size);
                let playback_position = usize_val as f32 / self.audio_player.duration as f32;
                let start_point = rect.center() - vec2(300.0, 0.0);

                let playback_point =
                    if playback_position > 0.0 {start_point + vec2(playback_position * 600.0, 0.0)}
                    else {start_point};

                    start_point + vec2(playback_position * 600.0, 0.0);
                let mut shapes = vec![];
                shapes.push(epaint::Shape::line(
                    [start_point, start_point + vec2(600.0, 0.0)].to_vec(),
                    Stroke::new(2.5, egui::Color32::BLACK)
                ));
                shapes.push(epaint::Shape::circle_filled(playback_point, 5.0, egui::Color32::LIGHT_BLUE));
                shapes.push(epaint::Shape::line(
                    [start_point, playback_point].to_vec(),
                    Stroke::new(2.5, egui::Color32::LIGHT_BLUE)
                ));

                ui.painter().extend(shapes);
            })
        });


        egui::SidePanel::left("left panel").exact_width(250.0).show(ctx, |ui| {
            if ui.add(Label::new("Library").sense(Sense::click())).clicked() {
                self.playlist_state = 0;
            };
            
            egui::CollapsingHeader::new("Playlists").show(ui, |ui|{
                let mut i: usize = 1;
                for x in &self.playlists {
                   if ui.add(Label::new(&x.name).sense(Sense::click())).clicked() {
                       self.playlist_state = (x.index + 1) as usize
                   }
                }
            });
        });

        egui::SidePanel::right("right panel").exact_width(250.0).show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                egui::Grid::new("some_unique_id")
                    .striped(true)
                    .max_col_width(250.0)
                    .min_col_width(250.0)
                    .min_row_height(20.0)
                    .show(ui, |ui| {
                        ui.label("Song Queue:");
                        ui.end_row();
                        for z in &self.song_queue {
                            if &z.title == "" {
                                ui.label(&z.name);
                            }
                            else {
                                ui.label(&z.title);
                            }
                            ui.end_row();
                        }
                    });
            });

        });

        egui::CentralPanel::default().show(ctx, |ui| {
            if self.visualizer_parameters.is_active {
                if ui.add(Label::new("EXIT").sense(Sense::click())).clicked() {
                    self.visualizer_parameters.is_active = false;
                }
                let selected_size = format!("{}", self.visualizer_parameters.buffer_size);
                let selected_style = if self.visualizer_parameters.style == 0 {
                    "Waveform"
                }
                else if self.visualizer_parameters.style == 1 {
                    "Lissajous"
                }
                else if self.visualizer_parameters.style == 2 {
                    "Stereo Spread"
                }
                else {
                    "EQ"
                };

                let style_options = ["Waveform", "Lissajous", "Stereo Spread", "EQ"];

                ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {

                    egui::ComboBox::from_label("STYLE")
                        .selected_text(selected_style)
                        .show_ui(ui, |ui| {
                            for &option1 in style_options.iter() {
                                let is_selected = match option1 {
                                    "Waveform" => self.visualizer_parameters.style == 0,
                                    "Lissajous" => self.visualizer_parameters.style == 1,
                                    "Stereo Spread" => self.visualizer_parameters.style == 2,
                                    "EQ" => self.visualizer_parameters.style == 4,
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
                                    else if option1 == "EQ" {
                                        self.visualizer_parameters.style = 4;
                                    }
                                }
                            }
                        });
                    if self.visualizer_parameters.style == 1 {
                        if ui.button(if self.visualizer_parameters.lines_active { "LINES OFF" } else { "LINES ON" }).clicked() {
                            if self.visualizer_parameters.lines_active {
                                self.visualizer_parameters.lines_active = false
                            } else {
                                self.visualizer_parameters.lines_active = true
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
                    let buf_size = 2048;
                    let samples_to_visualize = &self.audio_player.circular_buffer.lock().unwrap();

                    if self.visualizer_parameters.style == 0 {
                        if self.visualizer_parameters.buffer_size > 2048 {
                            self.visualizer_parameters.buffer_size = 512;
                        }



                        let middle_x = rect.center().x;


                        // Each sample will be spaced by a certain amount on the X-axis.
                        let spacing_x = (rect.width() / buf_size as f32) * 2.0 ;
                        let mut previous_point = pos2(0.0, 0.0);

                        for i in (0..buf_size).step_by(2) {
                            if i + 1 >= buf_size {
                                break;
                            }

                            let sample1 = samples_to_visualize.get(i).unwrap_or(&0.0);
                            let sample2 = samples_to_visualize.get(i + 1).unwrap_or(&0.0);




                            // Average the two samples
                            let average_sample = (sample1 + sample2) / 2.0;

                            // Calculate the x-coordinate offset from the middle
                            let offset_x = (i/2) as f32 * spacing_x - (buf_size as f32 * spacing_x / 4.0);  // divide by 4 because we're considering two samples as one

                            let start_x = middle_x + offset_x;
                            let end_x = start_x - spacing_x;

                            let start_point = pos2(start_x, rect.center().y + average_sample * rect.height() / 2.0);
                            if i == 0 {
                                previous_point = start_point;
                            }
                            shapes.push(epaint::Shape::line(
                                [previous_point, start_point].to_vec(),
                                Stroke::new(1.25, color)
                            ));

                            previous_point = start_point;

                        }

                    }

                    if self.visualizer_parameters.style == 1 {
                        if self.visualizer_parameters.buffer_size > 2048 {
                            self.visualizer_parameters.buffer_size = 512;
                        }

                        let mut previous_point = pos2(0.0, 0.0);
                        for i in (0..buf_size).step_by(2) {
                            if i + 1 >= samples_to_visualize.len() {
                                break;  // If there's no pair for the last sample, break out of the loop.
                            }

                            let x_sample = samples_to_visualize.get(i).unwrap_or(&0.0);
                            let y_sample = samples_to_visualize.get(i + 1).unwrap_or(&0.0);

                            let point_x = rect.center().x + x_sample * rect.width() / 2.0;
                            let point_y = rect.center().y - y_sample * rect.height() / 2.0;

                            let point = pos2(point_x, point_y);
                            if i == 0 {
                                previous_point = point;
                            }


                            if self.visualizer_parameters.lines_active {
                                if i > 1 {

                                    shapes.push(epaint::Shape::line(
                                        [previous_point, point].to_vec(),
                                        Stroke::new(0.5, color)
                                    ));
                                }
                            }
                            else {
                                shapes.push(epaint::Shape::circle_filled(point, 0.75, color));
                            }
                            previous_point = point;
                        }
                    }

                    if self.visualizer_parameters.style == 2 {
                        if self.visualizer_parameters.buffer_size > 2048 {
                            self.visualizer_parameters.buffer_size = 512;
                        }
                        let angle_rad = 45.0f32.to_radians();  // 45 degrees in radians

                        for i in (0..samples_to_visualize.len()).step_by(2) {
                            if i + 1 >= samples_to_visualize.len() {
                                break;
                            }

                            let mut x_sample = samples_to_visualize.get(i).unwrap_or(&0.0).abs();  // Convert negative to positive
                            let mut y_sample = samples_to_visualize.get(i + 1).unwrap_or(&0.0).abs();  // Convert negative to positive

                            // Apply rotation transformation
                            let rotated_x = x_sample * angle_rad.cos() - y_sample * angle_rad.sin();
                            let rotated_y = x_sample * angle_rad.sin() + y_sample * angle_rad.cos();

                            let point_x = rect.center().x + rotated_x * rect.width() / -2.0;
                            let point_y = (rect.max.y + rotated_y * rect.height() / -2.0 ) - 75.0;

                            let point = pos2(point_x, point_y);
                            shapes.push(epaint::Shape::circle_filled(point, 0.7, color));

                        }
                    }

                    if self.visualizer_parameters.style == 4 {
                        let buffer_size = buf_size;
                            let mut samples = samples_to_visualize.to_vec();
                            let mut planner = RealFftPlanner::<f32>::new();
                            let r2c = planner.plan_fft_forward(buffer_size);
                            let mut outdata = r2c.make_output_vec();
                            if let Err(e) = r2c.process(&mut samples, &mut outdata) {
                                // handle the error, maybe log it or show a message to the user
                                eprintln!("FFT processing error: {}", e);
                                return;
                            }
                            let amplitude_spectrum: Vec<f32> = outdata.iter().map(|c| c.norm()).collect();
                            let start_x = 0.0;
                            let start_y = rect.center().y + 20.0;
                            let offset_x = ((rect.width() - 500.0) / 205.0) * 2.0;
                            println!("{}", amplitude_spectrum.len());

                            for mut i in 1..69 {
                                let mut average: f32 = 0.0;
                                let mut sum: f32 = 0.0;
                                // x values, adjusted to center the visualization
                                for x in 0..20 {
                                    sum += amplitude_spectrum[(i * 4) + x];
                                    average = sum / x as f32;
                                }
                                let mut x1: f32 = 0.0;
                                let mut  x2 = 0.0;
                                let mut y1 = 0.0;
                                let mut y2 = 0.0;

                                x1 = 275.0 + (i as f32 * offset_x) * 2.0;

                                y1 = rect.center().y + 100.0 + (average * -4.0) ;

                                // Points
                                let start_point = pos2(x1, rect.center().y + 100.0);
                                let end_point = pos2(x1, y1);

                                shapes.push(epaint::Shape::line(
                                    [start_point, end_point].to_vec(),
                                    Stroke::new(10.0, egui::Color32::GRAY)
                                ));
                            }

                            // ... [rest of your code for visualization]


                    }
                    ui.painter().extend(shapes);
                    if self.visualizer_parameters.style == 1 {
                        ui.painter().extend(origin_paint)
                    }
                }




            }
            else {
                let spacing = (ui.available_width() - 500.0) / 4.0;
                egui::ScrollArea::vertical().show(ui, |ui| {
                    egui::Grid::new("some_unique_id")
                        .striped(true)
                        .min_col_width(spacing)
                        .min_row_height(35.0)
                        .show(ui, |ui| {
                            ui.label("Title:");
                            ui.label("Artist:");
                            ui.label("Album:");
                            ui.label("Duration:");
                            ui.label("");
                            ui.end_row();
                            if self.playlist_state == 0 {

                                let playlistadd_modal = egui_modal::Modal::new(ctx, "playlist_add modal")
                                    .with_close_on_outside_click(true);
                                playlistadd_modal.show(|ui| {
                                    playlistadd_modal.title(ui, "Select Playlist");

                                    playlistadd_modal.frame(ui, |ui|{
                                        for mut x in &mut self.playlists {
                                            if ui.add(Label::new(&x.name).sense(Sense::click())).clicked() {
                                                let song = self.song_holder.clone();
                                                x.add_song(song.unwrap());
                                                playlistadd_modal.close();
                                            }
                                        }
                                    });
                                    playlistadd_modal.buttons(ui, |ui| {
                                        if playlistadd_modal.button(ui, "close").clicked() {
                                            playlistadd_modal.close();
                                        }
                                    });
                                });

                                for z in &self.music_library {
                                    if &z.title == "" {
                                        let response =
                                            ui.add(Label::new(&z.name).sense(Sense::click()));

                                        if response.double_clicked() {
                                            self.audio_player.stop_playback();
                                            println!("double click executed456");
                                            let file_path = &z.file_path;
                                            self.audio_player.load_file(file_path.as_path());
                                            self.current_song = String::from(&z.name);
                                            print!("abcc, {}", self.current_song);
                                        }

                                        response.context_menu(|ui| {
                                            if ui.button("Play File").clicked() {
                                                self.audio_player.stop_playback();
                                                println!("double click executed456");
                                                let file_path = &z.file_path;
                                                self.audio_player.load_file(file_path.as_path());
                                                ui.close_menu();
                                                self.current_song = String::from(&z.name);
                                                print!("abcc, {}", self.current_song);
                                            }

                                            if ui.button("Add to Playlist").clicked() {
                                                self.song_holder = Some(z.clone());
                                                playlistadd_modal.open();
                                                ui.close_menu();
                                            }

                                            if ui.button("Add to Queue").clicked() {
                                                self.song_queue.push_back(z.clone());
                                                ui.close_menu();
                                            }

                                            if ui.button("Add to beginning of Queue").clicked() {
                                                self.song_queue.push_front(z.clone());
                                                ui.close_menu();
                                            }
                                            if ui.button("Close Menu").clicked() {
                                                ui.close_menu();
                                            }
                                        });
                                    }
                                    else {
                                        let response =
                                            ui.add(Label::new(&z.title).sense(Sense::click()));

                                        if response.double_clicked() {
                                            self.audio_player.stop_playback();
                                            println!("double click executed456");
                                            let file_path = &z.file_path;
                                            self.audio_player.load_file(file_path.as_path());
                                            self.current_song = String::from(&z.title);
                                            print!("abcc, {}", self.current_song);
                                        }

                                        response.context_menu(|ui| {
                                            if ui.button("Play File").clicked() {
                                                self.audio_player.stop_playback();
                                                println!("double click executed123");
                                                let file_path = &z.file_path;
                                                self.audio_player.load_file(file_path.as_path());
                                                self.current_song = String::from(&z.title);
                                            }
                                            if ui.button("Add to Playlist").clicked() {
                                                self.song_holder = Some(z.clone());
                                                playlistadd_modal.open();
                                                ui.close_menu();
                                            }

                                            if ui.button("Add to Queue").clicked() {
                                                self.song_queue.push_back(z.clone());
                                            }
                                            if ui.button("Add to beginning of Queue").clicked() {
                                                self.song_queue.push_front(z.clone());
                                            }
                                        });
                                    }
                                    ui.label(&z.artist);
                                    ui.label(&z.album);
                                    ui.label(&z.duration.to_string());
                                    ui.end_row();
                                }
                            }
                            else {
                                let playlist = &self.playlists[self.playlist_state - 1];
                                for z in &playlist.collection {
                                    if &z.title == "" {
                                        let response =
                                            ui.add(Label::new(&z.name).sense(Sense::click()));

                                        if response.double_clicked() {
                                            self.audio_player.stop_playback();
                                            println!("double click executed456");
                                            let file_path = &z.file_path;
                                            self.audio_player.load_file(file_path.as_path());
                                            self.current_song = String::from(&z.name);
                                            print!("abcc, {}", self.current_song);
                                        }

                                        response.context_menu(|ui| {
                                            if ui.button("Play File").clicked() {
                                                self.audio_player.stop_playback();
                                                println!("double click executed456");
                                                let file_path = &z.file_path;
                                                self.audio_player.load_file(file_path.as_path());
                                                ui.close_menu();
                                                self.current_song = String::from(&z.name);
                                                print!("abcc, {}", self.current_song);
                                            }
                                            if ui.button("Add to Queue").clicked() {
                                                self.song_queue.push_back(z.clone());
                                                ui.close_menu();
                                            }

                                            if ui.button("Add to beginning of Queue").clicked() {
                                                self.song_queue.push_front(z.clone());
                                                ui.close_menu();
                                            }
                                            if ui.button("Close Menu").clicked() {
                                                ui.close_menu();
                                            }
                                        });
                                    } else {
                                        let response =
                                            ui.add(Label::new(&z.title).sense(Sense::click()));

                                        if response.double_clicked() {
                                            self.audio_player.stop_playback();
                                            println!("double click executed456");
                                            let file_path = &z.file_path;
                                            self.audio_player.load_file(file_path.as_path());
                                            self.current_song = String::from(&z.title);
                                            print!("abcc, {}", self.current_song);
                                        }

                                        response.context_menu(|ui| {
                                            if ui.button("Play File").clicked() {
                                                self.audio_player.stop_playback();
                                                println!("double click executed123");
                                                let file_path = &z.file_path;
                                                self.audio_player.load_file(file_path.as_path());
                                                self.current_song = String::from(&z.title);
                                            }
                                            if ui.button("Add to Queue").clicked() {
                                                self.song_queue.push_back(z.clone());
                                            }
                                            if ui.button("Add to beginning of Queue").clicked() {
                                                self.song_queue.push_front(z.clone());
                                            }
                                        });
                                    }
                                    ui.label(&z.artist);
                                    ui.label(&z.album);
                                    ui.label(&z.duration.to_string());
                                    ui.end_row();
                                }
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
