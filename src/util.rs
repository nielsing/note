use std::env;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::PathBuf;

use colored::*;

use crate::note::*;

fn get_notes_file() -> PathBuf {
    if let Some(mut dir) = dirs::config_dir() {
        dir.push("notes.txt");
        return dir;
    }
    PathBuf::from("./notes.txt")
}

fn is_arg_set(short_name: &str, long_name: &str) -> bool {
    env::args().any(|arg| arg == short_name || arg == long_name)
}

pub fn read_to_notes(unwanted_ids: &[usize]) -> Vec<Note> {
    let path = get_notes_file();
    match OpenOptions::new().read(true).open(path.to_str().unwrap()) {
        Ok(mut file) => {
            let mut id = 0;
            let mut notes = String::new();
            if let Err(e) = file.read_to_string(&mut notes) {
                eprintln!("Unable to read file: {}", e);
            }
            notes.lines()
                .filter_map(|note| {
                    if note.len() <= 1 {
                        return None
                    }

                    id += 1;
                    if unwanted_ids.contains(&id) {
                        return None
                    }
                    let mut note = Note::from(note);
                    note.id = id;
                    Some(note)
                })
                .collect()
        }
        Err(e) => {
            eprintln!("Unable to open file: {}", e);
            vec![]
        },
    }
}

pub fn read_to_notes_str(unwanted_ids: &[usize]) -> Vec<String> {
    let path = get_notes_file();
    match OpenOptions::new().read(true).open(path.to_str().unwrap()) {
        Ok(mut file) => {
            let mut id = 0;
            let mut notes = String::new();
            if let Err(e) = file.read_to_string(&mut notes) {
                eprintln!("Unable to read to file: {}", e);
            }
            notes.lines()
                 .filter_map(|note| {
                     if note.len() <= 1 {
                         return None
                     }

                     id += 1;
                     if unwanted_ids.contains(&(id)) {
                         println!("Tossed note: {}", note.get(..note.len()-2).unwrap());
                         return None
                     }
                     Some(note.to_string())
                 })
                 .collect()
        }
        Err(e) => {
            eprintln!("Unable to open file: {}", e);
            vec![]
        }
    }
}

pub fn append_notes(notes: &str) {
    let path = get_notes_file();
    match OpenOptions::new().append(true).create(true).open(path.to_str().unwrap()) {
        Ok(mut file) => {
            if let Err(e) = write!(file, "{}\n", notes) {
                eprintln!("Unable to write to notes.txt: {}", e);
            }
        }
        Err(e) => eprintln!("Unable to open/create file: {}", e),
    }
}

pub fn edit_notes(unwanted_id: &usize, new_note: &[String], new_priority: &usize) {
    let path = get_notes_file();
    let mut all_notes = Vec::new();
    match OpenOptions::new().read(true).open(path.to_str().unwrap()) {
        Ok(mut file) => {
            let mut id = 0;
            let mut notes = String::new();
            if let Err(e) = file.read_to_string(&mut notes) {
                eprintln!("Unable to read file: {}", e);
            }
            all_notes = notes.lines()
                 .filter_map(|note| {
                     if note.len() <= 1 {
                         return None
                     }

                     id += 1;
                     if *unwanted_id == id {
                         if is_arg_set("-p", "--priority") {
                             if new_note.len() != 0 {
                                let new_note = format!("{}:{}", new_note.join(" "), new_priority);
                                return Some(new_note);
                             }
                             let mut new_note = Note::from(note);
                             new_note.priority = *new_priority;
                             return Some(format!("{}:{}", new_note.note, new_note.priority));
                         }
                         let note = Note::from(note);
                         return Some(format!("{}:{}", new_note.join(" "), note.priority));
                     }
                     Some(note.to_string())
                 })
                 .collect();
        }
        Err(e) => eprintln!("Unable to open file: {}", e),
    }
    write_notes(&all_notes);
}

pub fn clear_notes() {
    let path = get_notes_file();
    match OpenOptions::new().write(true).open(path.to_str().unwrap()) {
        Ok(file) => {
            file.set_len(0).unwrap();
        }
        Err(e) => eprintln!("Unable to clear file: {}", e),
    }
}

pub fn write_notes(notes: &[String]) {
    let path = get_notes_file();
    match OpenOptions::new().write(true).truncate(true).open(path.to_str().unwrap()) {
        Ok(mut file) => {
            if let Err(e) = write!(file, "{}\n", notes.join("\n")) {
                eprintln!("Unable to write to notes.txt: {}", e);
            }
        },
        Err(e) => eprintln!("Unable to open/create file: {}", e),
    }
}

pub fn style_string(original: &str, color: &str, style: &str) -> colored::ColoredString {
    let colored = match color {
        "black" => original.color(colored::Color::Black),
        "blue" => original.color(colored::Color::Blue),
        "cyan" => original.color(colored::Color::Cyan),
        "green" => original.color(colored::Color::Green),
        "magenta" => original.color(colored::Color::Magenta),
        "red" => original.color(colored::Color::Red),
        "white" => original.color(colored::Color::White),
        "yellow" => original.color(colored::Color::Yellow),
        "bright black" => original.color(colored::Color::BrightBlack),
        "bright blue" => original.color(colored::Color::BrightBlue),
        "bright cyan" => original.color(colored::Color::BrightCyan),
        "bright green" => original.color(colored::Color::BrightGreen),
        "bright magenta" => original.color(colored::Color::BrightMagenta),
        "bright red" => original.color(colored::Color::BrightRed),
        "bright white" => original.color(colored::Color::BrightWhite),
        "bright yellow" => original.color(colored::Color::BrightYellow),
        _ => original.normal()
    };

    match style {
        "bold" => colored.bold(),
        "italic" => colored.italic(),
        "underline" => colored.underline(),
        _ => colored,
    }
}
