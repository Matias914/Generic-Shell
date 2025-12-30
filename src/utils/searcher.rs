use std::collections::LinkedList;
use std::{env, fs};
use is_executable::IsExecutable;

pub struct Searcher;

impl Searcher {
    pub fn search_executable_in_path(command: &str) -> Option<String> {
        let key = "PATH";
        let Ok(paths) = env::var(key) else {
            return None;
        };
        for path in env::split_paths(&paths) {
            let file = path.join(command);
            if file.is_executable() {
                if let Some(file_string) = file.to_str() {
                    return Some(file_string.into());
                };
            }
        }
        None
    }

    pub fn search_possible_executables_in_path(incomplete: &str) -> LinkedList<String> {
        let key = "PATH";
        let Ok(paths) = env::var(key) else {
            return LinkedList::new();
        };
        let mut results = LinkedList::new();
        for path in env::split_paths(&paths) {
            if let Ok(entries) = fs::read_dir(path) {
                for result_entry in entries {
                    if let Ok(entry) = result_entry {
                        let file = entry.path();
                        if let Some(name) = entry.file_name().to_str() {
                            if file.is_executable() {
                                if name.starts_with(incomplete) {
                                    results.push_back(name.into())
                                }
                            }
                        }
                    }
                }
            }
        }
        results
    }
}