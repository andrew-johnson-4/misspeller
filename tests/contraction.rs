use misspeller::misspell;

#[test]
fn typo1() {
   let w = "can't";
   assert!( misspell(w).iter().any(|s| s=="ca't") );
   assert!( misspell(w).iter().any(|s| s=="cann't") );
   assert!( misspell(w).iter().any(|s| s=="can'") );
   assert!( misspell(w).iter().any(|s| s=="can'tt") );
   assert!( misspell(w).iter().any(|s| s=="cant'") );
   assert!( misspell(w).iter().any(|s| s=="ca'nt") );
}
