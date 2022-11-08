use crate::directory_children;
use std::collections::HashMap;
use std::path::PathBuf;

pub struct Screen {
    pub width: i32,
    pub height: i32,
}

pub struct AppState {
    pub screen: Screen,
    pub cursor_map: HashMap<PathBuf, usize>,
    pub cursor: usize,
    pub show_hidden: bool,
    pub current_path: PathBuf,
    pub child_files: Vec<PathBuf>,
    pub selected_map: HashMap<PathBuf, Vec<PathBuf>>,
    pub clipboard: Vec<(PathBuf, bool)>,
}

impl AppState {
    pub fn new(start_dir: PathBuf) -> Self {
        AppState {
            screen: Screen {
                height: 0,
                width: 0,
            },

            /// Cursor position stored for each previously visited directory
            cursor_map: HashMap::new(),

            /// Cursor position index in this directory
            cursor: 0,

            show_hidden: false,

            /// Path of the current working dir
            current_path: start_dir.clone(),

            /// List of paths of child directories, kept up to date in navigation functions
            child_files: directory_children(&start_dir, false).unwrap(),

            /// Selected files for each previously visited directory
            selected_map: HashMap::new(),

            /// Yanked or cut files stored as a tuple of path and bool (indicating weather it is cut
            /// or yanked)
            clipboard: vec![],
        }
    }

    /// Get the (full) path of file the cursor is on
    pub fn cursor_path(&self) -> PathBuf {
        self.current_path.join(&self.child_files[self.cursor])
    }

    pub fn selected_paths(&self) -> Vec<PathBuf> {
        match self.selected_map.get(&self.current_path) {
            Some(v) => v.to_vec(),
            None => vec![],
        }
    }

    pub fn refresh(&mut self) -> anyhow::Result<()> {
        self.child_files = directory_children(&self.current_path, self.show_hidden)?;
        self.cursor = *self.cursor_map.get(&self.current_path).unwrap_or(&0);
        Ok(())
    }

    pub fn parent_directory(&self) -> Option<PathBuf> {
        self.current_path.parent().map(|x| x.into())
    }
}
