use core::fmt;
use std::{fs, path::PathBuf};

use egui::Context;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub enum FileExplorerItemType {
    Folder,
    File,
}

impl fmt::Display for FileExplorerItemType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FileExplorerItemType::Folder => write!(f, "Folder"),
            FileExplorerItemType::File => write!(f, "File"),
        }
    }
}


#[derive(Serialize, Deserialize)]
pub struct FileExplorerItem {
    pub path: PathBuf,
    pub name: String,
    pub item_type: FileExplorerItemType,
    pub extension: Option<String>,
    pub children: Vec<FileExplorerItem>,
}

impl fmt::Display for FileExplorerItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Path: {}, Name: {}, Item Type: {}",
            self.path.display().to_string(),
            self.path.file_name().unwrap().to_str().unwrap().to_string(),
            self.item_type,
        )
    }
}

impl FileExplorerItem {
    pub fn resolve_file_icon(item: &FileExplorerItem) -> String {
        let file_extension = item.extension.as_ref().unwrap().as_str();
        
        match file_extension {
            "pdf" => egui_phosphor::regular::FILE_PDF,
            "c" => egui_phosphor::regular::FILE_C,
            "cs" => egui_phosphor::regular::FILE_C_SHARP,
            "cpp" => egui_phosphor::regular::FILE_CPP,
            "css" => egui_phosphor::regular::FILE_CSS,
            "csv" => egui_phosphor::regular::FILE_CSV,
            "doc" => egui_phosphor::regular::FILE_DOC,
            "html" => egui_phosphor::regular::FILE_HTML,
            "ini" => egui_phosphor::regular::FILE_INI,
            "jpg" => egui_phosphor::regular::FILE_JPG,
            "js" => egui_phosphor::regular::FILE_JS,
            "jsx" => egui_phosphor::regular::FILE_JSX,
            "md" => egui_phosphor::regular::FILE_MD,
            "png" => egui_phosphor::regular::FILE_PNG,
            "ppt" => egui_phosphor::regular::FILE_PPT,
            "py" => egui_phosphor::regular::FILE_PY,
            "rs" => egui_phosphor::regular::FILE_RS,
            "sql" => egui_phosphor::regular::FILE_SQL,
            "svg" => egui_phosphor::regular::FILE_SVG,
            "ts" => egui_phosphor::regular::FILE_TS,
            "tsx" => egui_phosphor::regular::FILE_TSX,
            "txt" => egui_phosphor::regular::FILE_TEXT,
            "vue" => egui_phosphor::regular::FILE_VUE,
            "xls" => egui_phosphor::regular::FILE_XLS,
            "zip" => egui_phosphor::regular::FILE_ZIP,
            _ => egui_phosphor::regular::FILE,
        }.to_owned()
    }


}






#[derive(Serialize, Deserialize)]
pub struct Project {
    pub project_path: Option<PathBuf>,
    pub root_item: Option<FileExplorerItem>,
    pub loaded: bool,
}

impl Default for Project {
    fn default() -> Self {
        Self {
            project_path: None,
            root_item: None,
            loaded: false,
        }
        
    }
}

impl fmt::Display for Project {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Path: {}, Root Item: {}, Loaded: {}",
            self.project_path.clone().expect("Project path is none").display().to_string(),
            "None",
            self.loaded.to_string(),
        )
    }
}

impl Project {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn load_files(path: PathBuf)  -> Option<FileExplorerItem> {
        let name = path.file_name()?.to_string_lossy().into_owned();
        let item_type = if path.is_dir() {
                FileExplorerItemType::Folder
            } else {
                FileExplorerItemType::File
            };

        let children = if matches!(item_type, FileExplorerItemType::Folder) {
            fs::read_dir(&path) 
                .ok()?
                .filter_map(|entry| entry.ok())
                .filter_map(|entry| Self::load_files(entry.path()))
                    .collect()
            } else {
                vec![]
            };

        Some(FileExplorerItem { 
            path: path.clone(), 
            name: name, 
            item_type: item_type, 
            extension: path.extension().map(|e| e.to_string_lossy().into_owned()),
            children: children,
        })
   }

   pub fn is_root(project: &Project, item: &FileExplorerItem) -> bool {
        item.path == project.root_item.as_ref().unwrap().path
   }

}









pub struct GameState {
    pub(crate) running: bool,
    pub(crate) size: [usize; 2],
}

pub fn launch_game(code_string: &str, game_state: &mut GameState) {
    crate::engine::python::run_code_threaded(&code_string);
    game_state.running = true;

}

pub fn close_game(game_state: &mut GameState, egui_ctx: &Context) {
    crate::engine::python::queue_python_instruction(|py| {
        let main = pyo3::prelude::PyModule::import(py, "__main__").unwrap();
        let game = main.getattr("game").unwrap();
        game.call_method0("quit").unwrap();
    });
    egui_ctx.forget_image("viewport_texture");
    game_state.running = false;
}