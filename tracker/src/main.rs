mod cli;
mod data;
mod store;
mod validation;

use chrono::Utc;
use clap::Parser;
use cli::{Cli, Commands};
use data::{EventKind, Issue, TimelineEvent};
use owo_colors::{OwoColorize, Stream};

const TRACKER_FILE: &str = "tracker.json";

// ── Display helpers ───────────────────────────────────────────────────────────

/// Pad to fixed width first, then colorize — ANSI codes don't affect padding.
fn fmt_status(s: &data::Status) -> String {
    let padded = format!("{s:<11}");
    match s {
        data::Status::Done       => format!("{}", padded.if_supports_color(Stream::Stdout, |t| t.green())),
        data::Status::InProgress => format!("{}", padded.if_supports_color(Stream::Stdout, |t| t.cyan())),
        data::Status::Stuck      => format!("{}", padded.if_supports_color(Stream::Stdout, |t| t.yellow())),
        data::Status::Blocked    => format!("{}", padded.if_supports_color(Stream::Stdout, |t| t.bright_yellow())),
        data::Status::Open       => padded,
    }
}

fn fmt_priority(p: &data::Priority) -> String {
    let s = p.to_string();
    match p {
        data::Priority::High   => format!("{}", s.if_supports_color(Stream::Stdout, |t| t.red())),
        data::Priority::Medium => s,
        data::Priority::Low    => format!("{}", s.if_supports_color(Stream::Stdout, |t| t.dimmed())),
    }
}

fn fmt_event(e: &EventKind) -> String {
    let s = e.to_string();
    match e {
        EventKind::Opened                    => s,
        EventKind::InProgress | EventKind::Unstuck => format!("{}", s.if_supports_color(Stream::Stdout, |t| t.cyan())),
        EventKind::Stuck                     => format!("{}", s.if_supports_color(Stream::Stdout, |t| t.yellow())),
        EventKind::Closed                    => format!("{}", s.if_supports_color(Stream::Stdout, |t| t.green())),
    }
}

/// Truncate a string to `max_chars` Unicode scalar values, appending … if cut.
fn truncate(s: &str, max_chars: usize) -> String {
    let mut chars = s.chars();
    let head: String = chars.by_ref().take(max_chars).collect();
    if chars.next().is_some() {
        format!("{head}…")
    } else {
        head
    }
}

fn main() {
    let cli = Cli::parse();

    if let Err(e) = run(cli) {
        eprintln!("{} {e}", "error:".if_supports_color(Stream::Stderr, |t| t.red()));
        std::process::exit(1);
    }
}


