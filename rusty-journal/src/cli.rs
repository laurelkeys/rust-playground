use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Action {
    /// Write a new task to the journal file.
    Add {
        #[structopt()]
        /// Task text.
        task: String,
    },
    /// Remove an entry from the journal file by its position.
    Done {
        #[structopt()]
        /// 1-based indexing position.
        position: usize,
    },
    /// List all tasks in the journal file.
    List,
}

#[derive(Debug, StructOpt)]
#[structopt(name = "Rusty Journal", about = "A command line to-do app")]
pub struct CommandLineArgs {
    #[structopt(subcommand)]
    pub action: Action,

    /// Use a different journal file.
    #[structopt(parse(from_os_str), short, long)]
    pub journal_file: Option<PathBuf>,
}
