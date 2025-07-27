use std::{ fs, path::PathBuf, sync::OnceLock};


use pyo3::prelude::*;

use egui::{mutex::Mutex, Stroke};
use egui_file_dialog::FileDialog;

use crate::engine::{python::FRAME_IMAGE, redengine::CentralTab};







/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state


pub struct TemplateApp { // Attempts to load this state if possible
    // #[serde(skip)] // This how you opt-out of serialization of a field

    #[serde(skip)]
    file_dialog: FileDialog,

    #[serde(skip)] // This how you opt-out of serialization of a field
    current_tab: CentralTab,
    // ------------
    code_editor_content: String,
    // ------------
    last_opened_file: Option<PathBuf>,
    project_directory: Option<PathBuf>,
    // ------------
    resource_search_term: String,
    
    #[serde(skip)]
    viewport_texture: Option<egui::TextureHandle>,
}

impl Default for TemplateApp { // Fallback State
    fn default() -> Self {
        Self {
            file_dialog: FileDialog::new(), 
            // ------------
            current_tab: CentralTab::Viewport,
            // ------------
            code_editor_content: "# Your code".into(),
            // ------------
            last_opened_file: None,
            project_directory: None,
            // ------------
            resource_search_term: "".to_owned(),
            // ------------
            viewport_texture: None,
        }
    }
}

impl TemplateApp {
    // App Constructor. Attempts to load program state from previous runs. 
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {

        egui_extras::install_image_loaders(&cc.egui_ctx);
        FRAME_IMAGE.set(Mutex::new(None));

        if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()

        } else {
            Default::default()
        }
    }
}

impl eframe::App for TemplateApp {
    /// Called by the framework to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Gets the color of seperators, for frames and such.
        let separator_color = ctx.style().visuals.widgets.noninteractive.bg_stroke.color;
        let mut can_run_code = false;


        // Use the file dialog to update the project path and window title.
        if let Some(proj_dir) = self.file_dialog.take_picked() {
            self.project_directory = Some(proj_dir.to_path_buf());
            let dir_str = proj_dir.display().to_string();
            println!("Picked!");
            
            ctx.send_viewport_cmd(egui::ViewportCommand::Title(dir_str));

        } else { // Use the last opened project and update the window title.
            if let Some(proj_dir) = &self.project_directory {
                let dir_str = proj_dir.display().to_string();

                ctx.send_viewport_cmd(egui::ViewportCommand::Title(dir_str));
            } 
        }

        
        egui::TopBottomPanel::top("Program Menu")
            .show(ctx, |ui| {
                egui::MenuBar::new().ui(ui, |ui| {
                    ui.menu_button("Project", |ui| {
                        if ui.button("Open Project").clicked() {
                            // will Load a project from a directory. 
                            self.file_dialog.pick_directory();

                        }
                    });
                    ui.menu_button("Edit", |_ui| {
                        
                    });
                    ui.menu_button("Games", |_ui| {
                        
                    });
                    ui.menu_button("Tools", |_ui| {
                        
                    });
                    ui.menu_button("Help", |ui| {
                        egui::widgets::global_theme_preference_buttons(ui);
                    });
                });
            });

        egui::SidePanel::left("Resource Panel")
            .default_width(200.0) 
            .resizable(true)
            .width_range(100.0..=350.0)
            .show(ctx, | ui | {
                ui.add_space(6.0);

                egui::Frame::none()
                    .fill(egui::Color32::from_gray(32))
                    .show(ui, |ui| {
                        ui.vertical_centered( |ui: &mut egui::Ui|{
                            ui.heading("Resource Panel");
                        });
                });
                ui.separator();
            
                ui.add(
                    egui::TextEdit::singleline(&mut self.resource_search_term)
                    .hint_text("Search...")
                );

                egui::Frame::none()
                    .inner_margin(6.0)
                    .fill(egui::Color32::from_gray(32))
                    .stroke(Stroke::new(1.0, separator_color))
                    .show(ui, |ui| {
                            ui.collapsing("Project Files", |ui|{
                            ui.set_min_width(ui.available_width()); 
                            if let Some(proj_dir) = &self.project_directory {
                                let dir_str = proj_dir.display().to_string();
                                
                                let paths = fs::read_dir(dir_str).unwrap();
                                for path in paths {
                                    let path = path.unwrap().path();
                                    let fname = path.file_name().unwrap().to_os_string().into_string().unwrap();
                        
                                    if ui.button(fname).clicked() {
                                        let contents: String = fs::read_to_string(path).unwrap();
                                            
                                        self.code_editor_content = contents;
            
                                    }
                                    
                                    
                                }
                            }
                        });
                    });
            });

        
        egui::CentralPanel::default()
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    for (tab, label) in [
                        (CentralTab::Viewport, "Viewport"),
                        (CentralTab::Scripting, "Scripting"),
                    ] {
                        if ui.selectable_label(self.current_tab == tab, label).clicked() {
                            self.current_tab = tab;
                        }
                    }
                });
                ui.separator();
   


                match self.current_tab {
                CentralTab::Scripting => {

                    let mut theme =
                        egui_extras::syntax_highlighting::CodeTheme::from_memory(ui.ctx(), ui.style());
                    ui.collapsing("Theme", |ui| {
                        ui.group(|ui| {
                            theme.ui(ui);
                            theme.clone().store_in_memory(ui.ctx());
                        });
                    });

                    let mut layouter = |ui: &egui::Ui, buf: &dyn egui::TextBuffer, wrap_width: f32| {
                        let mut layout_job = egui_extras::syntax_highlighting::highlight(
                            ui.ctx(),
                            ui.style(),
                            &theme,
                            buf.as_str(),
                            "Python",
                        );
                        layout_job.wrap.max_width = wrap_width;
                        ui.fonts(|f| f.layout_job(layout_job))
                    };


                    egui::ScrollArea::vertical().show(ui, |ui| {
                        ui.add(
                            egui::TextEdit::multiline(&mut self.code_editor_content)
                                .font(egui::TextStyle::Monospace) // for cursor height
                                .code_editor()
                                .desired_rows(10)
                                .lock_focus(true)
                                .desired_width(f32::INFINITY)
                                .layouter(&mut layouter),
                        );
                    });










                },
                CentralTab::Viewport => {

                    egui::Frame::none()
                        .fill(egui::Color32::from_gray(16))
                        .inner_margin(6.0)
                        .corner_radius(4.0)
                        .stroke(Stroke::new(1.0, separator_color))
                        .show(ui, |ui| {
                            ui.vertical_centered(|ui|{
                                if ui.button("Play").clicked() {    
                                    crate::engine::python::run_code_threaded(&self.code_editor_content);
                                }
                            });
                    });


                                        
                   

                    if let Some(lock) = FRAME_IMAGE.get() {
                        let guard = lock.lock();
                        if let Some(image) = &*guard {
                            if let Some(vp_texture) = &mut self.viewport_texture {
                                // If it exists and is assigned
                                vp_texture.set(image.clone(), Default::default());

                            } else {
                                self.viewport_texture = Some(ctx.load_texture(
                                    "viewport_texture",
                                    image.clone(),
                                    Default::default(),
                                ));
                            }
                        }   
                    }

                    ui.centered_and_justified(|ui|{
                        if let Some(tex) = &self.viewport_texture {
                            ui.image((tex.id(), tex.size_vec2()));
                        } else {
                            ui.label("No image available");
                        }
                    });  




                    








                }}


                
        });

        // Misc
        self.file_dialog.update(ctx);

    }
}