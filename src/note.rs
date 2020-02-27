use std::fmt;

#[derive(Debug)]
pub struct Note {
    pub id: usize,
    pub note: String,
    pub priority: usize,
    pub in_use: bool,
}

impl fmt::Display for Note {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.id, self.note)
    }
}

impl From<&str> for Note {
    fn from(line: &str) -> Self {
        let mut values: Vec<&str> = line.split(':').collect();
        if values.len() == 1 {
            return Note {
                id: 0,
                note: String::from(values[0]),
                priority: 1,
                in_use: true,
            };
        }
        let priority = match values.pop() {
            Some(value) => match value.parse() {
                Ok(num) => num,
                Err(_) => 1,
            },
            None => 1,
        };
        let note = values.join(":");
        Note {
            id: 0,
            note,
            priority,
            in_use: true,
        }
    }
}
