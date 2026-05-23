use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// The top-level structure of tracker.json.
#[derive(Debug, Serialize, Deserialize)]
pub struct TrackerFile {
    pub meta: Meta,
    pub issues: Vec<Issue>,
}

impl TrackerFile {
    pub fn new(name: &str) -> Self {
        TrackerFile {
            meta: Meta {
                name: name.to_string(),
                created_at: Utc::now(),
            },
            issues: Vec::new(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Meta {
    pub name: String,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Issue {
    pub id: u32,
    pub title: String,
    pub priority: Priority,
    pub labels: Vec<String>,
    #[serde(rename = "blockedBy")]
    pub blocked_by: Vec<u32>,
    pub timeline: Vec<TimelineEvent>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Priority {
    Low,
    Medium,
    High,
}

impl Priority {
    /// Numeric sort key: higher number = higher priority (for sorting high→low).
    pub fn sort_key(&self) -> u8 {
        match self {
            Priority::High => 2,
            Priority::Medium => 1,
            Priority::Low => 0,
        }
    }
}

impl std::fmt::Display for Priority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Priority::Low => write!(f, "low"),
            Priority::Medium => write!(f, "medium"),
            Priority::High => write!(f, "high"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TimelineEvent {
    pub timestamp: DateTime<Utc>,
    pub event: EventKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum EventKind {
    Opened,
    InProgress,
    Stuck,
    Unstuck,
    Closed,
}

impl std::fmt::Display for EventKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EventKind::Opened => write!(f, "opened"),
            EventKind::InProgress => write!(f, "in-progress"),
            EventKind::Stuck => write!(f, "stuck"),
            EventKind::Unstuck => write!(f, "unstuck"),
            EventKind::Closed => write!(f, "closed"),
        }
    }
}

/// Derived status of an issue. Not stored — computed at read time.
#[derive(Debug, PartialEq)]
pub enum Status {
    Open,
    InProgress,
    Stuck,
    Blocked,
    Done,
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Status::Open => write!(f, "open"),
            Status::InProgress => write!(f, "in-progress"),
            Status::Stuck => write!(f, "stuck"),
            Status::Blocked => write!(f, "blocked"),
            Status::Done => write!(f, "done"),
        }
    }
}

impl Issue {
    /// Derive status from blockedBy relationships and the timeline.
    /// `done_ids` is the set of issue IDs that are currently done.
    pub fn status(&self, done_ids: &std::collections::HashSet<u32>) -> Status {
        // blockedBy takes precedence: if any dependency isn't done, we're blocked.
        if self
            .blocked_by
            .iter()
            .any(|dep_id| !done_ids.contains(dep_id))
        {
            return Status::Blocked;
        }

        // Derive from the last timeline event.
        match self.timeline.last().map(|e| &e.event) {
            Some(EventKind::Closed) => Status::Done,
            Some(EventKind::Stuck) => Status::Stuck,
            Some(EventKind::InProgress) | Some(EventKind::Unstuck) => Status::InProgress,
            _ => Status::Open,
        }
    }
}
