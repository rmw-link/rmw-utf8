use aho_corasick::{AhoCorasick, AhoCorasickBuilder, MatchKind};
use bit_vec::BitVec;
use huffman_compress::{Book, CodeBuilder, Tree};
use lazy_static::*;
use speedy::{Endianness, Readable};
use std::io::{prelude::*, Cursor};
use std::str::from_utf8_unchecked;
use utf8_width::get_width;

type VecStrU64 = Vec<(String, u64)>;

#[derive(PartialEq, Debug, Readable)]
struct WordCharCount(VecStrU64, Vec<(char, u64)>);

lazy_static! {
  pub static ref WORD_CHAR_COUNT: (VecStrU64, VecStrU64) = {
    let zst = include_bytes!("d.zst");
    let mut zstc = Cursor::new(zst);
    let mut decoder = ruzstd::StreamingDecoder::new(&mut zstc).unwrap();
    let mut bytes: Vec<u8> = Vec::with_capacity(zst.len() * 5);
    decoder.read_to_end(&mut bytes).unwrap();

    let wcc = WordCharCount::read_from_buffer_with_ctx(Endianness::LittleEndian, &bytes).unwrap();

    let mut word_li = wcc.0;
    let char_li = wcc.1;
    let mut char_vec = Vec::with_capacity(char_li.len());
    for (k, v) in char_li {
      if k == '\n' {
        word_li.push(("\n\n".into(), v / 4));
      }
      char_vec.push((k.to_string(), v));
    }

    (word_li, char_vec)
  };
  pub static ref G: (AhoCorasick, Book<&'static [u8]>, Tree<&'static [u8]>) = {
    let (word_li, char_li) = &*WORD_CHAR_COUNT;

    let mut weights = Vec::with_capacity(word_li.len() + char_li.len());
    for (k, v) in word_li.iter().chain(char_li.iter()) {
      weights.push((k.as_bytes(), *v));
    }

    let (book, tree) = CodeBuilder::from_iter(weights.into_iter()).finish();

    let ac = AhoCorasickBuilder::new()
      .match_kind(MatchKind::LeftmostFirst)
      .build(
        [&b"\r\n"[..]]
          .iter()
          .chain(&word_li.iter().map(|x| x.0.as_bytes()).collect::<Vec<_>>()),
      );

    (ac, book, tree)
  };
}

pub fn init() {
  lazy_static::initialize(&WORD_CHAR_COUNT);
  lazy_static::initialize(&G);
}

pub fn encode(input: &[u8]) -> Vec<u8> {
  let (ac, book, _) = &*G;

  let mut buffer = BitVec::new();
  let mut pos = 0;

  macro_rules! encode_char {
    ($max:ident) => {
      while pos < $max {
        let n = get_width(input[pos]);
        if n == 0 {
          pos += 1;
          continue;
        }
        let pos_next = pos + n;
        let mut c = unsafe { input.get_unchecked(pos..pos_next) };
        if n == 1 && c == b"\r" {
          c = b"\n";
        }
        pos = pos_next;
        let _ = book.encode(&mut buffer, &c);
      }
    };
  }

  for mat in ac.find_iter(input) {
    let start = mat.start();
    let end = mat.end();
    encode_char!(start);
    pos = end;
    let c = {
      if mat.pattern() == 0 {
        &b"\n"[..]
      } else {
        unsafe { input.get_unchecked(start..end) }
      }
    };
    let _ = book.encode(&mut buffer, c);
  }

  let len = input.len();
  encode_char!(len);

  buffer.push(true);

  buffer.to_bytes()
}

pub fn decode(input: &[u8]) -> String {
  // 解压缩
  let tree = &(G.2);
  let mut bits = BitVec::from_bytes(input);
  let mut len = bits.len();

  while len != 0 {
    len -= 1;
    if bits[len] {
      break;
    }
  }
  unsafe { bits.set_len(len) };

  let mut result = Vec::with_capacity(len);

  for i in tree.unbounded_decoder(&bits) {
    for j in i {
      result.push(*j);
    }
  }

  unsafe { from_utf8_unchecked(&result) }.to_string()
}

/*
fn main() -> Result<(), Box<dyn Error>> {
  for input in [
    "市场上绝大多数人基本上都仅仅着眼当前盛况的外象，而没有思考背后的根本原因。",
    "2006美版7.7分爱情《触不到的恋人》BD1080p.中文字幕",
  ] {
    let compressed = encode(input.as_bytes());
    println!(
      "\n{:?}\nbytes {} -> {} compresse ratio = {:.2}%",
      &compressed,
      input.len(),
      compressed.len(),
      100.0 * (compressed.len() as f64 / input.len() as f64),
    );

    println!("{}", input);
    println!("{}", decode(&compressed));
  }
  Ok(())
}
*/
