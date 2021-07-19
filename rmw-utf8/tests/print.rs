use speedy::{Endianness, Readable};
use std::char;
use std::error::Error;
use std::io::prelude::*;
use std::io::Cursor;

#[derive(PartialEq, Debug, Readable)]
struct WordCharCount(Vec<(String, u64)>, Vec<(char, u64)>);

#[test]
fn main() -> Result<(), Box<dyn Error>> {
  //return Ok(());
  let wcc: (Vec<(String, u64)>, Vec<(String, u64)>) = {
    let zst = include_bytes!("../src/d.zst");
    let mut zstc = Cursor::new(zst);
    let mut decoder = ruzstd::StreamingDecoder::new(&mut zstc).unwrap();
    let mut bytes: Vec<u8> = Vec::with_capacity(zst.len() * 5);
    decoder.read_to_end(&mut bytes).unwrap();

    let wcc = WordCharCount::read_from_buffer_with_ctx(Endianness::LittleEndian, &bytes).unwrap();
    (
      wcc.0,
      wcc
        .1
        .iter()
        .map(|(k, v)| (k.to_string(), *v))
        .collect::<Vec<_>>(),
    )
  };
  let mut li = wcc.0.clone();
  li.sort_by(|a, b| a.1.cmp(&b.1));
  for (pos, i) in li.iter().enumerate() {
    println!("{} {:?}", pos, i);
  }
  println!("\n{:?}", &wcc.1[..10]);
  Ok(())
}
