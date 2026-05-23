mod cli;
mod data;
mod store;

use chrono::Utc;
use clap::Parser;
use cli::{Cli, Commands};
use data::{EventKind, Issue, Priority, TimelineEvent};

const TRACKER_FILE: &str = "tracker.json";

fn main() {
    let cli = Cli::parse();

    if let Err(e) = run(cli) {
        eprintln!("error: {e}");
        std::process::exit(1);
    }
}

/// Returns (known_ids, done_ids) for status derivation.
/// known_ids = all IDs in the tracker; done_ids = subset that are closed.
fn known_and_done(
    tracker: &data::TrackerFile,
) -> (
    std::collections::HashSet<u32>,
    std::collections::HashSet<u32>,
) {
    let known_ids = tracker.issues.iter().map(|i| i.id).collect();
    let done_ids = tracker
        .issues
        .iter()
        .filter(|i| matches!(i.timeline.last().map(|e| &e.event), Some(EventKind::Closed)))
        .map(|i| i.id)
        .collect();
    (known_ids, done_ids)
}

fn parse_priority(s: &str) -> Result<Priority, String> {
    match s {
        "low" => Ok(Priority::Low),
        "medium" => Ok(Priority::Medium),
        "high" => Ok(Priority::High),
        other => Err(format!("invalid priority {other:?} — must be low, medium, or high")),
    }
}

