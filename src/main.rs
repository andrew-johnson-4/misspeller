use misspeller::misspell;

fn main() {
   if let Some(word) = std::env::args().last() {
      for ms in misspell(&word).iter() {
         println!("{}", ms);
      }
   }
}
