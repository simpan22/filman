use std::{io::Write, process::Command};

use crate::app_state::AppState;
use crate::path::directory_children;

pub fn delete_cursor(app_state: &mut AppState) -> anyhow::Result<()> {
    let mut tmp_file = tempfile::NamedTempFile::new()?;
    for file in app_state.selected_paths() {
        let filename = file.to_str().unwrap();
        tmp_file.write_all(format!("rm -rf {}", filename).as_bytes())?;
    }

    if app_state.selected_paths().is_empty() {
        tmp_file.write_all(
            format!("rm -rf \"{}\"", app_state.cursor_path().to_str().unwrap()).as_bytes(),
        )?;
    }

    tmp_file.flush()?;

    // ncurses::endwin();
    // Command::new("nvim")
    //     .arg(tmp_file.path().to_str().unwrap())
    //     .status()?;

    Command::new("bash")
        .arg(tmp_file.path().to_str().unwrap())
        .status()?;

    ncurses::refresh();

    app_state.refresh()?;
    Ok(())
}

pub fn yank(app_state: &mut AppState, cut: bool) -> anyhow::Result<()> {
    // List of selected entries or cursor if none selected
    let selected = match app_state.selected_map.get(&app_state.current_path) {
        // Nothing is selected in this dir
        None => vec![app_state
            .current_path
            .join(&app_state.child_files[app_state.cursor])],

        // Return selected entries
        Some(x) => x.to_vec(),
    };

    app_state.clipboard = selected.into_iter().map(|x| (x, cut)).collect();
    Ok(())
}

pub fn paste(app_state: &mut AppState) -> anyhow::Result<()> {
    let children = directory_children(&app_state.current_path, true)?;
    let children_names: Vec<_> = children
        .into_iter()
        .map(|x| x.file_name().unwrap().to_str().unwrap().to_string())
        .collect();
    let mut tmp_file = tempfile::NamedTempFile::new()?;

    for (file, del) in app_state.clipboard.iter() {
        let filename = file.file_name().unwrap().to_str().unwrap();

        let mut filename = filename.to_string();
        while children_names.contains(&filename) {
            filename = format!("_{}", filename);
        }

        let dest_file = app_state.current_path.join(filename);

        tmp_file.write_all(
            format!(
                "cp -r \"{}\" \"{}\"",
                file.to_str().unwrap(),
                dest_file.to_str().unwrap()
            )
            .as_bytes(),
        )?;
        tmp_file.write_all(&[b'\n'])?;

        if *del {
            tmp_file.write_all(format!("rm -rf \"{}\"", file.to_str().unwrap()).as_bytes())?;
        }
    }
    tmp_file.flush()?;

    // ncurses::endwin();
    // Command::new("nvim")
    //     .arg(tmp_file.path().to_str().unwrap())
    //     .status()?;

    Command::new("bash")
        .arg(tmp_file.path().to_str().unwrap())
        .status()?;

    ncurses::refresh();

    app_state
        .cursor_map
        .insert(app_state.current_path.clone(), app_state.cursor);
    app_state.child_files =
        directory_children(&app_state.current_path, app_state.show_hidden).unwrap();
    app_state.cursor = *app_state
        .cursor_map
        .get(&app_state.current_path)
        .unwrap_or(&0);
    Ok(())
}
