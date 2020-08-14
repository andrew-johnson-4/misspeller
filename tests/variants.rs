use misspeller::misspell;

#[test]
fn variant1() {
   assert!( misspell("㐵").iter().any(|s| s=="儒") );
   assert!( misspell("儒").iter().any(|s| s=="㐵") );
}
