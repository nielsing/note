use structopt::StructOpt;

use note::arguments::*;
use note::note::*;
use note::util;

fn edit(id: usize, note: &[String], priority: usize) {
    util::edit_note(id, note, priority);
}

fn list(show_id: bool, priority: bool, limit: Option<usize>) {
    let mut notes: Vec<Note> = util::read_to_notes(&[]);

    // Print empty message if notes file is empty
    if notes.is_empty() {
        print!(
            "{}",
            util::style_string(
                &get_env_arg(EnvArgs::EmptyMessage),
                &get_env_arg(EnvArgs::EmptyMessageColor),
                &get_env_arg(EnvArgs::EmptyMessageStyle)
            )
        );
        return;
    }

    // Print welcome message
    print!(
        "{}\n{}\n",
        util::style_string(
            &get_env_arg(EnvArgs::Message),
            &get_env_arg(EnvArgs::MessageColor),
            &get_env_arg(EnvArgs::MessageStyle)
        ),
        "-".repeat(get_env_arg(EnvArgs::Message).len())
    );

    // Sort by priority if priority flag is set
    if priority {
        notes.sort_by(|a, b| a.priority.cmp(&b.priority).reverse());
    }

    let styled_notes = util::get_styled_notes(&notes, show_id);
    match limit {
        Some(limit) => println!("{}", styled_notes[..limit].join("\n")),
        None => println!("{}", styled_notes.join("\n")),
    };
}

fn stick(note: &[String], priority: usize) {
    let note = format!("{}:{}", note.join(" "), priority);
    util::append_note(&note);
}

fn toss(ids: &[usize], all: bool) {
    let notes = util::read_to_notes(ids);
    let mut tmp = Vec::new();
    for note in &notes {
        if !note.in_use {
            println!("Tossed note: {}", note.note.get(..note.note.len()-2).unwrap());
        } else {
            tmp.push(note.to_string());
        }
    }
    if all || notes.is_empty() {
        println!("All notes tossed away!");
        util::clear_notes();
        return;
    }

    util::write_notes(&tmp);
}

fn main() {
    let args = Args::from_args();

    match args.action {
        Action::Edit { id, note, priority } => edit(id, &note, priority),
        Action::List {
            show_id,
            priority,
            limit,
        } => list(show_id, priority, limit),
        Action::Stick { note, priority } => stick(&note, priority),
        Action::Toss { ids, all } => toss(&ids, all),
    }
}
