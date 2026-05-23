use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "tracker",
    about = "A personal issue tracker — captures what you built, where you got stuck, and how you got unstuck.",
    version
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Create a new issue
    Create {
        /// Short, atomic description of the work (max 200 chars)
        title: String,
        /// Priority level [default: medium]
        #[arg(long, value_name = "low|medium|high", default_value = "medium")]
        priority: String,
        /// Add a label — repeatable: --label bug --label cli
        /// (labels are lowercased; letters, numbers, hyphens, underscores, dots only)
        #[arg(long, value_name = "TAG", action = clap::ArgAction::Append)]
        label: Vec<String>,
    },

    /// List issues, sorted by priority (high → low)
    ///
    /// Default: shows all non-done issues (open, in-progress, stuck, blocked).
    /// Use --status done to see completed issues.
    /// Multiple --label flags require ALL labels to match (AND semantics).
    List {
        /// Show only issues with this status
        /// [possible values: open, in-progress, stuck, blocked, done]
        #[arg(long, value_name = "STATUS")]
        status: Option<String>,
        /// Show only issues with this priority [possible values: low, medium, high]
        #[arg(long, value_name = "LEVEL")]
        priority: Option<String>,
        /// Filter by label — repeatable, AND semantics (issue must have all given labels)
        #[arg(long, value_name = "TAG", action = clap::ArgAction::Append)]
        label: Vec<String>,
    },

    /// Show full details of an issue including its complete timeline
    Show {
        /// Issue ID
        id: u32,
    },

    /// Change the workflow status of an issue
    ///
    /// Valid transitions: open → in-progress → done.
    /// done is terminal — closed issues cannot be reopened.
    /// To record that you hit a wall inside an issue, use `stuck` instead.
    Status {
        /// Issue ID
        id: u32,
        /// New status: open | in-progress | done
        new_status: String,
    },

    /// Record that you hit a wall inside this issue (narrative note on the timeline)
    ///
    /// Use this for internal blockages — things you need to figure out within the
    /// scope of this issue. For external dependencies (another issue must finish
    /// first), use `blocked-by` instead.
    Stuck {
        /// Issue ID
        id: u32,
        /// What you're stuck on (max 500 chars)
        reason: String,
    },

    /// Record what resolved an internal blockage (pairs with `stuck`)
    Unstuck {
        /// Issue ID
        id: u32,
        /// What resolved it (max 500 chars)
        resolution: String,
    },

    /// Mark this issue as blocked until another issue is done
    ///
    /// Adds a dependency: this issue will show as `blocked` until the other
    /// issue reaches `done`. Use `unblock` to remove the dependency.
    #[command(name = "blocked-by")]
    BlockedBy {
        /// Issue ID (the one being blocked)
        id: u32,
        /// ID of the issue that must be done first
        other_id: u32,
    },

    /// Remove a blocking dependency between two issues
    Unblock {
        /// Issue ID (the one currently blocked)
        id: u32,
        /// ID of the issue to remove from the dependency list
        other_id: u32,
    },

    /// Add one or more labels to an issue
    ///
    /// Labels are lowercased automatically. Duplicates are ignored silently.
    /// Allowed characters: letters, numbers, hyphens, underscores, dots.
    Label {
        /// Issue ID
        id: u32,
        /// Labels to add (space-separated)
        #[arg(required = true)]
        tags: Vec<String>,
    },

    /// Permanently delete an issue
    Delete {
        /// Issue ID
        id: u32,
    },
}
