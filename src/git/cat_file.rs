use anyhow::{Result, ensure};
use std::io::Write;
use tracing::debug;

pub enum PrettyPrint {
    Yes,
    No,
}

impl From<&bool> for PrettyPrint {
    fn from(value: &bool) -> Self {
        match value {
            false => Self::No,
            true => Self::Yes,
        }
    }
}

pub fn run(hash: &str, pretty_print: PrettyPrint, mut w: impl Write) -> Result<()> {
    todo!()
}