fn run(cli: Cli) -> Result<(), String> {
    match cli.command {
        Commands::Create { title, priority, label } => {
            let title = validation::validate_title(&title)?;
            let priority = data::Priority::parse(&priority)?;
            let labels: Vec<String> = {
                let mut ls: Vec<String> = label.iter()
                    .map(|raw| validation::validate_label(raw))
                    .collect::<Result<Vec<_>, _>>()?;
                ls.sort();
                ls.dedup();
                ls
            };

            let mut tracker = store::load(TRACKER_FILE)?;
            validation::check_issue_limit(tracker.issues.len())?;
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

            // Parse and validate filters up front so we error before loading.
            let filter_status: Option<data::Status> = status
                .as_deref()
                .map(data::Status::parse)
                .transpose()?;
            let filter_priority: Option<data::Priority> = priority
                .as_deref()
                .map(data::Priority::parse)
                .transpose()?;
            let filter_labels: Vec<String> = label.iter()
                .map(|raw| validation::validate_label(raw))
                .collect::<Result<Vec<_>, _>>()?;

            let (known_ids, done_ids) = tracker.known_and_done();

            let mut issues: Vec<_> = tracker
                .issues
                .iter()
                .filter(|i| {
                    let s = i.status(&known_ids, &done_ids);
                    match &filter_status {
                        // Default: show everything not done
                        None => s != data::Status::Done,
                        Some(wanted) => &s == wanted,
                    }
                })
                .filter(|i| {
                    match &filter_priority {
                        None => true,
                        Some(wanted) => &i.priority == wanted,
                    }
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
                    println!("#{:<4} [{}] [{}] {}",
                        issue.id,
                        fmt_status(&s),
                        fmt_priority(&issue.priority),
                        issue.title);
                }
            }
        }
        Commands::Show { id } => {
            let tracker = store::load(TRACKER_FILE)?;
            let (known_ids, done_ids) = tracker.known_and_done();

            let issue = tracker.issues.iter().find(|i| i.id == id)
                .ok_or_else(|| format!("issue #{id} not found"))?;

            let status = issue.status(&known_ids, &done_ids);
            println!("#{} — {}", issue.id, issue.title);
            println!("  Status:   {}", fmt_status(&status));
            println!("  Priority: {}", fmt_priority(&issue.priority));

            if issue.labels.is_empty() {
                println!("  Labels:   (none)");
            } else {
                println!("  Labels:   {}", issue.labels.join(", "));
            }

            if issue.blocked_by.is_empty() {
                println!("  Blocked by: (none)");
            } else {
                println!("  Blocked by:");
                for dep_id in &issue.blocked_by {
                    let title = tracker.issues.iter()
                        .find(|i| i.id == *dep_id)
                        .map(|i| format!(" — {}", truncate(&i.title, 50)))
                        .unwrap_or_default();
                    println!("    #{dep_id}{title}");
                }
            }

            println!("  Timeline:");
            for event in &issue.timeline {
                let ts = event.timestamp.format("%Y-%m-%d %H:%M:%S UTC");
                if let Some(note) = &event.note {
                    println!("    {ts}  {}  — {note}", fmt_event(&event.event));
                } else {
                    println!("    {ts}  {}", fmt_event(&event.event));
                }
            }
        }
        Commands::Status { id, new_status } => {
            let mut tracker = store::load(TRACKER_FILE)?;
            let (known_ids, done_ids) = tracker.known_and_done();

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
            let reason = validation::validate_note(&reason, "reason")?;

            let mut tracker = store::load(TRACKER_FILE)?;
            let (known_ids, done_ids) = tracker.known_and_done();
            let issue = tracker.issues.iter_mut().find(|i| i.id == id)
                .ok_or_else(|| format!("issue #{id} not found"))?;

            if matches!(issue.timeline.last().map(|e| &e.event), Some(EventKind::Closed)) {
                return Err(format!("issue #{id} is done; done is a terminal state"));
            }
            if issue.status(&known_ids, &done_ids) == data::Status::Stuck {
                return Err(format!("issue #{id} is already stuck"));
            }

            issue.timeline.push(TimelineEvent {
                timestamp: Utc::now(),
                event: EventKind::Stuck,
                note: Some(reason.clone()),
            });
            store::save(TRACKER_FILE, &tracker)?;
            println!("Issue #{id} marked stuck: {reason}");
        }

        Commands::Unstuck { id, resolution } => {
            let resolution = validation::validate_note(&resolution, "resolution")?;

            let mut tracker = store::load(TRACKER_FILE)?;
            let (known_ids, done_ids) = tracker.known_and_done();
            let issue = tracker.issues.iter_mut().find(|i| i.id == id)
                .ok_or_else(|| format!("issue #{id} not found"))?;

            if issue.status(&known_ids, &done_ids) != data::Status::Stuck {
                return Err(format!("issue #{id} is not currently stuck"));
            }

            issue.timeline.push(TimelineEvent {
                timestamp: Utc::now(),
                event: EventKind::Unstuck,
                note: Some(resolution.clone()),
            });
            store::save(TRACKER_FILE, &tracker)?;
            println!("Issue #{id} unstuck: {resolution}");
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
            for raw in tags {
                let tag = validation::validate_label(&raw)?;
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
