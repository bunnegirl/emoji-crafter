mod build;
mod info;
mod new;
mod watch;

use structopt::StructOpt;

/// Create, manage and export emojis
#[derive(StructOpt, Debug)]
#[structopt(name = "emoji")]
enum Opt {
    /// Create a new emojiset
    New(new::Command),
    /// Export emoji from the current emojiset
    Build(build::Command),
    /// List emoji information from the current emojiset
    Info(info::Command),
    /// Watch project assets for changes and then rebuild
    Watch(watch::Command),
}

fn main() {
    match Opt::from_args() {
        Opt::New(cmd) => cmd.run(),
        Opt::Build(cmd) => cmd.run(),
        Opt::Info(cmd) => cmd.run(),
        Opt::Watch(cmd) => cmd.run(),
    }
}
