mod app_state;
mod clipboard;
mod command;
mod navigation;
mod path;

use anyhow::Result;
use app_state::AppState;
use clipboard::{delete_cursor, paste, yank};
use command::{execute_command, Command};
use navigation::{navigate, toggle_hidden, toggle_select, Direction};
use ncurses::wprintw;
use path::directory_children;
use std::env;

// TODO: Rename files and directories
// TODO: Remove files
//
// TODO: Preview window with children of the selected directory
// TODO: Cusomizable key bindings from config file
// TODO: Backspace in commandwindow

fn create_main_window(w: i32, h: i32, split_x: i32) -> *mut i8 {
    let window = ncurses::newwin(h - 1, w - split_x, 0, split_x);
    ncurses::wmove(window, 1, 1);
    window
}

fn create_parent_window(_w: i32, h: i32, split_x: i32) -> *mut i8 {
    let window = ncurses::newwin(h - 1, split_x, 0, 0);
    ncurses::wmove(window, 1, 1);
    window
}

fn create_command_window(w: i32, h: i32) -> *mut i8 {
    let window = ncurses::newwin(1, w, h - 1, 0);
    ncurses::wmove(window, 1, 1);
    window
}

fn clear_window(window: *mut i8) {
    ncurses::wclear(window);
    ncurses::wmove(window, 1, 1);
}

fn refresh_window(window: *mut i8, border: bool) {
    if border {
        ncurses::box_(window, 0, 0);
    }
    ncurses::wrefresh(window);
    ncurses::wmove(window, 1, 1);
}

fn init_ncurses() {
    ncurses::initscr();
    ncurses::raw();

    // ncurses::noecho();
    ncurses::cbreak();
    ncurses::curs_set(ncurses::CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    ncurses::start_color();
    ncurses::refresh();

    // Directories blue
    ncurses::init_pair(1, ncurses::COLOR_BLUE, ncurses::COLOR_BLACK);
}

fn main() -> Result<()> {
    let mut app_state = AppState::new(env::current_dir()?);

    init_ncurses();
    ncurses::getmaxyx(
        ncurses::stdscr(),
        &mut app_state.screen.height,
        &mut app_state.screen.width,
    );

    let split = 25;
    let main_window = create_main_window(app_state.screen.width, app_state.screen.height, split);
    let parent_window =
        create_parent_window(app_state.screen.width, app_state.screen.height, split);
    let command_window = create_command_window(app_state.screen.width, app_state.screen.height);

    let mut key = 0;

    loop {
        clear_window(main_window);
        clear_window(parent_window);
        clear_window(command_window);

        match char::from_u32(key).unwrap() {
            'q' => break,
            'j' => {
                navigate(Direction::Down, &mut app_state)?;
            }
            'k' => {
                navigate(Direction::Up, &mut app_state)?;
            }
            'l' | '\n' => {
                navigate(Direction::In, &mut app_state)?;
            }
            'h' => {
                navigate(Direction::Out, &mut app_state)?;
            }
            '.' => {
                toggle_hidden(&mut app_state)?;
            }
            'd' => {
                yank(&mut app_state, true)?;
            }
            'y' => {
                yank(&mut app_state, false)?;
            }
            'p' => {
                paste(&mut app_state)?;
            }
            ' ' => {
                toggle_select(&mut app_state)?;
            }
            'D' => {
                delete_cursor(&mut app_state)?;
            }
            'A' => {
                ncurses::endwin();
                todo!("Rename from end")
            }
            'i' => {
                ncurses::endwin();
                todo!("Rename")
            }
            ':' => {
                let mut command_str = "".to_string();
                ncurses::wgetstr(command_window, &mut command_str);
                let command: anyhow::Result<Command> = command_str.as_str().try_into();
                if let Ok(command) = command {
                    execute_command(command, &mut app_state)?;
                }
            }
            _ => {}
        }

        let parent_files = directory_children(
            &app_state.parent_directory().unwrap_or_else(|| "/".into()),
            app_state.show_hidden,
        )?;

        for (_idx, child) in parent_files.iter().enumerate() {
            // Cursor
            if child == &app_state.current_path {
                ncurses::wattron(parent_window, ncurses::A_STANDOUT());
            }

            // Directories
            if child.is_dir() {
                ncurses::wattron(parent_window, ncurses::COLOR_PAIR(1));
            }

            wprintw(
                parent_window,
                &format!("{}\n ", child.file_name().unwrap().to_str().unwrap()),
            );
            ncurses::wattroff(parent_window, ncurses::A_BOLD());
            ncurses::wattroff(parent_window, ncurses::COLOR_PAIR(1));
            ncurses::wattroff(parent_window, ncurses::A_STANDOUT());
        }

        for (idx, child) in app_state.child_files.iter().enumerate() {
            let child_path = app_state.current_path.join(child);

            // Cursor
            if idx == app_state.cursor {
                ncurses::wattron(main_window, ncurses::A_STANDOUT());
            }

            // Directories
            if child_path.is_dir() {
                ncurses::wattron(main_window, ncurses::COLOR_PAIR(1));
            }

            // Selected
            if let Some(selected) = app_state.selected_map.get(&app_state.current_path) {
                if selected.contains(&child_path) {
                    ncurses::wattron(main_window, ncurses::A_BOLD());
                }
            }

            wprintw(
                main_window,
                &format!("{}\n ", child.file_name().unwrap().to_str().unwrap()),
            );
            ncurses::wattroff(main_window, ncurses::A_BOLD());
            ncurses::wattroff(main_window, ncurses::COLOR_PAIR(1));
            ncurses::wattroff(main_window, ncurses::A_STANDOUT());
        }

        ncurses::wattroff(main_window, ncurses::A_STANDOUT());
        ncurses::wattroff(main_window, ncurses::A_BOLD());
        ncurses::wattroff(main_window, ncurses::COLOR_PAIR(1));

        // TODO: Why does commenting this out work?
        ncurses::refresh();

        refresh_window(main_window, true);
        refresh_window(parent_window, true);
        refresh_window(command_window, false);

        key = ncurses::getch() as u32;
    }

    ncurses::endwin();
    Ok(())
}
