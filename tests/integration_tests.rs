use std::fs::File;
use std::path::{PathBuf, Path};
use tempfile::TempDir;
use std::error::Error;

use gitchain::options::Opts;
use git2::Repository;

fn generate_terminal_opts_for_commit(repo_path: PathBuf, prefix: &str) -> Opts {
    Opts::Commit {
        repo: repo_path,
        msg: "Test Commit".to_string(),
        prefix: prefix.to_string(),
    }
}

#[test]
#[ignore]
fn test_commit() -> Result<(), Box<dyn Error>> {
    let td = TempDir::new()?;
    let td_path = td.path().to_path_buf();
    let repo = Repository::init(td.path())?;

    let prefix = "000000";
    let terminal_opts = generate_terminal_opts_for_commit(td_path.clone(), &prefix);

    {
        let mut index = repo.index()?;
        let filepath = &td.path().join("test.txt");
        File::create(&filepath)?;
        index.add_path(Path::new("test.txt"))?;
        index.write().unwrap();

        gitchain::run(terminal_opts)?;
    }

    let repository = Repository::open(td_path)?;
    let head = repository.revparse_single("HEAD")?;
    let head_id = format!("{}", head.id());

     assert!(head_id.starts_with(&prefix));

    Ok(())
}
