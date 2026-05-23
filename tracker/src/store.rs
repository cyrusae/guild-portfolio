use std::path::Path;

use crate::data::TrackerFile;

/// Load tracker.json from the given path.
/// If the file doesn't exist, returns a fresh TrackerFile using the directory
/// name as the project name (so bootstrapping on first `create` is seamless).
pub fn load(path: &str) -> Result<TrackerFile, String> {
    let p = Path::new(path);

    if !p.exists() {
        let project_name = std::env::current_dir()
            .ok()
            .and_then(|d| d.file_name().map(|n| n.to_string_lossy().into_owned()))
            .unwrap_or_else(|| "project".to_string());
        return Ok(TrackerFile::new(&project_name));
    }

    let contents = std::fs::read_to_string(p)
        .map_err(|e| format!("error reading {path}: {e}"))?;

    if contents.trim().is_empty() {
        return Err(format!("{path} exists but is empty"));
    }

    serde_json::from_str(&contents)
        .map_err(|e| format!("error parsing {path}: {e}"))
}

/// Save a TrackerFile to the given path as pretty-printed JSON.
pub fn save(path: &str, tracker: &TrackerFile) -> Result<(), String> {
    let json = serde_json::to_string_pretty(tracker)
        .map_err(|e| format!("error serializing tracker: {e}"))?;

    std::fs::write(path, json)
        .map_err(|e| format!("error writing {path}: {e}"))?;

    Ok(())
}
