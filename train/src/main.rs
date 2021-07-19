use aho_corasick::{AhoCorasickBuilder, MatchKind};
use hashbrown::{HashMap, HashSet};
use jieba_rs::Jieba;
use ngrams::Ngram;
use rayon::iter::ParallelBridge;
use rayon::prelude::ParallelIterator;
use speedy::{Endianness, Readable, Writable};
use static_init::dynamic;
use std::convert::TryInto;
use std::io::{prelude::*, BufReader};
use std::path::{Path, PathBuf};
use std::sync::mpsc::sync_channel;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use std::{
  env,
  fs::{create_dir, File},
  str, thread,
};
use walkdir::WalkDir;
mod lossycounter;
use lossycounter::LossyCounter;

const CACHE_COUNTED: &str = "cache/counted";

#[dynamic]
pub static DIR: PathBuf = env::current_exe()
  .unwrap()
  .parent()
  .unwrap()
  .parent()
  .unwrap()
  .parent()
  .unwrap()
  .into();

#[dynamic]
pub static CACHE: PathBuf = (&*DIR).join("cache");

#[dynamic]
pub static DB: sled::Db = {
  let cache = &*CACHE;
  let _ = create_dir(cache);
  sled::open(cache).unwrap()
};

#[dynamic]
pub static VERSION: u32 = SystemTime::now()
  .duration_since(UNIX_EPOCH + Duration::from_secs(1625844173))
  .unwrap()
  .as_secs() as u32;

fn main() {
  println!("version {:?}", *VERSION);

  word_huffman();
  //unicode_huffman();
}

struct LineIter {
  exist: HashSet<String>,
  buf: std::io::Lines<BufReader<File>>,
}

impl LineIter {
  pub fn new(fp: &str) -> Self {
    let file = File::open(&fp).unwrap();
    LineIter {
      exist: HashSet::new(),
      buf: BufReader::new(file).lines(),
    }
  }
}

impl Iterator for LineIter {
  type Item = String;
  fn next(&mut self) -> Option<Self::Item> {
    let exist = &mut self.exist;
    loop {
      if let Some(line) = self.buf.next() {
        if let Ok(txt) = line {
          let txt = txt.trim();
          if txt.len() > 0 {
            let txt = txt.to_string();
            if None == exist.get(&txt) {
              exist.insert(txt.clone());
              return Some(txt);
            }
          }
        }
      } else {
        return None;
      }
    }
  }
}

fn txt_iter() -> impl Iterator<Item = String> {
  WalkDir::new("../txt")
    .follow_links(true)
    .into_iter()
    .filter_map(|e| e.ok())
    .filter_map(move |e| {
      let p = e.path();
      if let Some(ext) = p.extension() {
        if matches!(
          ext.to_str()?,
          "txt" | "md" | "html" | "css" | "js" | "py" | "ini" | "cpp"
        ) {
          return Some(p.display().to_string());
        }
      }
      None
    })
}

fn txt_line_iter() -> impl Iterator<Item = (String, LineIter)> {
  let mut n = 0;
  let now = Instant::now();

  txt_iter().filter_map(move |p| {
    n += 1;
    if n % 100 == 1 {
      println!(
        "{:.2} sec/txt {} {}",
        now.elapsed().as_secs() as f64 / n as f64,
        n,
        p
      );
    }
    return Some((p.clone(), LineIter::new(&p)));
  })
}

fn line_iter() -> impl Iterator<Item = LineIter> {
  txt_line_iter().filter_map(move |(_, p)| {
    return Some(p);
  })
}

