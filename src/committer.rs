use crate::options::Options;
use git2::Repository;
pub use crate::miner::Miner;
use crate::writer;
use std::io::Write;
use std::process::{Command, Stdio};
use crate::errors::{RepositoryError, IoError, GitTerminalError};

use failure::Error;
use std::env;
use std::path::PathBuf;

/// A Committer does the work of issuing a git commit whose hash will match
/// the desired prefix.
pub struct Committer {
    miner: Miner,
    working_dir: PathBuf,
}

impl Committer {
    /// Creates a new Committer, and also in the process a new Miner to be used by the
    /// Committer.
    pub fn new(options: Options) -> Result<Committer, Error> {
        let mut repo = Committer::get_repository(&options)?;
        let working_dir = repo.workdir()
            .ok_or(RepositoryError::WorkdirRetrievalError {})?.to_path_buf();

        let tree = Committer::create_tree(&mut repo)?;
        let parent = Committer::get_parent(&repo);
        let author = Committer::get_author(&repo)?;

        let blob = writer::generate_blob(
            tree,
            parent,
            author,
            options.msg,
            time::now(),
        )?;

        let miner = Miner::new(
            options.prefix,
            blob,
        );

        return Ok(Committer {
            miner,
            working_dir,
        })
    }

    /// This method can be called to commit files that have been staged.
    pub fn commit(&mut self) -> Result<(), Error> {
        let (blob, hash) = self.miner.solve()?;

        self.commit_blob(&blob)?;
        self.reset_head_to_hash(&hash)?;

        Ok(())
    }

    fn reset_head_to_hash(&self, hash: &str) -> Result<(), Error> {
        env::set_current_dir(&self.working_dir).is_ok();

        Command::new("git")
            .args(&["reset", "--hard", hash])
            .output()
            .map_err(|_| GitTerminalError::ResetHeadError {})?;

        Ok(())
    }

    fn commit_blob(&self, blob: &String) -> Result<(), Error> {
        env::set_current_dir(&self.working_dir).is_ok();

        let mut commit_command = Command::new("git")
            .args(&["hash-object", "-t", "commit", "-w", "--stdin"])
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .spawn()
            .map_err(|_| GitTerminalError::CommitObjectError {})?;

        let stdin = commit_command.stdin.as_mut()
            .ok_or(IoError::StdinOpenError {})?;

        stdin.write_all(blob.as_bytes())
            .map_err(|_| IoError::StdinWriteError {} )?;

        Ok(())
    }

    fn get_repository(options: &Options) -> Result<Repository, Error> {
        let repository = Repository::open(&options.repo)
            .map_err(|_| RepositoryError::OpenError {})?;

        Ok(repository)
    }

    fn get_author(repo: &Repository) -> Result<String, Error>{
        let signature = repo.signature()
            .map_err(|_| RepositoryError::SignatureRetrievalError {})?;

        let name = signature.name()
            .ok_or(RepositoryError::NameRetrievalError {})?;

        let email = signature.email()
            .ok_or(RepositoryError::EmailRetrievalError {})?;

        Ok(format!("{} <{}>", name, email))
    }


    fn create_tree(repository: &mut Repository) -> Result<String, Error> {
        let mut index = repository.index()
            .map_err(|_| RepositoryError::FindIndexError {})?;

        let tree = index.write_tree()
            .map_err(|_| RepositoryError::TreeWriteError {})?;

        return Ok(format!("{}", tree));
    }

    fn get_parent(repository: &Repository) -> Option<String> {
        if let Ok(head) = repository.revparse_single("HEAD") {
            return Some(format!("{}", head.id()));
        } else {
            return None;
        }
    }
}