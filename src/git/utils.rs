/** This module contains helper functions for reading and writing git objects
*
**/
use anyhow::{Context, Ok, Result, bail, ensure};
use flate2::Compression;
use flate2::bufread::ZlibEncoder;
use flate2::read::ZlibDecoder;
use sha1::Digest;
use std::ffi::CStr;
use std::io::{BufRead, Write};
use std::io::{BufReader, Cursor};
use std::{fmt, io::Read, path::Path};

pub(crate) enum GitObjectType {
    Blob,
    Tree,
    Commit,
}

impl fmt::Display for GitObjectType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GitObjectType::Blob => write!(f, "blob"),
            GitObjectType::Tree => write!(f, "tree"),
            GitObjectType::Commit => write!(f, "commit"),
        }
    }
}

impl TryFrom<&str> for GitObjectType {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        match value {
            "blob" => Ok(Self::Blob),
            "tree" => Ok(Self::Tree),
            "commit" => Ok(Self::Commit),
            _ => bail!("invalid kind: {value}"),
        }
    }
}

// TODO: I think this causes extra uneeded allocations
impl Into<Vec<u8>> for GitObjectType {
    fn into(self) -> Vec<u8> {
        match self {
            GitObjectType::Blob => "blob".as_bytes().to_vec(),
            GitObjectType::Tree => "tree".as_bytes().to_vec(),
            GitObjectType::Commit => "commit".as_bytes().to_vec(),
        }
    }
}

// the main idea behind this is that different methods will return a git object with the correct
// kind, and a reader that you can use to read the remaining bytes fr with the correct kind, and a
// reader that you can use to read the remaining bytes from.
pub(crate) struct GitObject<R> {
    pub(crate) kind: GitObjectType,
    pub(crate) expected_length: usize,
    pub(crate) reader: R,
}

impl GitObject<()> {
    // whenever we are traversing a tree and know the type of the file is not a dir, we can use
    // this function to load it in
    pub(crate) fn read_blob_from_file(
        filename: impl AsRef<Path>,
    ) -> Result<GitObject<impl BufRead>> {
        // make sure the file exists
        ensure!(
            Path::new(filename.as_ref()).exists(),
            "file to read blob from does not exist"
        );

        // open the file
        // TODO: Some filesystems allow non UTF-8 filenames
        let f = std::fs::File::open(filename.as_ref()).with_context(|| {
            format!(
                "open blob with hash {:?} for reading",
                filename.as_ref().to_str()
            )
        })?;

        // convert the file into a bufreader for improve performance
        let mut r = BufReader::new(f);

        // read everything into an in memory buffer
        let mut buf: Vec<u8> = Vec::new();
        r.read_to_end(&mut buf)
            .context("read git blob into memory")?;

        // get the length
        let length = buf.len();

        // convert the in memory buffer into a cursor so that it can be passed back
        let cur = Cursor::new(buf);

        Ok(GitObject {
            kind: GitObjectType::Blob,
            expected_length: length,
            reader: cur,
        })
    }

    pub(crate) fn read(hash: &str) -> Result<GitObject<impl BufRead>> {
        // first we need to split the hash into its dir and remaining bytes
        ensure!(hash.len() == 40, "hash length not 40 hex characters");
        let (dir, filename) = hash.split_at(2);
        let object_path = format!(".git/objects/{}/{}", dir, filename);

        // open the file
        let f = std::fs::File::open(&object_path)
            .with_context(|| format!("read object {object_path}"))?;

        // stream through our decoder, decoding bytes as they pass through
        let zd = ZlibDecoder::new(f);

        // wrap the decoder in a bufread so that the reads are a bit cheaper
        let mut br = BufReader::new(zd);

        // if we read until the nukll byte, we have both the type and the length of the payload
        let mut buf: Vec<u8> = Vec::with_capacity(64);
        let type_and_length = {
            br.read_until(0, &mut buf)
                .context("read type and length into buf")?;

            CStr::from_bytes_with_nul(&buf).context("convert header into CStr")
        }?;

        // because the header should be valid ASCII, and ASCII is a subset of UTF-8, we should be
        // able to convert the header into UTF-8
        let (kind, length) = type_and_length
            .to_str()
            .context("convert header CStr into UTF-8")?
            .split_once(' ')
            .context("split header on space to get kind and length")?;

        // parse the kind
        let kind: GitObjectType = kind
            .try_into()
            .context("convert kind str into kind object")?;

        // parse the length
        let length: usize = length
            .parse::<usize>()
            .context("convert length str into usize")?;

        Ok(GitObject {
            kind,
            expected_length: length,
            reader: br,
        })
    }
}

impl<R> GitObject<R>
where
    R: Read,
{
    // TODO: A better way of doing this would be to create
    pub fn write(mut self, mut w: impl Write) -> Result<[u8; 20]> {
        // before we can write the bytes to storage, need the hash of the uncompressed header +
        // bytes.
        let mut buf: Vec<u8> = Vec::new();

        // write the kind,length, and null byte using the display impl for kind
        write!(buf, "{} {}\0", self.kind, self.expected_length)
            .context("write kind, length, and null byte to buffer")?;

        // write the rest of the bytes
        std::io::copy(&mut self.reader, &mut buf).context("copy bytes from reader to writer")?;

        // now we can compute the hash of the buffer
        let cur = Cursor::new(&buf);
        let mut encoder = ZlibEncoder::new(cur, Compression::default());
        let mut res: Vec<u8> = Vec::new();
        encoder
            .read_to_end(&mut res)
            .context("read sha1 bytes from zlib encoder")?;

        // now we can hash our buf
        let mut sha1_hasher = sha1::Sha1::new();
        sha1_hasher.update(&buf[..]);
        let hash = sha1_hasher.finalize();

        // finally we write the compressed bytes to file
        w.write_all(&res)
            .context("write compressed bytes to writer")?;

        Ok(hash.into())
    }

    pub fn write_to_objects(self) -> Result<[u8; 20]> {
        // create our in memory buffer we are writing to
        let mut buf: Vec<u8> = Vec::new();

        // write our object
        let hash = self.write(&mut buf).context("writing object to buffer")?;
        let hash_str = hex::encode(&hash);
        let (dir, filename) = hash_str.split_at(2);

        // create the directories needed
        let base_dir = format!(".git/objects/{}", dir);
        std::fs::create_dir_all(&base_dir)
            .with_context(|| "create directory {base_dir} to write object")?;

        // write the contents of the buffer to the file
        let file_path = format!("{}/{}", base_dir, filename);
        let mut f = std::fs::File::create(&file_path)
            .with_context(|| format!("open object for reading: {file_path}"))?;

        f.write_all(&buf)
            .with_context(|| format!("wrote buffer to {file_path}"))?;

        Ok(hash)
    }
}
