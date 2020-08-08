use misspeller::misspell;

#[test]
fn typo1() {
   let w = "ô";
   assert!( misspell(w).iter().any(|s| s=="ôû") );
}
