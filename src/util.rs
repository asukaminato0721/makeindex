use std::{
    io::{self, Write},
    path::{Path, PathBuf},
};

use anyhow::{Result, anyhow};

pub fn derive_output_path(input: &Path, new_extension: &str) -> PathBuf {
    let mut path = input.to_path_buf();
    path.set_extension(new_extension);
    path
}

pub fn decode_latin1(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(bytes.len());
    for &b in bytes {
        out.push(b as char);
    }
    out
}

pub fn encode_latin1(text: &str) -> Result<Vec<u8>> {
    let mut buf = Vec::with_capacity(text.len());
    for ch in text.chars() {
        if (ch as u32) > 0xFF {
            return Err(anyhow!("Character U+{:04X} outside 8-bit range", ch as u32));
        }
        buf.push(ch as u8);
    }
    Ok(buf)
}

pub fn write_latin1<W: Write>(writer: &mut W, text: &str) -> io::Result<()> {
    let bytes =
        encode_latin1(text).map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))?;
    writer.write_all(&bytes)
}
