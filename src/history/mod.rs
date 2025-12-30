use std::io::Write;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

pub struct History {
    file: Option<File>,
    index: usize,
    cursor: usize,
    append: usize,
    entries: Vec<String>,
}

impl History {
    pub fn new() -> History {
        let append = 0;
        let mut index = 0;
        let mut cursor = 0;
        let mut entries = Vec::new();

        let env_path = std::env::var("HISTFILE").map(PathBuf::from);
        let Ok(path) = env_path else {
            return History { file: None, index, cursor, append, entries };
        };

        let options = OpenOptions::new()
            .write(true)
            .read(true)
            .create(true)
            .append(true)
            .open(&path);

        if let Ok(file) = options {
            if let Some(parent_dir) = &path.parent() {
                if let Err(_) = std::fs::create_dir_all(parent_dir) {
                    return History { file: None, index, cursor, append, entries };
                }
            }

            // Brings to RAM every line
            let reader = BufReader::new(&file);
            for line in reader.lines() {
                if let Ok(cmd) = line {
                    entries.push(cmd);
                    index += 1;
                    cursor += 1;
                }
            }
            return History { file: Some(file), index, cursor, append, entries };
        }
        History { file: None, index, cursor, append, entries }
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn add(&mut self, line: String) {
        self.entries.push(line);
        self.cursor = self.entries.len();
    }

    pub fn back(&mut self) -> Option<&str> {
        if self.entries.is_empty() {
            return None;
        }
        if self.cursor != 0 {
            self.cursor -= 1;
        }
        Some(&self.entries[self.cursor])
    }
    pub fn next(&mut self) -> Option<&str> {
        if self.entries.is_empty() || self.cursor >= self.entries.len() {
            return None;
        }
        if self.cursor == self.entries.len() - 1 {
            self.cursor += 1;
            return None;
        }
        self.cursor += 1;
        Some(&self.entries[self.cursor])
    }

    pub fn iter(&'_ self) -> std::slice::Iter<'_, String> {
        self.entries.iter()
    }

    pub fn rev(&'_ self) -> std::iter::Rev<std::slice::Iter<'_, String>> {
        self.entries.iter().rev()
    }
    pub fn read(&mut self, history: File) {
        let mut entries = Vec::new();
        let reader = BufReader::new(&history);
        for line in reader.lines() {
            if let Ok(cmd) = line {
                entries.push(cmd);
            }
        }
        self.entries.append(&mut entries);
    }

    pub fn write(&self, mut file: File) {
        let mut output = String::new();
        for entry in self.entries.iter() {
            output.push_str(entry);
            output.push('\n');
        }
        write!(file, "{}", &output).expect("something went wrong");
    }

    pub fn append(&mut self, mut file: File) {
        let mut output = String::new();
        for i in self.append..self.entries.len() {
            let entry = self.entries[i].as_str();
            output.push_str(entry);
            output.push('\n');
        }
        self.append = self.entries.len() - self.append;
        write!(file, "{}", &output).expect("something went wrong");
    }
}

impl Drop for History {
    fn drop(&mut self) {
        let mut output = String::new();
        if let Some(mut file) = self.file.take() {
            for i in self.index..self.entries.len() {
                let entry = self.entries[i].as_str();
                output.push_str(entry);
                output.push('\n');
            }
            write!(file, "{}", &output).expect("something went wrong");
        }
    }
}