use misspeller::{misspell,ateji_mistakes};

#[test]
fn ateji1() {
   let w = "二本";
   println!("{:?}", ateji_mistakes(w));
   assert!( ateji_mistakes(w).iter().any(|s| s=="仁本") );
}

#[test]
fn ateji2() {
   let w = "二本";
   assert!( misspell(w).iter().any(|s| s=="仁本") );
}
