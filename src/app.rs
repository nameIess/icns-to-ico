use std::path::PathBuf;

/// Which screen the app is currently on
#[derive(Debug, Clone, PartialEq)]
pub enum Screen {
    /// Waiting for user to place files and press Enter
    WaitingForInput,
    /// Currently converting files
    Converting,
    /// Conversion finished, showing results
    Done,
}

/// A single log entry emitted during conversion
#[derive(Debug, Clone)]
pub struct LogEntry {
    pub success: bool,
    pub message: String,
}

/// Application state
pub struct App {
    pub screen: Screen,
    pub icns_dir: PathBuf,
    pub ico_dir: PathBuf,
    pub logs: Vec<LogEntry>,
    pub converted: usize,
    pub failed: usize,
    /// Whether the user pressed 'q' / Esc to quit
    pub should_quit: bool,
}

impl App {
    pub fn new(icns_dir: PathBuf, ico_dir: PathBuf) -> Self {
        Self {
            screen: Screen::WaitingForInput,
            icns_dir,
            ico_dir,
            logs: Vec::new(),
            converted: 0,
            failed: 0,
            should_quit: false,
        }
    }

    pub fn add_result(&mut self, success: bool, message: String) {
        self.logs.push(LogEntry { success, message });
        if success {
            self.converted += 1;
        } else {
            self.failed += 1;
        }
    }
}
