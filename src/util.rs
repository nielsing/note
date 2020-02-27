use std::env;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::PathBuf;

use colored::*;
use regex::Regex;

use crate::arguments::*;
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
            notes
                .lines()
                .filter_map(|note| {
                    if note.len() <= 1 {
                        return None;
                    }

                    id += 1;

                    let mut note = Note::from(note);
                    note.id = id;
                    // Marking a note as unused
                    if unwanted_ids.contains(&id) {
                        note.in_use = false;
                    }

                    Some(note)
                })
                .collect()
        }
        Err(e) => {
            eprintln!("Unable to open file: {}", e);
            vec![]
        }
    }
}

pub fn append_note(notes: &str) {
    let path = get_notes_file();
    match OpenOptions::new()
        .append(true)
        .create(true)
        .open(path.to_str().unwrap())
    {
        Ok(mut file) => {
            if let Err(e) = writeln!(file, "{}", notes) {
                eprintln!("Unable to write to notes.txt: {}", e);
            }
        }
        Err(e) => eprintln!("Unable to open/create file: {}", e),
    }
}

pub fn edit_note(unwanted_id: usize, new_note: &[String], new_priority: usize) {
    let path = get_notes_file();
    let mut all_notes = Vec::new();
    match OpenOptions::new().read(true).open(path.to_str().unwrap()) {
        Ok(mut file) => {
            let mut id = 0;
            let mut notes = String::new();
            if let Err(e) = file.read_to_string(&mut notes) {
                eprintln!("Unable to read file: {}", e);
            }
            all_notes = notes
                .lines()
                .filter_map(|note| {
                    if note.len() <= 1 {
                        return None;
                    }

                    id += 1;
                    if unwanted_id == id {
                        if is_arg_set("-p", "--priority") {
                            if !new_note.is_empty() {
                                let new_note = format!("{}:{}", new_note.join(" "), new_priority);
                                return Some(new_note);
                            }
                            let mut new_note = Note::from(note);
                            new_note.priority = new_priority;
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

// Completely empties notes file
pub fn clear_notes() {
    let path = get_notes_file();
    match OpenOptions::new().write(true).open(path.to_str().unwrap()) {
        Ok(file) => {
            file.set_len(0).unwrap();
        }
        Err(e) => eprintln!("Unable to clear file: {}", e),
    }
}

// Writes specified notes to notes file and truncates the file if the number of notes being written
// to the file is lower than the number of notes within the file.
pub fn write_notes(notes: &[String]) {
    let path = get_notes_file();
    match OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(path.to_str().unwrap())
    {
        Ok(mut file) => {
            if let Err(e) = writeln!(file, "{}", notes.join("\n")) {
                eprintln!("Unable to write to notes.txt: {}", e);
            }
        }
        Err(e) => eprintln!("Unable to open/create file: {}", e),
    }
}

// Takes in a string (original) and colors and styles it, returning a ColoredString
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
        _ => original.normal(),
    };

    match style {
        "bold" => colored.bold(),
        "italic" => colored.italic(),
        "underline" => colored.underline(),
        _ => colored,
    }
}

fn get_bracket_locations(notes: &[Note]) -> Vec<Option<usize>> {
    let re = Regex::new(r"^\[[[:alpha:]\s\+]+\]").unwrap();
    notes
        .iter()
        .map(|note| match re.find(&note.note) {
            Some(pos) => Some(pos.end()),
            None => None,
        })
        .collect()
}

// Style all notes based by priority level
pub fn get_styled_notes(notes: &[Note], show_id: bool) -> Vec<String> {
    let bracket_locations = get_bracket_locations(notes);
    let max_bracket = match bracket_locations.iter().max() {
        Some(option) => match option {
            Some(val) => *val,
            None => 0,
        },
        None => 0,
    };

    notes
        .iter()
        .enumerate()
        .map(|(index, note)| {
            let location = match bracket_locations[index] {
                Some(location) => location,
                None => 0,
            };
            let num_of_spaces = max_bracket - location;

            // Specify format of the note.
            // If the show_id flag is set we will format notes with the ID
            // If the note matches the regex rule in 'get_bracket_locations()' we will format
            // with an appropriate number of spaces
            let note_format = if show_id && location != 0 {
                format!(
                    "{}{}{}{}",
                    note.id,
                    note.note.get(..location).unwrap(),
                    " ".repeat(num_of_spaces),
                    note.note.get(location..).unwrap()
                )
            } else if show_id {
                note.to_string()
            } else if location != 0 {
                format!(
                    "{}{}{}",
                    note.note.get(..location).unwrap(),
                    " ".repeat(num_of_spaces),
                    note.note.get(location..).unwrap()
                )
            } else {
                note.note.clone()
            };

            match note.priority {
                5 => format!(
                    "{}",
                    style_string(
                        &note_format,
                        &get_env_arg(EnvArgs::P5Color),
                        &get_env_arg(EnvArgs::P5Style)
                    )
                ),
                4 => format!(
                    "{}",
                    style_string(
                        &note_format,
                        &get_env_arg(EnvArgs::P4Color),
                        &get_env_arg(EnvArgs::P3Style)
                    )
                ),
                3 => format!(
                    "{}",
                    style_string(
                        &note_format,
                        &get_env_arg(EnvArgs::P3Color),
                        &get_env_arg(EnvArgs::P3Style)
                    )
                ),
                2 => format!(
                    "{}",
                    style_string(
                        &note_format,
                        &get_env_arg(EnvArgs::P2Color),
                        &get_env_arg(EnvArgs::P2Style)
                    )
                ),
                _ => format!(
                    "{}",
                    style_string(
                        &note_format,
                        &get_env_arg(EnvArgs::P1Color),
                        &get_env_arg(EnvArgs::P1Style)
                    )
                ),
            }
        })
        .collect()
}
