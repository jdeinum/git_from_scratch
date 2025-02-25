use crate::git::utils::create_directory;
use crate::git::{convert_file, git_objects_dir_exists, read_file};
use anyhow::{Result, ensure};
use flate2::Compression;
use flate2::write::ZlibEncoder;
use sha1::{Digest, Sha1};
use std::io::Write;
use std::path::{Path, PathBuf};
use tracing::{debug, info, instrument};

#[instrument(err)]
pub fn init_git_repo(p: PathBuf) -> Result<String> {
    // first we create the .git directory
    create_directory(p.clone(), ".git/")?;

    // adjust the path for creating directories
    let new_path = {
        let mut x = p.clone();
        x.push(".git/");
        x
    };

    // now the .git/objects directory
    create_directory(new_path.clone(), "objects/")?;

    // now the .git/refs directory
    create_directory(new_path.clone(), "refs/")?;

    // now we'll create the HEAD file
    let mut head_file_path = new_path.clone();
    head_file_path.push("HEAD");
    std::fs::write(head_file_path, "ref: refs/heads/main\n")?;

    Ok(new_path.to_string_lossy().to_string())
}

#[instrument(err)]
pub fn init_git_repo_in_current_dir() -> Result<()> {
    let cur = std::env::current_dir()?;
    debug!("creating git repo in {cur:?}");
    init_git_repo(cur)?;
    Ok(())
}

// todo: pass in an output object instead of println
pub fn cat_git_object(hash: &str) -> Result<String> {
    ensure!(git_objects_dir_exists(), ".git/objects/ does not exist");

    // read the file
    let buf = read_file(hash)?;

    debug!("buf: {buf:?}");

    // convert the file into UTF8
    let content = convert_file(buf)?;

    Ok(content)
}

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
pub fn hash_git_object(filename: &Path, should_hash: StoreHash) -> Result<String> {
    // get the file bytes
    let file_bytes = std::fs::read(filename)?;

    // generate the sha hash of the file
    let hash = {
        let mut sha = Sha1::new();
        sha.update(&file_bytes);
        let res = sha.finalize();
        hex::encode(&res)
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
        f.write(&compressed_bytes)?;
        f.flush()?;
    }

    // return the hash
    Ok(hash)
}
