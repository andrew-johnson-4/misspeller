use std::collections::HashSet;

static KEYBOARDS: [&str; 1] = [
   "abcdefghijklmnopqrstuvwxyz",
];
static KEYBOARD_VOWELS: [&str; 1] = [
   "aeiouy",
];
static KEYBOARD_CONSONANTS: [&str; 1] = [
   "bcdfghjklmnpqrstvwxz",
];

pub fn detect_keyboard(_s: &str) -> usize {
   0
}
pub fn detect_keyboard_layout(s: &str) -> &str {
   KEYBOARDS[detect_keyboard(s)]
}
pub fn detect_keyboard_vowels(s: &str) -> &str {
   KEYBOARD_VOWELS[detect_keyboard(s)]
}
pub fn detect_keyboard_consonants(s: &str) -> &str {
   KEYBOARD_CONSONANTS[detect_keyboard(s)]
}

pub fn consonant_mistakes(s: &str) -> Vec<String> {
   let kb = detect_keyboard_consonants(s);
   let mut ts = Vec::new();
   for oi in 0..s.len() {
      //replace consonant, add, or omit
      if !kb.contains(&s[oi..oi+1]) { continue; }
      ts.push(format!("{}{}", &s[..oi], &s[oi+1..]));
      for k in kb.chars() {
         ts.push(format!("{}{}{}", &s[..oi], k, &s[oi+1..]));
         ts.push(format!("{}{}{}", &s[..oi], k, &s[oi..]));
         ts.push(format!("{}{}{}", &s[..oi+1], k, &s[oi+1..]));
      }
   }
   ts
}

pub fn vowel_mistakes(s: &str) -> Vec<String> {
   let kb = detect_keyboard_vowels(s);
   let mut ts = Vec::new();
   for oi in 0..s.len() {
      //replace vowel, add, or omit
      if !kb.contains(&s[oi..oi+1]) { continue; }
      ts.push(format!("{}{}", &s[..oi], &s[oi+1..]));
      for k1 in kb.chars() {
         ts.push(format!("{}{}{}", &s[..oi], k1, &s[oi+1..]));
         ts.push(format!("{}{}{}", &s[..oi], k1, &s[oi..]));
         ts.push(format!("{}{}{}", &s[..oi+1], k1, &s[oi+1..]));
         for k2 in kb.chars() {
            ts.push(format!("{}{}{}{}", &s[..oi], k1, k2, &s[oi..]));
         }
      }
   }
   ts
}

pub fn typos(s: &str) -> Vec<String> {
   let kb = detect_keyboard_layout(s);
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
   for w in vowel_mistakes(s).into_iter() {
      ms.insert(w);
   }
   for w in consonant_mistakes(s).into_iter() {
      ms.insert(w);
   }
   if ms.contains(s) {
      ms.remove(s);
   }
   ms
}
