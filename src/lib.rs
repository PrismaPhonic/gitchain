//! # Gitchain
//!
//! Gitchain is a tool that will issue git commits with custom hash prefixes. The default prefix
//! is 000000 (six zeroes), but can be adjusted by passing in the -p or --prefix flag with a custom
//! prefix.
//!
//! ## Methodology
//!
//! Gitchain works very similarly to how blockchain miner's operate.  When you issue a commit using
//! gitchain, in the process you create a miner.  A miner will try to hash your commit along with
//! an incrementing value called a `nonce`. This random value is appended to the end of the commit, causing the
//! hash to change.  Once a `nonce` has been discovered which when hashed with the commit causes the
//! resulting hash to have the desired prefix, the miner will return the resulting blob (commit contents)
//! which will include the nonce in it.
//!
//! The committer will then use this to issue the desired commit with the correct hash.
//!
//! For more details about the flow of data in the application:
//!
//! ![Gitchain Flow Diagram](../../images/Gitchain-Flow-Diagram.png)
//!
//! ## Install
//!
//! To install this program globally, `cd` into the `gitchain-tool` folder and run:
//!
//! ```console
//! $ cargo install --path .
//! ```
//!
//! If you do not have cargo installed, simply follow these install instructions.
//!
//! [Rust & Cargo installation](https://rustup.rs/)
//!
//! ## Use
//!
//! This terminal application was designed to be interactive and like most shell applications you can
//! ask it directly for help.
//!
//! That being said let's go over the commands you can use with `gitchain`.
//!
//! ### Add
//!
//! The add subcommand is simply a passthrough to `git add` and is a convenience method.  Use it exactly
//! how you would use `git add`:
//!
//! ```console
//! $ gitchain add .
//! ```
//!
//! ### Commit
//!
//! If you would like to issue a git commit that will have a custom prefix of `000000`, simply use
//! `gitchain` the same way you would use `git`:
//!
//! ```console
//! $ gitchain commit -m "Commit message."
//! ```
//!
//! This assumes that you have files staged for commiting.  Please use the `add` subcommand if you do not.
//!
//! You can pass in a custom hash prefix as well:
//!
//! ```console
//! $ gitchain commit -p 010101 -m "Commit message."
//! ```
//!
//! By default `gitchain` will use the current directory as the root directory for the git repo.
//! If you would like to supply an alternate path, simply pass it using the -r or --repository flag:
//!
//! ```console
//! $ gitchain commit -r ~/git/custom_folder -m "Commit message"
//! ```
//!
//! ## Testing
//!
//! To run tests simply change into the root directory for the crate and run:
//!
//! ```console
//! $ cargo test
//! ```
//!
//! The integration test is currently set to ignore.  To run it simply pass the ignored flag:
//!
//! ```console
//! $ cargo test -- --ignored
//! ```
//!
//! ## Benchmarking
//!
//! To run criterion benchmarks simply cd into the root directory and run:
//!
//! ```console
//! $ cargo bench
//! ```
//!
//! In my testing on my personal computer I found that average solve time for six zeroes was 1.8 seconds.
//!
//! **WARNING: CRITERION WILL RUN 5050 LOOP ITERATIONS WITH A WARM UP PHASE.  THIS IS AN EXTREMELY
//! LONG BENCHMARK TO GET A STATISTICAL AVERAGE THAT IS MEANINGFUL**
//!
//! ## Performance Data
//!
//! After running criterion for roughly 2 hours on a six zero prefix test, I got the following results:
//!
//! ![Mining Benchmarks](../../images/Gitchain-Benchmarks.png)

#[macro_use]
extern crate failure;
#[macro_use]
extern crate serde_derive;

extern crate serde;

/// committer contains the Committer struct which kicks off mining and issues a commit whose hash
/// prefix will match the target.
pub mod committer;
/// custom in-house errors that we translate to from other errors received by external crates.
pub mod errors;
/// hasher contains methods for hashing a blob.
pub mod hasher;
/// miner contains the Miner struct which handles solving the Proof of Work in parallel.
pub mod miner;
/// options contains Structopt enum for parsing terminal commands and providing helpful menus.
pub mod options;
/// writer contains methods for building and manipulating git blobs.
pub mod writer;

use crate::committer::Committer;
use crate::errors::GitTerminalError;
pub use crate::options::{Options, Opts};
use std::process::Command;

use failure::Error;

/// Calling this function from a binary program will cause it to match on the commands
/// passed by the user, and run the appropriate internal functions.
pub fn run(config: Opts) -> Result<(), Error> {
    match config {
        Opts::Commit { repo, msg, prefix } => commit(Options { repo, msg, prefix }),
        Opts::Add { path } => add(path),
    }
}

fn commit(opts: Options) -> Result<(), Error> {
    let mut committer = Committer::new(opts)?;
    committer.commit()?;
    println!("Successfully committed with desired prefix.");
    Ok(())
}

fn add(path: String) -> Result<(), Error> {
    Command::new("git")
        .args(&["add", &path])
        .output()
        .map_err(|_| GitTerminalError::AddError {})?;

    Ok(())
}
