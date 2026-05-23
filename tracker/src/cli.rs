use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "tracker", about = "A personal issue tracker", version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Create a new issue
    Create {
        /// Short description of the work to be done
        title: String,
        /// Priority level (default: medium)
        #[arg(long, value_name = "LEVEL", default_value = "medium")]
        priority: String,
        /// Add a label (repeatable: --label bug --label cli)
        #[arg(long, value_name = "TAG", action = clap::ArgAction::Append)]
        label: Vec<String>,
    },

    /// List issues (default: everything not done)
    List {
        /// Filter by status
        #[arg(long, value_name = "STATUS")]
        status: Option<String>,
        /// Filter by priority
        #[arg(long, value_name = "LEVEL")]
        priority: Option<String>,
        /// Filter by label — multiple flags use AND semantics
        #[arg(long, value_name = "TAG", action = clap::ArgAction::Append)]
        label: Vec<String>,
    },

    /// Show full details of an issue including its timeline
    Show {
        /// Issue ID
        id: u32,
    },

    /// Change the status of an issue (open, in-progress, done)
    Status {
        /// Issue ID
        id: u32,
        /// New status: open | in-progress | done
        new_status: String,
    },

    /// Record an internal blockage note (you hit a wall within this issue)
    Stuck {
        /// Issue ID
        id: u32,
        /// What you're stuck on
        reason: String,
    },

    /// Record the resolution of an internal blockage
    Unstuck {
        /// Issue ID
        id: u32,
        /// What resolved it
        resolution: String,
    },

    /// Mark this issue as depending on another issue
    #[command(name = "blocked-by")]
    BlockedBy {
        /// Issue ID
        id: u32,
        /// ID of the issue blocking this one
        other_id: u32,
    },

    /// Remove a blocking dependency between two issues
    Unblock {
        /// Issue ID
        id: u32,
        /// ID of the issue to remove from the blockedBy list
        other_id: u32,
    },

    /// Add one or more labels to an issue
    Label {
        /// Issue ID
        id: u32,
        /// Labels to add
        #[arg(required = true)]
        tags: Vec<String>,
    },

    /// Delete an issue entirely
    Delete {
        /// Issue ID
        id: u32,
    },
}
