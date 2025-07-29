use std::path::PathBuf;

use egui::{mutex::Mutex, Color32};
use egui_dock::{DockArea, DockState, Style};
use egui_file_dialog::FileDialog;

use crate::engine::{self, python::FRAME_IMAGE, redengine::{GameState, Project}, ui::CentralTabViewer};







/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state


pub struct TemplateApp { // Attempts to load this state if possible
    // #[serde(skip)] // This how you opt-out of serialization of a field

    #[serde(skip)]
    file_dialog: FileDialog,

    #[serde(skip)]
    central_dock_state: DockState<crate::engine::ui::CentralPanelTab>,
    #[serde(skip)]
    side_dock_state: DockState<crate::engine::ui::SidePanelTab>,
    // ------------
    code_editor_content: String,
    // ------------
    last_opened_file: Option<PathBuf>,
    project: Project,
    // ------------
    resource_search_term: String,
    #[serde(skip)]
    viewport_texture: Option<egui::TextureHandle>,
    #[serde(skip)]
    game_state: GameState
}   

impl Default for TemplateApp { // Fallback State
    fn default() -> Self {
        Self {
            file_dialog: FileDialog::new(), 
            // ------------
            central_dock_state: DockState::new(vec![engine::ui::CentralPanelTab::Viewport, engine::ui::CentralPanelTab::Scripting]),
            side_dock_state: DockState::new(vec![engine::ui::SidePanelTab::FileExplorer]),
            // ------------
            code_editor_content: "# Your code".into(),
            // ------------
            last_opened_file: None,
            project: Project::new(),
            // ------------
            resource_search_term: "".to_owned(),
            // ------------
            viewport_texture: None,
            game_state: GameState { running: false, size: [1280, 720]}
        }
    }
}

impl TemplateApp {
    // App Constructor. Attempts to load program state from previous runs. 
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Init stuff :---
        egui_extras::install_image_loaders(&cc.egui_ctx);
        let _ = FRAME_IMAGE.set(Mutex::new(None));

        // Enable phosphor fonts for icons.
        let mut fonts = egui::FontDefinitions::default();
        egui_phosphor::add_to_fonts(&mut fonts, egui_phosphor::Variant::Regular);
        cc.egui_ctx.set_fonts(fonts);


        // Load state or revert to default.
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
        self.project.loaded = false;
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Gets the color of seperators, for frames and such.
        let separator_color = ctx.style().visuals.widgets.noninteractive.bg_stroke.color;

        
        // Use the file dialog to update the project path and window title.
        if let Some(proj_dir) = self.file_dialog.take_picked() {
            self.project.project_path = Some(proj_dir.clone());
            let dir_str = proj_dir.display().to_string();
            
            
            ctx.send_viewport_cmd(egui::ViewportCommand::Title(dir_str.clone()));
        
            // if !self.project.loaded {
            //     self.project = Project::new();
            // } 
        
            self.project.root_item = Project::load_files(proj_dir.clone());
            self.project.loaded = true;

            println!("{}", self.project)

        } else { // Use the last opened project and update the window title.
            if !self.project.loaded {
                if let Some(proj_dir) = &self.project.project_path {
                    
                    self.project.root_item = Project::load_files(proj_dir.clone());
                    self.project.loaded = true;

                    let dir_str = proj_dir.display().to_string();
                    ctx.send_viewport_cmd(egui::ViewportCommand::Title(dir_str));
                
                } 
            }
        }

        egui::TopBottomPanel::top("Program Menu")
            .show(ctx, |ui| {
                egui::MenuBar::new().ui(ui, |ui| {
                    ui.style_mut().visuals.widgets.active.corner_radius = egui::CornerRadius::same(0);
                    ui.style_mut().visuals.widgets.inactive.corner_radius = egui::CornerRadius::same(0);
                    ui.style_mut().visuals.widgets.hovered.corner_radius = egui::CornerRadius::same(0);
                    ui.style_mut().visuals.widgets.open.corner_radius = egui::CornerRadius::same(0);



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

                let mut viewer = engine::ui::SideTabViewer {
                    project: &mut self.project,
                    code_editor_content: &mut self.code_editor_content,
                    egui_ctx: ctx,
                }; let mut style = Style::from_egui(ui.style());
                style.tab_bar.bg_fill = Color32::from_gray(22);   

                DockArea::new(&mut self.side_dock_state)
                    .style(style)
                    .show_close_buttons(false)
                    .show_leaf_collapse_buttons(false)
                    .show_leaf_close_all_buttons(false)
                    .show_inside(ui, &mut viewer);
            });

        
        egui::CentralPanel::default()
            .show(ctx, |ui| {
                if self.game_state.running {  // Handle, Assign and Update the viewport texture
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
                }

                let mut viewer = CentralTabViewer {
                    // code_editor: &mut self.code_editor,
                    viewport_texture: &mut self.viewport_texture,
                    code_editor_content: &mut self.code_editor_content,
                    game_state: &mut self.game_state,
                    egui_ctx: ctx,
                }; let mut style = Style::from_egui(ui.style());
                style.tab_bar.bg_fill = Color32::from_gray(22);   

                DockArea::new(&mut self.central_dock_state)
                    .style(style)
                    .show_close_buttons(false)
                    .show_leaf_collapse_buttons(false)
                    .show_leaf_close_all_buttons(false)
                    .show_inside(ui, &mut viewer);
    
        });

        // Misc
        self.file_dialog.update(ctx);

    }
}