use rmw_utf8::{decode, encode};

#[test]
fn main() {
  let txt = "测试\r\n一下\r就\n一下";
  let compressed = encode(&txt.as_bytes());
  let decompressed = decode(&compressed[..]);
  println!("{:?}", decompressed);
}
