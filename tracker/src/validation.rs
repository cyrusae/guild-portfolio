/// Validation and sanitization for all user-supplied string fields.
///
/// Each function returns the normalized value on success so callers don't
/// have to normalize separately.

pub const MAX_TITLE_LEN: usize = 200;
pub const MAX_LABEL_LEN: usize = 50;
pub const MAX_NOTE_LEN: usize = 500;
pub const MAX_ISSUES: usize = 10_000;

/// Trim and validate a title. Returns the trimmed title on success.
/// Rejects: empty/whitespace-only, control characters, over-length.
pub fn validate_title(raw: &str) -> Result<String, String> {
    let title = raw.trim().to_string();
    if title.is_empty() {
        return Err("title must not be empty".to_string());
    }
    if title.chars().any(|c| c.is_control()) {
        return Err("title must not contain control characters".to_string());
    }
    if title.len() > MAX_TITLE_LEN {
        return Err(format!(
            "title must be {MAX_TITLE_LEN} characters or fewer (got {})",
            title.len()
        ));
    }
    Ok(title)
}

/// Normalize and validate a label. Returns the lowercased label on success.
/// Rejects: empty, control characters, over-length, disallowed characters.
/// Allowed: letters, numbers, hyphens, underscores, dots.
pub fn validate_label(raw: &str) -> Result<String, String> {
    let label = raw.trim().to_lowercase();
    if label.is_empty() {
        return Err("label must not be empty".to_string());
    }
    if label.chars().any(|c| c.is_control()) {
        return Err(format!(
            "label {raw:?} must not contain control characters"
        ));
    }
    if label.len() > MAX_LABEL_LEN {
        return Err(format!(
            "label must be {MAX_LABEL_LEN} characters or fewer (got {})",
            label.len()
        ));
    }
    if !label
        .chars()
        .all(|c| c.is_alphanumeric() || matches!(c, '-' | '_' | '.'))
    {
        return Err(format!(
            "label {raw:?} contains invalid characters — use letters, numbers, hyphens, underscores, or dots"
        ));
    }
    Ok(label)
}

/// Trim and validate a freetext note (stuck reason, unstuck resolution).
/// Returns the trimmed note on success.
/// Rejects: empty/whitespace-only, control characters, over-length.
pub fn validate_note(raw: &str, field: &str) -> Result<String, String> {
    let note = raw.trim().to_string();
    if note.is_empty() {
        return Err(format!("{field} must not be empty"));
    }
    if note.chars().any(|c| c.is_control()) {
        return Err(format!("{field} must not contain control characters"));
    }
    if note.len() > MAX_NOTE_LEN {
        return Err(format!(
            "{field} must be {MAX_NOTE_LEN} characters or fewer (got {})",
            note.len()
        ));
    }
    Ok(note)
}

/// Check that the tracker has not hit the issue limit before creating a new one.
pub fn check_issue_limit(current_count: usize) -> Result<(), String> {
    if current_count >= MAX_ISSUES {
        return Err(format!(
            "tracker has reached the maximum of {MAX_ISSUES} issues"
        ));
    }
    Ok(())
}
