mod info;
mod new;
mod run;

use emoji_crafter::*;
use structopt::StructOpt;

/// Create, manage and export emojis
#[derive(StructOpt, Debug)]
#[structopt(name = "emoji")]
enum Opt {
    /// Create a new emojiset
    New(new::Command),
    /// Export emoji from the current emojiset
    Run(run::Command),
    /// List emoji information from the current emojiset
    Info(info::Command),
}

fn main() {
    match Opt::from_args() {
        Opt::New(cmd) => cmd.run(),
        Opt::Run(cmd) => cmd.run(),
        Opt::Info(cmd) => cmd.run(),
    }
}
