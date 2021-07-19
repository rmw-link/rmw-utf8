use blake3_file::hash;

#[test]
fn main() {
  let r = hash("/etc/passwd");
  println!(">>> {:?}", r);
}
