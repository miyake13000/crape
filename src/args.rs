use clap::{Args as ArgsTrait, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Args {
    #[command(subcommand)]
    pub action: Action,
}

#[derive(Debug, Subcommand)]
pub enum Action {
    /// Initialize version controlled environment
    Init,

    /// Run command in version controlled environment
    Run(RunArgs),

    /// Show commit logs
    Log(LogArgs),

    /// Record changes
    Commit(CommitArgs),

    /// List, create, or delete branches
    Branch(BranchArgs),

    /// Join two branches together
    Merge(TargetArgs),

    /// Show changes between commits
    Diff,

    /// Delete uncommited chenges
    Clean,

    /// Reset current HEAD specified state
    Reset(TargetArgs),

    /// Switch branches
    Checkout(TargetArgs),
}

#[derive(Debug, ArgsTrait)]
pub struct RunArgs {
    /// Command to execute
    pub command: Option<String>,

    /// Arguments of command
    #[arg(allow_hyphen_values = true)]
    pub args: Option<Vec<String>>,
}

#[derive(Debug, ArgsTrait)]
pub struct LogArgs {
    /// Show all branch
    #[arg(short, long)]
    pub all: bool,
}

#[derive(Debug, ArgsTrait)]
pub struct CommitArgs {
    /// Commit message
    #[arg(short, long)]
    pub message: Option<String>,
}

#[derive(Debug, ArgsTrait)]
pub struct BranchArgs {
    /// Branch name you want to create or delete
    pub branch_name: Option<String>,

    /// Delete specified branch
    #[arg(short, long, requires = "branch_name")]
    pub delete: bool,

    /// Show all branches
    #[arg(short, long, exclusive = true)]
    pub all: bool,
}

#[derive(Debug, ArgsTrait)]
pub struct TargetArgs {
    /// Commit ID or branch
    pub query: String,
}
