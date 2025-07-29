use core::f32;

use egui::{Context, Stroke, TextureHandle, Ui, WidgetText};
use egui_dock::TabViewer;

use crate::engine::redengine::{self, FileExplorerItem, GameState, Project};

#[derive(Debug, PartialEq)]
pub enum  CentralPanelTab {
    Viewport,
    Scripting,
}

pub struct CentralTabViewer<'a> {
    pub viewport_texture: &'a mut Option<TextureHandle>,
    pub code_editor_content: &'a mut String,
    pub game_state: &'a mut GameState,
    pub egui_ctx: &'a Context,
}

impl<'a> TabViewer for CentralTabViewer<'a> {
    type Tab =  CentralPanelTab;

    fn title(&mut self, tab: &mut Self::Tab) -> WidgetText {
        match tab {
            CentralPanelTab::Viewport => "Viewport".into(),
            CentralPanelTab::Scripting => "Scripting".into(),
        }
    }

    fn ui(&mut self, ui: &mut Ui, tab: &mut Self::Tab) {
        match tab {
            CentralPanelTab::Viewport => {
                ui.vertical_centered(|ui| {
                   let button_ico = |b| if b { format!("{}", egui_phosphor::regular::STOP)} else { format!("{}", egui_phosphor::regular::PLAY) };
                   
                   if ui.add( // Handles the launch/close button
                    egui::Button::new(egui::RichText::new(button_ico(self.game_state.running)))
                            .min_size(egui::vec2(75.0, 0.0))
                   ).clicked() { // Handles the button logic when clicked
                        if self.game_state.running {
                            redengine::close_game(&mut self.game_state, self.egui_ctx);
                        } else {
                            redengine::launch_game(&self.code_editor_content, &mut self.game_state);
                        }
                   }
                
                });
                ui.add(egui::Separator::default().grow(5.0));

                // Draw viewport texture
                ui.centered_and_justified(|ui|{
                    if self.game_state.running {
                        if let Some(tex) = &self.viewport_texture {
                            let original_size = self.game_state.size; // [width, height]
                            let available_size = ui.available_size();
                            let fit_size = crate::engine::helpers::fit_aspect(original_size, available_size);
                            
                            ui.image((tex.id(), fit_size));
                        }
                    } else {
                        ui.label("No image available");
                    }
                });  

            }
            CentralPanelTab::Scripting => {
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
                        egui::TextEdit::multiline(self.code_editor_content)
                            .font(egui::TextStyle::Monospace) // for cursor height
                            .code_editor()
                            .desired_rows(10)
                            .lock_focus(true)
                            .desired_width(f32::INFINITY)
                            .layouter(&mut layouter),
                    );
                });

            }
        }
    }

    fn is_closeable(&self, _tab: &Self::Tab) -> bool {
        false
    }
    
}



pub enum SidePanelTab {
    FileExplorer
}

pub struct SideTabViewer<'a> {
    pub project: &'a mut Project,
    pub code_editor_content: &'a mut String,
    pub egui_ctx: &'a Context,
}

impl<'a> TabViewer for SideTabViewer<'a> {
    type Tab =  SidePanelTab;

    fn title(&mut self, tab: &mut Self::Tab) -> WidgetText {
        match tab {
            SidePanelTab::FileExplorer => "File Explorer".into(),
        }
    }

    fn ui(&mut self, ui: &mut Ui, tab: &mut Self::Tab) {
        //let default_bg_color = ui.style().visuals.widgets.noninteractive.bg_fill;

        fn draw_file_explorer_child(ui: &mut Ui, root_itm: &FileExplorerItem, project: &Project) {
            let default_bg_color = ui.style().visuals.widgets.noninteractive.bg_fill;

            match &root_itm.item_type {
                redengine::FileExplorerItemType::Folder => {
                   let header_name = |b| if b { format!("{} Project", egui_phosphor::regular::FOLDER)} else { format!("{} {}", egui_phosphor::regular::FOLDER, &root_itm.name) };
                 
                    egui::CollapsingHeader::new( header_name(Project::is_root(project, root_itm)))
                        .default_open(false)
                        .show(ui, |ui| {
                            for child in &root_itm.children {
                                draw_file_explorer_child(ui, child, &project);
                            }
                        });
                }
                redengine::FileExplorerItemType::File => {
                    let base_color = ui.style().visuals.widgets.noninteractive.bg_fill;
                    //  // your base color
                    ui.style_mut().visuals.widgets.active.weak_bg_fill = base_color;
                    
                    let adjusted_color = if ui.style_mut().visuals.dark_mode {
                        base_color.gamma_multiply(1.5) // brighten in dark mode
                    } else {
                        base_color.linear_multiply(5.0) // darken in light mode
                    };

                    ui.style_mut().visuals.widgets.inactive.weak_bg_fill = base_color;
                    ui.style_mut().visuals.widgets.active.weak_bg_fill = base_color;

                    ui.style_mut().visuals.widgets.hovered.weak_bg_fill = adjusted_color;

                    let btn = egui::Button::new(format!("{} {}", FileExplorerItem::resolve_file_icon(root_itm), &root_itm.name))
                        .corner_radius(0)
                        .min_size(egui::vec2(ui.available_width(), 0.0))
                        .stroke(Stroke::new(0.0, default_bg_color));
    
                    let response = ui.add(btn);
                    if response.clicked() {
                        // let contents: String = fs::read_to_string(&root_itm.path).unwrap();
                        // *self.code_editor_content = contents;
                    }
                }
            }
        }
    
      
        match tab {
            SidePanelTab::FileExplorer => {

                ui.horizontal(|ui| {
                    if ui.add( // Plus Button (Creates files, folders etc...)
                        egui::Button::new(format!("{}", egui_phosphor::regular::PLUS))
                    ).clicked() {
                        // Code Here >>>>
                    };

                    ui.add(
                        egui::TextEdit::singleline(&mut String::from(""))
                            .hint_text("Search...")
                            .desired_width(f32::INFINITY)
                    );
                });

                ui.add(egui::Separator::default().grow(5.0));
               
                if self.project.loaded {
                    if let Some(root_itm) = &self.project.root_item {
                        draw_file_explorer_child(ui, &root_itm, self.project);
                    }      
                }
            }
        }
    }

    fn is_closeable(&self, _tab: &Self::Tab) -> bool {
        false
    }

   
    
}

