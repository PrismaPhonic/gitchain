use failure::Error;
use crate::errors::WriterErrors;

/// `generate_blob` associative method first checks to see if there is a parent, and if so it will
/// write a blob for a standard commit.  If not, then we need to write an initial commit, so it will
/// generate a blob for an initial commit.
pub fn generate_blob(
    tree: String,
    parent: Option<String>,
    author: String,
    message: String,
    commit_time: time::Tm,
) -> Result<String, Error> {
    let formatted_time = commit_time.strftime("%s %z")
        .map_err(|_| WriterErrors::TimeFormatError {})?;

    let time_str = format!("{}", formatted_time);

    let blob = if let Some(p) = parent {
        generate_non_initial_blob(tree, p, author, message, time_str)
    } else {
        generate_initial_blob(tree, author, message, time_str)
    };

    Ok(blob)
}

fn generate_initial_blob(
    tree: String,
    author: String,
    message: String,
    commit_time: String,
) -> String {
    return format!("tree {}\n\
                       author {} {}\n\
                       committer {} {}\n\n\
                       {}",
                   tree,
                   author,
                   commit_time,
                   author,
                   commit_time,
                   message);
}

fn generate_non_initial_blob(
    tree: String,
    parent: String,
    author: String,
    message: String,
    commit_time: String,
) -> String {
    return format!("tree {}\n\
                       parent {}\n\
                       author {} {}\n\
                       committer {} {}\n\n\
                       {}",
                   tree,
                   parent,
                   author,
                   commit_time,
                   author,
                   commit_time,
                   message);
}

/// Appends a nonce to the end of the blob formatted as hexadecimal.  This is used to modify
/// the blobs hash as we solve the Proof of Work.
pub fn append_nonce_to_blob(blob: &str, nonce: u32) -> String {
    format!("{}\n{:08x}", blob, nonce)
}

/// Prepends the necessary header to the blob, which is necessary before we check the blobs
/// resulting hash, or the hash will be incorrect.
pub fn prepend_header_to_blob(blob: &str) -> String {
    format!("commit {}\0{}", blob.len(), blob)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_appending_nonce() {
        let blob = append_nonce_to_blob("test", 15);
        assert_eq!(blob, "test\n0000000f");
    }

    #[test]
    fn test_prepending_header() {
        let blob = prepend_header_to_blob("test");
        assert_eq!(blob, "commit 4\0test");
    }

    #[test]
    fn test_blob_generation_with_parent() -> Result<(), Error> {
        let tree = "TreeTest".to_string();
        let parent = Some("ParentTest".to_string());
        let author = "AuthorTest <test@test.com>".to_string();
        let message = "MessageTest".to_string();
        let commit_time = time::strptime("2016-02-05 16:52:22", "%Y-%m-%d %H:%M:%S")?;
        let blob = generate_blob(tree, parent, author, message, commit_time)?;

        let expected = "tree TreeTest\n\
                                parent ParentTest\n\
                                author AuthorTest <test@test.com> 1454691142 -0000\n\
                                committer AuthorTest <test@test.com> 1454691142 -0000\n\n\
                                MessageTest";
        assert_eq!(blob, expected);

        Ok(())
    }

    #[test]
    fn test_blob_generation_without_parent() -> Result<(), Error> {
        let tree = "TreeTest".to_string();
        let parent = None;
        let author = "AuthorTest <test@test.com>".to_string();
        let message = "MessageTest".to_string();
        let commit_time = time::strptime("2016-02-05 16:52:22", "%Y-%m-%d %H:%M:%S")?;
        let blob = generate_blob(tree, parent, author, message, commit_time)?;

        let expected = "tree TreeTest\n\
                                author AuthorTest <test@test.com> 1454691142 -0000\n\
                                committer AuthorTest <test@test.com> 1454691142 -0000\n\n\
                                MessageTest";
        assert_eq!(blob, expected);

        Ok(())
    }
}