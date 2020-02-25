use std::env;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "Note - Terminal Sticky Notes!")]
pub struct Args {
    #[structopt(subcommand)]
    pub action: Action,
}

#[derive(Debug, StructOpt)]
pub enum Action {
    #[structopt(alias = "e")]
    /// Edit a note
    Edit {
        /// ID of note to edit
        id: usize,
        /// New value of note
        note: Vec<String>,
        #[structopt(short, long, default_value = "1")]
        /// New priority of note
        #[structopt(possible_values = &["1", "2", "3", "4", "5"])]
        priority: usize
    },
    /// List all notes
    #[structopt(alias = "ls")]
    List {
        /// List all notes with their IDs
        #[structopt(short, long)]
        show_id: bool,
        /// List notes in order of priority
        #[structopt(short, long)]
        priority: bool,
        /// Limit list to <limit> many notes. (useful with -p flag)
        #[structopt(short, long)]
        limit: Option<usize>,
    },
    /// Stick a new note
    #[structopt(alias = "s")]
    Stick {
        /// Note to stick to notes file
        note: Vec<String>,
        #[structopt(short, long, default_value = "1")]
        /// Priority of note
        #[structopt(possible_values = &["1", "2", "3", "4", "5"])]
        priority: usize
    },
    /// Toss a note to the trash
    #[structopt(alias = "t")]
    Toss {
        /// ID of notes to toss
        ids: Vec<usize>,
        /// Toss all notes
        #[structopt(short, long, conflicts_with = "ids")]
        all: bool
    },
}

const ENV_ARG_KEYS: [&str; 16] = [
    "NOTE_MESSAGE",
    "NOTE_MESSAGE_COLOR",
    "NOTE_MESSAGE_STYLE",
    "NOTE_EMPTY_MESSAGE",
    "NOTE_EMPTY_MESSAGE_COLOR",
    "NOTE_EMPTY_MESSAGE_STYLE",
    "NOTE_P1_COLOR",
    "NOTE_P1_STYLE",
    "NOTE_P2_COLOR",
    "NOTE_P2_STYLE",
    "NOTE_P3_COLOR",
    "NOTE_P3_STYLE",
    "NOTE_P4_COLOR",
    "NOTE_P4_STYLE",
    "NOTE_P5_COLOR",
    "NOTE_P5_STYLE"
];

const DEFAULT_ENV_VALUES: [&str; 16] = [
    "NOTES:",
    "",
    "bold",
    "",
    "",
    "bold",
    "",
    "normal",
    "blue",
    "normal",
    "green",
    "normal",
    "yellow",
    "normal",
    "red",
    "bold"
];

#[derive(Copy, Clone)]
pub enum EnvArgs {
    Message,
    MessageColor,
    MessageStyle,
    EmptyMessage,
    EmptyMessageColor,
    EmptyMessageStyle,
    P1Color,
    P1Style,
    P2Color,
    P2Style,
    P3Color,
    P3Style,
    P4Color,
    P4Style,
    P5Color,
    P5Style,
}

pub fn get_env_arg(key: EnvArgs) -> String {
    match env::var(ENV_ARG_KEYS[key as usize]) {
        Ok(val) => val,
        _ => DEFAULT_ENV_VALUES[key as usize].to_string(),
    }
}
