use std::collections::HashSet;

static KEYBOARDS: [&str; 1] = [
   "abcdefghijklmnopqrstuvwxyz",
];

pub fn detect_keyboard(_s: &str) -> &str {
   KEYBOARDS[0]
}

pub fn typos(s: &str) -> Vec<String> {
   let kb = detect_keyboard(s);
   let mut ts = Vec::new();
   for oi in 0..s.len() {
      //omit key
      ts.push(format!("{}{}", &s[..oi], &s[oi+1..]));
   }
   for oi in 0..s.len() {
      //replace key
      for k in kb.chars() {
         ts.push(format!("{}{}{}", &s[..oi], k, &s[oi+1..]));
      }
   }
   for oi in 0..s.len() {
      //add key
      for k in kb.chars() {
         ts.push(format!("{}{}{}", &s[..oi], k, &s[oi..]));
      }
   }
   ts
}

pub fn misspell(s: &str) -> HashSet<String> {
   let mut ms = HashSet::new();
   for w in typos(s).into_iter() {
      ms.insert(w);
   }
   if ms.contains(s) {
      ms.remove(s);
   }
   ms
}
