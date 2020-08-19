use misspeller::misspell;

#[test]
fn typo1() {
   let w = "hello";
   assert!( misspell(w).iter().any(|s| s=="heelo") );
   assert!( misspell(w).iter().any(|s| s=="helo") );
   assert!( misspell(w).iter().any(|s| s=="hllo") );
   assert!( misspell(w).iter().any(|s| s=="helllo") );
   assert!( misspell(w).iter().any(|s| s=="helloo") );
   assert!( misspell(w).iter().any(|s| s=="hell") );
   assert!( misspell(w).iter().any(|s| s=="hlelo") );
   assert!( misspell(w).iter().any(|s| s=="ehllo") );
   assert!( misspell(w).iter().any(|s| s=="helol") );
}
