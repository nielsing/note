use structopt::StructOpt;

use note::util;
use note::note::*;
use note::arguments::{Args, Action, EnvArgs, get_env_arg};

fn edit(id: &usize, note: &[String], priority: &usize) {
    util::edit_notes(id, note, priority);
}

fn list(show_id: bool, priority: bool) {
    let mut notes: Vec<Note> = util::read_to_notes(&[]);
    if notes.len() == 0 {
        print!("{}", util::style_string(&get_env_arg(EnvArgs::EmptyMessage),
                                        &get_env_arg(EnvArgs::EmptyMessageColor),
                                        &get_env_arg(EnvArgs::EmptyMessageStyle)));
        return;
    }

    print!("{}\n{}\n", util::style_string(&get_env_arg(EnvArgs::Message), 
                                          &get_env_arg(EnvArgs::MessageColor), 
                                          &get_env_arg(EnvArgs::MessageStyle)), 
                       vec!["-"; get_env_arg(EnvArgs::Message).len()].join(""));
    if priority {
        notes.sort_by(|a, b| a.priority.cmp(&b.priority).reverse());
    }

    let notes: Vec<String> = notes.into_iter()
        .map(|note| {
            let note_format = if show_id { note.to_string() } else { note.note };
            match note.priority {
                5 => format!("{}", util::style_string(&note_format,
                                                      &get_env_arg(EnvArgs::P5Color),
                                                      &get_env_arg(EnvArgs::P5Style))),
                4 => format!("{}", util::style_string(&note_format,
                                                      &get_env_arg(EnvArgs::P4Color),
                                                      &get_env_arg(EnvArgs::P3Style))),
                3 => format!("{}", util::style_string(&note_format,
                                                      &get_env_arg(EnvArgs::P3Color),
                                                      &get_env_arg(EnvArgs::P3Style))),
                2 => format!("{}", util::style_string(&note_format,
                                                      &get_env_arg(EnvArgs::P2Color),
                                                      &get_env_arg(EnvArgs::P2Style))),
                _ => format!("{}", util::style_string(&note_format,
                                                      &get_env_arg(EnvArgs::P1Color),
                                                      &get_env_arg(EnvArgs::P1Style))),
            }
        })
        .collect();
    println!("{}", notes.join("\n"));
}

fn stick(note: &[String], priority: usize) {
    let note = format!("{}:{}", note.join(" "), priority);
    util::append_notes(&note);
}

fn toss(ids: &[usize], all: bool) {
    let notes = util::read_to_notes_str(ids);
    if all || notes.len() == 0 {
        util::clear_notes();
        return;
    }

    util::write_notes(&notes);
}

fn main() {
    let args = Args::from_args();
    
    match args.action {
        Action::Edit{ id, note, priority } => edit(&id, &note, &priority),
        Action::List{ show_id, priority } => list(show_id, priority),
        Action::Stick{ note, priority } => stick(&note, priority),
        Action::Toss{ ids, all } => toss(&ids, all)
    }
}
