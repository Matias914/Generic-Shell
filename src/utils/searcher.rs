use std::env;
use is_executable::IsExecutable;

pub struct ExecutableSearcher;

impl ExecutableSearcher {
    pub fn search_in_path(command: &str) -> Option<String> {
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
}