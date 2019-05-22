use crate::errors::MiningError;
use crate::hasher;
use crate::writer;
use failure::Error;
use rayon::prelude::*;

/// Miner carries out the work of mining for a correct nonce, that when appended to the commit causes
/// the resulting commit hash to have the desired prefix.
pub struct Miner {
    prefix: String,
    blob: String,
    max_nonce: u32,
}

impl Miner {
    /// Used to create a new instance of a miner, which should always be associated with
    /// a Committer.  In this sense a Miner always works for a Committer and if the Committer
    /// is cleaned up, the Miner should be as well.
    pub fn new(prefix: String, blob: String) -> Miner {
        let max_nonce = u32::max_value();

        Miner {
            prefix,
            blob,
            max_nonce,
        }
    }

    /// Solve method will attempt to find a nonce (random value) that when applied to the commit
    /// causes it's hash to have the desired prefix. This is carried out using a thread pool and a
    /// succeed fast strategy. If successful it will return a tuple of the blob and successful hash.
    pub fn solve(&mut self) -> Result<(String, String), Error> {
        let winning_nonce = self.find_correct_nonce()?;
        let blob = writer::append_nonce_to_blob(&self.blob, winning_nonce);
        let hash = hasher::hash_blob(&blob);

        return Ok((blob, hash));
    }

    fn find_correct_nonce(&mut self) -> Result<u32, Error> {
        let result = (0..self.max_nonce).into_par_iter().find_any(|nonce| {
            let blob = writer::append_nonce_to_blob(&self.blob, *nonce);
            let hash = hasher::hash_blob(&blob);
            hash.starts_with(&self.prefix)
        });

        let winning_nonce = result.ok_or(MiningError::SolveError {})?;

        Ok(winning_nonce)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_miner_finds_a_correct_nonce() -> Result<(), Error> {
        let blob = "tree TreeTest\n\
                    parent ParentTest\n\
                    author AuthorTest <test@test.com> 1454691142 -0000\n\
                    committer AuthorTest <test@test.com> 1454691142 -0000\n\n\
                    MessageTest";
        let prefix = "0000".to_string();
        let mut miner = Miner::new(prefix, blob.to_string());
        let (_, hash) = miner.solve()?;
        assert!(hash.starts_with("0000"));
        Ok(())
    }
}