fn word_huffman() {
  let len_rev = |a: &String, b: &String| b.len().cmp(&a.len());

  let epsilon = 0.00000003;
  let mut lc = LossyCounter::with_epsilon(epsilon);

  let pre_version = if let Some(v) = DB.get(&"").unwrap() {
    u32::from_ne_bytes(v[..4].try_into().unwrap())
  } else {
    0
  };
  if pre_version != 0 {
    let fp = Path::new(&*DIR).join(CACHE_COUNTED.to_owned() + ".zst");
    println!("加载缓存中 {}", fp.display().to_string());
    let mut file = File::open(&fp).unwrap();
    let mut decoder = ruzstd::StreamingDecoder::new(&mut file).unwrap();
    let mut bytes: Vec<u8> = Vec::new();
    decoder.read_to_end(&mut bytes).unwrap();
    let counted = LossyCounted::read_from_buffer_with_ctx(Endianness::LittleEndian, &bytes)
      .unwrap()
      .0;

    for (k, v) in counted {
      lc.add(k, v);
    }
  }

  println!("第1次扫描，统计潜在的高频ngram");
  let elements = {
    let (tx, recv) = sync_channel(8);
    let jieba = Jieba::new();

    thread::spawn(move || {
      txt_line_iter()
        .par_bridge()
        .map_with(tx, |tx, (txt_path, iter)| {
          let hash = blake3_file::hash(&txt_path).unwrap();
          let exist;
          if let Some(v) = DB.get(&hash).unwrap() {
            let v = u32::from_ne_bytes(v[..4].try_into().unwrap());
            if v <= pre_version {
              return;
            } else {
              exist = true;
            }
          } else {
            exist = false;
          }
          let mut wc = HashMap::new();
          for line in iter {
            let words = jieba
              .cut(&line, true)
              .iter()
              .map(|x| x.to_string())
              .collect::<Vec<_>>();
            for n in 2..std::cmp::min(words.len() + 1, 8) {
              for i in (&words).into_iter().map(|x| x.clone()).ngrams(n) {
                let i = i.join("");
                let c = i.len() + i.chars().count() + wc.get(&i).unwrap_or(&0);
                wc.insert(i, c);
              }
            }
            for i in words {
              let n = i.chars().count();
              if n > 1 {
                let c = i.len() + i.chars().count() + wc.get(&i).unwrap_or(&0);
                wc.insert(i, c);
              }
            }
            if wc.len() > 100000 {
              let _ = tx.send(wc);
              wc = HashMap::new();
            }
          }
          let _ = tx.send(wc);
          if !exist {
            let _ = DB.insert(hash, &(*VERSION).to_ne_bytes());
          }
        })
        .for_each(drop);
    });

    for map in recv {
      for (words, n) in map {
        lc.add(words, n);
      }
    }
    write(
      LossyCounted(lc.query(0.0).collect::<Vec<_>>()),
      CACHE_COUNTED,
    );
    let _ = DB.insert(&"", &(*VERSION).to_ne_bytes());
    let threshold = 0.0000003;

    let elements: Vec<_> = lc.query(threshold).collect();
    drop(lc);
    let mut elements: Vec<_> = elements.iter().map(|x| x.0.clone()).collect();
    elements.sort_by(len_rev);
    elements
  };

  println!("第2次扫描，用aho-corasick统计ngram的频率，假设『成人之美』和『成人之』都是高频词，那么这一步只会统计到成人之美");

  //这一步筛选32768个高频词
  let elements = {
    let ac = AhoCorasickBuilder::new()
      .match_kind(MatchKind::LeftmostFirst)
      .build(&elements);

    let mut count = HashMap::new();
    for txt in line_iter() {
      for line in txt {
        for mat in ac.find_iter(&line) {
          let id = mat.pattern();
          let start = mat.start();
          let end = mat.end();
          let s = &line[start..end];

          count.insert(
            id,
            s.chars().count() + end - start + count.get(&id).unwrap_or(&0),
          );
        }
      }
    }
    let mut count: Vec<_> = count.into_iter().collect();
    count.sort_by(|a, b| b.1.cmp(&a.1));

    let mut elements: Vec<_> = count[..std::cmp::min(32768, count.len() / 2)]
      .iter()
      .map(|x| elements[x.0].clone())
      .collect();

    elements.sort_by(len_rev);
    elements
  };

  println!("第3次扫描，统计词和字的出现次数");
  let ac = AhoCorasickBuilder::new()
    .match_kind(MatchKind::LeftmostFirst)
    .build(&elements);

  let mut word_count = HashMap::new();
  let mut char_count = HashMap::new();

  let content = str::from_utf8(include_bytes!("utf8.char")).unwrap(); //确保每个字符至少出现一次

  for i in content.chars() {
    char_count.insert(i, 1u64);
  }

  for txt in line_iter() {
    for line in txt {
      char_count.insert('\n', 1 + char_count.get(&'\n').unwrap_or(&0));

      let mut pos = 0;
      let line_len = line.len();
      if line_len == 0 {
        continue;
      }
      for mat in ac.find_iter(&line) {
        let id = mat.pattern();
        let start = mat.start();
        let end = mat.end();
        let s = &line[start..end];
        word_count.insert(
          id,
          (s.chars().count() + end - start) as u64 + word_count.get(&id).unwrap_or(&0),
        );
        let start = mat.start();
        for c in line[pos..start].chars() {
          char_count.insert(c, 1 + char_count.get(&c).unwrap_or(&0));
        }
        pos = mat.end();
      }
      if pos < line_len {
        for c in line[pos..].chars() {
          char_count.insert(c, 1 + char_count.get(&c).unwrap_or(&0));
        }
      }
    }
  }

  let mut word_count: Vec<_> = word_count
    .into_iter()
    .map(|x| (elements[x.0].clone(), x.1 / 2))
    .collect();
  word_count.sort_by(|a, b| b.0.len().cmp(&a.0.len()));

  let mut char_count: Vec<_> = char_count.into_iter().collect();
  char_count.sort_by(|a, b| b.1.cmp(&a.1));
  write(WordCharCount(word_count, char_count), "d");
}

fn write<T: Writable<Endianness>>(t: T, path: &str) {
  let p = Path::new(&*DIR).join(path.to_owned() + ".zst");
  let bytes = t.write_to_vec_with_ctx(Endianness::LittleEndian).unwrap();
  println!(
    "写入 {} 压缩前 大小 {:.2}MB",
    (&p).display().to_string(),
    bytes.len() as f64 / (1024.0 * 1024.0)
  );

  let f = File::create(p).unwrap();
  let mut encoder = zstd::stream::write::Encoder::new(f, 19)
    .unwrap()
    .auto_finish();
  std::io::copy(&mut std::io::Cursor::new(bytes), &mut encoder).unwrap();
}

#[derive(PartialEq, Debug, Writable, Readable)]
struct LossyCounted(Vec<(String, usize)>);

#[derive(PartialEq, Debug, Writable)]
struct WordCharCount(Vec<(String, u64)>, Vec<(char, u64)>);

/*
fn unicode_huffman() {
  let content = str::from_utf8(include_bytes!("utf8.char")).unwrap();
  let mut weights = HashMap::new();

  for i in content.chars() {
    weights.insert(i, 1u64);
  }

  let mut incr = |key| {
    if weights.contains_key(&key) {
      *weights.get_mut(&key).unwrap() += 1;
    }
  };

  for p in line_iter() {
    let txt = fs::read_to_string(p).unwrap();
    txt.chars().for_each(|x| incr(x))
  }

  let mut buf = BytesMut::with_capacity(content.len() * (4 + 8));

  for (c, n) in weights {
    buf.put_u32(c.into());
    buf.put_u64(n);
  }
  {
    let mut file = File::create("unicode.huffman").unwrap();
    file.write_all(&buf.split()).unwrap();
  }
}
*/
