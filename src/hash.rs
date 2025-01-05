use tiger::{Digest, Tiger};

pub fn hash_file(data: impl AsRef<[u8]>) -> Vec<u8> {
    let mut hasher = Tiger::new();
    hasher.update(data);
    let result = hasher.finalize();

    result.to_vec()
}
