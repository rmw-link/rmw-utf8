#![feature(new_uninit)]
#![feature(seek_stream_len)]

use anyhow::Result;
use std::io::{BufReader, Read};
use std::{fs::File, io::Seek};

const MB_16_USIZE: usize = 1024 * 1024 * 16;
const MB_16: u64 = MB_16_USIZE as u64;

pub fn hash(path: &str) -> Result<[u8; 32]> {
  let mut file = File::open(path)?;
  let mut h = blake3::Hasher::new();
  let len = file.stream_len()?;
  let mut reader = BufReader::new(file);
  let mut buf = unsafe { Box::<[u8; MB_16_USIZE]>::new_uninit().assume_init() };

  if len > MB_16 {
    if len / 2 > MB_16 {
      loop {
        let n = reader.read(&mut buf[..])?;
        if n == 0 {
          break;
        }
        h.update_with_join::<blake3::join::RayonJoin>(&buf[..n]);
      }
    } else {
      loop {
        let n = reader.read(&mut buf[..])?;
        if n == 0 {
          break;
        }
        h.update(&buf[..n]);
      }
    }
  } else {
    let n = reader.read(&mut buf[..])?;
    h.update(&buf[..n]);
  }

  Ok(*h.finalize().as_bytes())
}
