use crypto::digest::Digest;
use crypto::sha1::Sha1;
use crate::writer;

/// Hashes a blob by first using the writer struct to attach the necessary header to
/// the blob, and then hash it using sha1, and returns the hash.
pub fn hash_blob(blob: &str) -> String {
    let full_blob = writer::prepend_header_to_blob(&blob);
    sha1_hash(&full_blob)
}

fn sha1_hash(input: &str) -> String {
    let mut sha1_hasher = Sha1::new();
    sha1_hasher.input_str(&input);
    sha1_hasher.result_str()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hasher_correctly_hashes_blob() {
        let blob = "tree TreeTest\n\
                                parent ParentTest\n\
                                author AuthorTest <test@test.com> 1454691142 -0000\n\
                                committer AuthorTest <test@test.com> 1454691142 -0000\n\n\
                                MessageTest";
        let hash = hash_blob(blob);
        assert_eq!(hash, "9dd04fe53bacc70ecd3da2a7880c001c5bd2ff4a");
    }
}
