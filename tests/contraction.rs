use misspeller::misspell;

#[test]
fn contraction1() {
   let w = "can't";
   assert!( misspell(w).iter().any(|s| s=="ca't") );
   assert!( misspell(w).iter().any(|s| s=="cann't") );
   assert!( misspell(w).iter().any(|s| s=="can'") );
   assert!( misspell(w).iter().any(|s| s=="can'tt") );
   assert!( misspell(w).iter().any(|s| s=="cant'") );
   assert!( misspell(w).iter().any(|s| s=="ca'nt") );
}

#[test]
fn contraction2() {
   let w = "'aight";
   misspell(w);
}

#[test]
fn contraction3() {
   let w = "dye'";
   misspell(w);
}
