use misspeller::misspell;

#[test]
fn typo1() {
   println!("misspell hello");
   for s in misspell("hello").iter() {
      println!("{}", s);
   }
}
