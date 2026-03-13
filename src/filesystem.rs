use std::fs;
use std::path::{Path, PathBuf};

/// Creates `icons/icns` and `icons/ico` inside the user's Downloads folder.
/// Returns `(icns_dir, ico_dir)`.
pub fn create_directories() -> std::io::Result<(PathBuf, PathBuf)> {
    let home = dirs_next::home_dir().ok_or_else(|| {
        std::io::Error::new(std::io::ErrorKind::NotFound, "Could not find home directory")
    })?;
    let base = home.join("Downloads").join("icons");
    let icns_dir = base.join("icns");
    let ico_dir = base.join("ico");

    fs::create_dir_all(&icns_dir)?;
    fs::create_dir_all(&ico_dir)?;

    Ok((icns_dir, ico_dir))
}

/// Opens the given path in the OS native file explorer (Windows Explorer).
pub fn open_folder(path: &Path) {
    let _ = std::process::Command::new("explorer")
        .arg(path)
        .spawn();
}
