mod info;
mod new;
mod run;

use emoji_crafter::*;
use structopt::StructOpt;

/// Create, manage and export emojis
#[derive(StructOpt, Debug)]
#[structopt(name = "emoji")]
enum Opt {
    /// Create a new project
    New(new::Command),
    /// Run the current project
    Run(run::Command),
    /// Show information about the current project
    Info(info::Command),
}

fn main() {
    match Opt::from_args() {
        Opt::New(cmd) => cmd.run(),
        Opt::Run(cmd) => cmd.run(),
        Opt::Info(cmd) => cmd.run(),
    }
}
