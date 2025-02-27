use anyhow::Result;
use flate2::Compression;
use flate2::write::ZlibEncoder;
use sha1::{Digest, Sha1};
use std::io::Write;
use std::path::Path;
use tracing::info;

pub enum StoreHash {
    No,
    Yes,
}

impl From<bool> for StoreHash {
    fn from(value: bool) -> Self {
        match value {
            true => Self::Yes,
            false => Self::No,
        }
    }
}

// TODO: Pass in an ouput object to print to
pub fn hash_git_object(filename: &Path, should_hash: StoreHash, mut w: impl Write) -> Result<()> {
    // get the file bytes
    let file_bytes = std::fs::read(filename)?;

    // generate the sha hash of the file
    let hash = {
        let mut sha = Sha1::new();
        sha.update(&file_bytes);
        let res = sha.finalize();
        hex::encode(res)
    };

    // seperate into the directory and filename
    let (dir, filename) = hash.split_at(2);

    // write the header to the buffer
    let header = format!("blob {}\0", file_bytes.len());

    // compress it using zlib
    let compressed_bytes = {
        let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(header.as_bytes())?;
        encoder.write_all(&file_bytes)?;
        encoder.finish()
    }?;

    if let StoreHash::Yes = should_hash {
        // make sure the directory exists
        // TODO: I think we should probably recurse until the root .git directory because you
        // should be able run this command from any subdirectory of the root directory
        if !Path::new(&format!(".git/objects/{dir}")).is_dir() {
            info!("Directory .git/objects/{dir} does not exist, creating");
            std::fs::create_dir(format!(".git/objects/{dir}"))?;
        }

        // write the bytes
        let mut f = std::fs::File::create(format!(".git/objects/{}/{}", dir, filename))?;
        f.write_all(&compressed_bytes)?;
        f.flush()?;
    }

    // return the hash
    write!(w, "{}", hash)?;
    Ok(())
}
