use anyhow::Result;
use bytes::Bytes;
use std::io::Write;

/// This module contains helper functions for reading and writing git objects

pub enum GitObjectType {
    Blob { buf: Bytes },
}

impl Write for GitObjectType {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        todo!()
    }

    fn flush(&mut self) -> std::io::Result<()> {
        todo!()
    }
}

pub fn read_file(hash: String) -> Result<Bytes> {
    todo!()
}

pub fn convert_file(buf: Bytes) -> Result<String> {
    todo!()
}

pub fn write_file(s: &str) -> Result<()> {
    todo!()
}
