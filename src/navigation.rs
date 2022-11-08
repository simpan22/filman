use std::process::Command;

use crate::{app_state::AppState, path::directory_children};

pub enum Direction {
    Up,
    Down,
    In,
    Out,
}

pub fn navigate(dir: Direction, app_state: &mut AppState) -> Result<(), anyhow::Error> {
    match dir {
        Direction::Down => {
            app_state.cursor = if app_state.cursor == app_state.child_files.len() - 1 {
                app_state.cursor
            } else {
                app_state.cursor + 1
            };
        }
        Direction::Up => {
            app_state.cursor = if app_state.cursor == 0 {
                0
            } else {
                app_state.cursor - 1
            };
        }
        Direction::In => {
            if app_state.cursor_path().is_dir() {
                app_state
                    .cursor_map
                    .insert(app_state.current_path.clone(), app_state.cursor);
                app_state.current_path = app_state.cursor_path();
                app_state.refresh()?;
            } else {
                ncurses::endwin();
                Command::new("nvim")
                    .arg(app_state.cursor_path().to_str().unwrap())
                    .status()?;
                ncurses::refresh();
            }
        }
        Direction::Out => {
            let old_path = &app_state.current_path.clone();
            app_state
                .cursor_map
                .insert(app_state.current_path.clone(), app_state.cursor);
            app_state.current_path = match app_state.current_path.parent() {
                Some(current_path) => current_path.to_path_buf(),
                None => app_state.current_path.clone(),
            };

            app_state.refresh()?;

            app_state.cursor = app_state
                .child_files
                .iter()
                .position(|p| p == old_path)
                .unwrap();
        }
    }
    Ok(())
}

pub fn toggle_hidden(app_state: &mut AppState) -> Result<(), anyhow::Error> {
    app_state.show_hidden = !app_state.show_hidden;
    app_state.child_files = directory_children(&app_state.current_path, app_state.show_hidden)?;
    Ok(())
}

pub fn toggle_select(app_state: &mut AppState) -> anyhow::Result<()> {
    let cursor_path = app_state.cursor_path();

    match app_state.selected_map.get_mut(&app_state.current_path) {
        None => {
            app_state
                .selected_map
                .insert(app_state.current_path.clone(), vec![cursor_path]);
        }
        Some(selected) => {
            if selected.contains(&cursor_path) {
                // Deselect
                selected.retain(|x| x != &cursor_path);
            } else {
                // Select
                selected.push(cursor_path)
            }

            // Don't keep empty vectors in hashmap
            if selected.is_empty() {
                app_state.selected_map.remove(&app_state.current_path);
            }
        }
    }

    app_state.cursor = if app_state.cursor == app_state.child_files.len() - 1 {
        app_state.cursor
    } else {
        app_state.cursor + 1
    };
    Ok(())
}