fn run(cli: Cli) -> Result<(), String> {
    match cli.command {
        Commands::Create { title, priority, label } => {
            let title = title.trim().to_string();
            if title.is_empty() {
                return Err("title must not be empty".to_string());
            }
            let priority = parse_priority(&priority)?;
            let labels: Vec<String> = {
                let mut ls: Vec<String> = label.iter()
                    .map(|l| l.trim().to_lowercase())
                    .filter(|l| !l.is_empty())
                    .collect();
                ls.sort();
                ls.dedup();
                ls
            };

            let mut tracker = store::load(TRACKER_FILE)?;
            let next_id = tracker.issues.iter().map(|i| i.id).max().unwrap_or(0) + 1;

            let issue = Issue {
                id: next_id,
                title: title.clone(),
                priority,
                labels,
                blocked_by: Vec::new(),
                timeline: vec![TimelineEvent {
                    timestamp: Utc::now(),
                    event: EventKind::Opened,
                    note: None,
                }],
            };

            tracker.issues.push(issue);
            store::save(TRACKER_FILE, &tracker)?;
            println!("Created issue #{next_id}: {title}");
        }

        Commands::List { status, priority, label } => {
            let tracker = store::load(TRACKER_FILE)?;

            // Normalise filter inputs
            let filter_labels: Vec<String> = label.iter()
                .map(|l| l.trim().to_lowercase())
                .filter(|l| !l.is_empty())
                .collect();

            let (known_ids, done_ids) = known_and_done(&tracker);

            let mut issues: Vec<_> = tracker
                .issues
                .iter()
                .filter(|i| {
                    let s = i.status(&known_ids, &done_ids);
                    // Default: show everything not done
                    s != data::Status::Done
                })
                .filter(|i| {
                    // AND semantics: issue must have every requested label
                    filter_labels.iter().all(|l| i.labels.contains(l))
                })
                .collect();

            // Sort high → medium → low
            issues.sort_by(|a, b| {
                b.priority.sort_key().cmp(&a.priority.sort_key())
            });

            if issues.is_empty() {
                println!("No matching issues.");
            } else {
                for issue in issues {
                    let s = issue.status(&known_ids, &done_ids);
                    println!("#{:<4} [{:<11}] [{}] {}",
                        issue.id, s.to_string(), issue.priority, issue.title);
                }
            }
            // status and priority filters come in Phase 8
            let _ = (status, priority);
        }
        Commands::Show { id } => {
            let tracker = store::load(TRACKER_FILE)?;
            let (known_ids, done_ids) = known_and_done(&tracker);

            let issue = tracker.issues.iter().find(|i| i.id == id)
                .ok_or_else(|| format!("issue #{id} not found"))?;

            let status = issue.status(&known_ids, &done_ids);
            println!("#{} — {}", issue.id, issue.title);
            println!("  Status:   {status}");
            println!("  Priority: {}", issue.priority);

            if issue.labels.is_empty() {
                println!("  Labels:   (none)");
            } else {
                println!("  Labels:   {}", issue.labels.join(", "));
            }

            if issue.blocked_by.is_empty() {
                println!("  Blocked by: (none)");
            } else {
                let ids: Vec<String> = issue.blocked_by.iter().map(|i| format!("#{i}")).collect();
                println!("  Blocked by: {}", ids.join(", "));
            }

            println!("  Timeline:");
            for event in &issue.timeline {
                let ts = event.timestamp.format("%Y-%m-%d %H:%M:%S UTC");
                if let Some(note) = &event.note {
                    println!("    {ts}  {}  — {note}", event.event);
                } else {
                    println!("    {ts}  {}", event.event);
                }
            }
        }
        Commands::Status { id, new_status } => {
            let mut tracker = store::load(TRACKER_FILE)?;
            let (known_ids, done_ids) = known_and_done(&tracker);

            let event_kind = match new_status.as_str() {
                "open"        => EventKind::Opened,
                "in-progress" => EventKind::InProgress,
                "done"        => EventKind::Closed,
                other => return Err(format!(
                    "invalid status {other:?} — must be open, in-progress, or done"
                )),
            };

            let issue = tracker.issues.iter_mut().find(|i| i.id == id)
                .ok_or_else(|| format!("issue #{id} not found"))?;

            // done is terminal
            if matches!(issue.timeline.last().map(|e| &e.event), Some(EventKind::Closed)) {
                return Err(format!("issue #{id} is done; done is a terminal state"));
            }

            // Check blocked status before allowing in-progress/done
            let current_status = issue.status(&known_ids, &done_ids);
            if current_status == data::Status::Blocked
                && matches!(event_kind, EventKind::Closed | EventKind::InProgress)
            {
                return Err(format!(
                    "issue #{id} is blocked by unresolved dependencies; resolve them first"
                ));
            }

            issue.timeline.push(TimelineEvent {
                timestamp: Utc::now(),
                event: event_kind,
                note: None,
            });
            store::save(TRACKER_FILE, &tracker)?;
            println!("Issue #{id} is now {new_status}");
        }
        Commands::Stuck { id, reason } => {
            println!("stuck: not yet implemented (id={id}, reason={reason:?})");
        }
        Commands::Unstuck { id, resolution } => {
            println!("unstuck: not yet implemented (id={id}, resolution={resolution:?})");
        }
        Commands::BlockedBy { id, other_id } => {
            if id == other_id {
                return Err(format!("issue #{id} cannot be blocked by itself"));
            }
            let mut tracker = store::load(TRACKER_FILE)?;

            // Verify both issues exist before mutating anything.
            if !tracker.issues.iter().any(|i| i.id == other_id) {
                return Err(format!("issue #{other_id} not found"));
            }
            let issue = tracker.issues.iter_mut().find(|i| i.id == id)
                .ok_or_else(|| format!("issue #{id} not found"))?;

            if issue.blocked_by.contains(&other_id) {
                println!("Issue #{id} is already blocked by #{other_id}.");
            } else {
                issue.blocked_by.push(other_id);
                store::save(TRACKER_FILE, &tracker)?;
                println!("Issue #{id} is now blocked by #{other_id}.");
            }
        }

        Commands::Unblock { id, other_id } => {
            let mut tracker = store::load(TRACKER_FILE)?;
            let issue = tracker.issues.iter_mut().find(|i| i.id == id)
                .ok_or_else(|| format!("issue #{id} not found"))?;

            if !issue.blocked_by.contains(&other_id) {
                return Err(format!("issue #{id} is not blocked by #{other_id}"));
            }
            issue.blocked_by.retain(|&dep| dep != other_id);
            store::save(TRACKER_FILE, &tracker)?;
            println!("Removed dependency: #{id} is no longer blocked by #{other_id}.");
        }
        Commands::Label { id, tags } => {
            let mut tracker = store::load(TRACKER_FILE)?;
            let issue = tracker.issues.iter_mut().find(|i| i.id == id)
                .ok_or_else(|| format!("issue #{id} not found"))?;

            let mut added = Vec::new();
            for tag in tags {
                let tag = tag.trim().to_lowercase();
                if tag.is_empty() {
                    return Err("label must not be empty".to_string());
                }
                if !issue.labels.contains(&tag) {
                    issue.labels.push(tag.clone());
                    added.push(tag);
                }
            }
            issue.labels.sort();

            store::save(TRACKER_FILE, &tracker)?;
            if added.is_empty() {
                println!("No new labels added (all already present).");
            } else {
                println!("Added labels to #{id}: {}", added.join(", "));
            }
        }
        Commands::Delete { id } => {
            let mut tracker = store::load(TRACKER_FILE)?;
            let before = tracker.issues.len();
            tracker.issues.retain(|i| i.id != id);
            if tracker.issues.len() == before {
                return Err(format!("issue #{id} not found"));
            }
            store::save(TRACKER_FILE, &tracker)?;
            println!("Deleted issue #{id}");
        }
    }
    Ok(())
}
