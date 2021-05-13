mod cli;
mod tasks;

use anyhow::anyhow;
use std::path::PathBuf;
use structopt::StructOpt;

fn find_default_journal_file() -> Option<PathBuf> {
    home::home_dir().map(|mut path| {
        path.push(".rusty-journal.json");
        path
    })
}

fn main() -> anyhow::Result<()> {
    use cli::{Action::*, CommandLineArgs};
    use tasks::Task;

    let CommandLineArgs {
        action,
        journal_file,
    } = CommandLineArgs::from_args();

    let journal_file = journal_file
        .or_else(find_default_journal_file)
        .ok_or_else(|| anyhow!("Failed to find journal file"))?;

    match action {
        Add { task: text } => tasks::add_task(journal_file, Task::new(text)),
        Done { position } => tasks::complete_task(journal_file, position),
        List => tasks::list_tasks(journal_file),
    }?;

    Ok(())
}
