use misspeller::misspell;

#[test]
fn slur1() {
   assert!( misspell("aaa").iter().any(|s| s=="a") );
   assert!( misspell("aaaa").iter().any(|s| s=="a") );
   assert!( misspell("aaaa").iter().any(|s| s=="aa") );
   assert!( misspell("aabb").iter().any(|s| s=="a") );
   assert!( misspell("aabb").iter().any(|s| s=="aa") );
   assert!( misspell("aabb").iter().any(|s| s=="b") );
   assert!( misspell("aabb").iter().any(|s| s=="bb") );
}
