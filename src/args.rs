use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Params {
    /// the file to use. Default: ~/.markctl
    #[arg(short, long)]
    pub file: Option<String>,
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// insert some marks
    Insert {
        /// the subject
        sub: String,
        /// the actual mark
        mark: i32,
        /// the class of the mark
        class: String,
        /// an optional comment
        comment: Option<String>,
    },
    /// list marks of a specific subject
    List {
        /// enable sorting
        #[arg(short, long)]
        sort: Option<Sort>,
        /// the subject to list
        sub: String,
    },
    /// compute the average
    Avg {
        /// the subject to average
        sub: String,
    },
    /// produce a complete report for all subjects
    Report,
}

#[derive(Eq, PartialEq, Clone, Copy, PartialOrd, Ord, ValueEnum)]
pub enum Sort {
    /// sort ascending
    Ascending,
    /// sort descending
    Descending,
}
