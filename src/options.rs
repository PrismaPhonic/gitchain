extern crate structopt;
use structopt::StructOpt;
use std::path::PathBuf;

#[derive(StructOpt, Debug)]
#[structopt(
name = "gitchain",
about = "The custom git commit hash prefixer",
)]
/// You can use gitchain to create a git commit with a git hash that is prefixed with zeros.
pub enum Opts {
    #[structopt(name = "commit")]
    /// Git commits with a custom hash prefix.
    Commit {
        /// Provide a path to the base directory of your github repository.
        #[structopt(short = "r", long = "repository", parse(from_os_str), default_value = ".")]
        repo: PathBuf,

        /// Message flag allows you to provide a commit message.
        #[structopt(short = "m", long = "message")]
        msg: String,

        /// Pass in a custom prefix for the git hash.
        #[structopt(short = "p", long = "prefix", default_value = "000000")]
        prefix: String,
    },

    #[structopt(name = "add")]
    /// Same as the `git add` command.
    Add {
        /// Provide a path to the directory from which you would like to recursively add unstaged files.
        path: String,
    },
}

/// For ease of use in the application as a translation from commands
pub struct Options {
    pub repo: PathBuf,
    pub msg: String,
    pub prefix: String,
}