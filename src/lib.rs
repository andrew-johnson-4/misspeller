use std::collections::HashSet;
use std::iter::FromIterator;

static KEYBOARDS: [&str; 2] = [
   "abcdefghijklmnopqrstuvwxyz",
   "abcdefghijklmnopqrstuvwxyzñõàèìòùäëïöüÿâêîôû",
];
static KEYBOARD_VOWELS: [&str; 2] = [
   "aeiouy",
   "aeiouyõàèìòùäëïöüÿâêîôû",
];
static KEYBOARD_CONSONANTS: [&str; 2] = [
   "bcdfghjklmnpqrstvwxz",
   "bcdfghjklmnpqrstvwxzñ",
];

pub fn detect_keyboard(s: &str) -> usize {
   let mut best = (0, 0);
   for ki in 0..KEYBOARDS.len() {
      let count_contains = s.chars().filter(|c| KEYBOARDS[ki].contains(*c)).count();
      if count_contains==s.len() {
         return ki;
      }
      if best.1 < count_contains {
         best = (ki, count_contains);
      }
   }
   best.0
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

pub fn ateji_mistakes(s: &str) -> Vec<String> {
   let s = s.chars().collect::<Vec<char>>();
   let mut ts = Vec::new();
   for oi in 0..s.len() {
      let c = s[oi];
      if let Some(cs) = cjk::JOUYOU_ATEJI_INDEX.get(&c) {
         for ct in cs.iter() {
            ts.push(format!("{}{}{}", String::from_iter(&s[..oi]), ct, String::from_iter(&s[oi+1..])));
         }
      }
   }
   ts
}

pub fn contraction_mistakes(s: &str) -> Vec<String> {
   let s = s.chars().collect::<Vec<char>>();
   let mut ts = Vec::new();
   if let Some(ai) = s.iter().position(|&c| c=='\'') {
      if ai > 0 {
         ts.push(format!("{}{}", String::from_iter(&s[..ai-1]), String::from_iter(&s[ai..]))); //missing preceding letter
         ts.push(format!("{}{}{}", String::from_iter(&s[..ai]), &s[ai-1], String::from_iter(&s[ai..]))); //duplicate preceding letter
      }
      if ai+2 <= s.len() {
         ts.push(format!("{}{}", String::from_iter(&s[..ai+1]), String::from_iter(&s[ai+2..]))); //missing following letter
         ts.push(format!("{}{}{}", String::from_iter(&s[..ai+1]), &s[ai+1], String::from_iter(&s[ai+1..]))); //duplicate following letter
         ts.push(format!("{}{}{}{}", String::from_iter(&s[..ai]), &s[ai+1], "'", String::from_iter(&s[ai+2..]))); //late apostrophe
      }
      if ai > 0 && ai+1 <= s.len() {
         ts.push(format!("{}{}{}{}", String::from_iter(&s[..ai-1]), "'", &s[ai-1], String::from_iter(&s[ai+1..]))); //early apostrophe
      }
   }
   ts
}

pub fn consonant_mistakes(s: &str) -> Vec<String> {
   let kb = detect_keyboard_consonants(s);
   let s = s.chars().collect::<Vec<char>>();
   let mut ts = Vec::new();
   for oi in 0..s.len() {
      //replace consonant, add, or omit
      if !kb.contains(s[oi]) { continue; }
      ts.push(format!("{}{}", String::from_iter(&s[..oi]), String::from_iter(&s[oi+1..])));
      for k in kb.chars() {
         ts.push(format!("{}{}{}", String::from_iter(&s[..oi]), k, String::from_iter(&s[oi+1..])));
         ts.push(format!("{}{}{}", String::from_iter(&s[..oi]), k, String::from_iter(&s[oi..])));
         ts.push(format!("{}{}{}", String::from_iter(&s[..oi+1]), k, String::from_iter(&s[oi+1..])));
      }
   }
   ts
}

pub fn vowel_mistakes(s: &str) -> Vec<String> {
   let kb = detect_keyboard_vowels(s);
   let s = s.chars().collect::<Vec<char>>();
   let mut ts = Vec::new();
   for oi in 0..s.len() {
      //replace vowel, add, or omit
      if !kb.contains(s[oi]) { continue; }
      ts.push(format!("{}{}", String::from_iter(&s[..oi]), String::from_iter(&s[oi+1..])));
      for k1 in kb.chars() {
         ts.push(format!("{}{}{}", String::from_iter(&s[..oi]), k1, String::from_iter(&s[oi+1..])));
         ts.push(format!("{}{}{}", String::from_iter(&s[..oi]), k1, String::from_iter(&s[oi..])));
         ts.push(format!("{}{}{}", String::from_iter(&s[..oi+1]), k1, String::from_iter(&s[oi+1..])));
         for k2 in kb.chars() {
            ts.push(format!("{}{}{}{}", String::from_iter(&s[..oi]), k1, k2, String::from_iter(&s[oi..])));
         }
      }
   }
   ts
}

pub fn typos(s: &str) -> Vec<String> {
   let kb = detect_keyboard_layout(s);
   let s = s.chars().collect::<Vec<char>>();
   let mut ts = Vec::new();
   for oi in 0..s.len() {
      //omit key
      ts.push(format!("{}{}", String::from_iter(&s[..oi]), String::from_iter(&s[oi+1..])));
   }
   for oi in 0..s.len() {
      //replace key
      for k in kb.chars() {
         ts.push(format!("{}{}{}", String::from_iter(&s[..oi]), k, String::from_iter(&s[oi+1..])));
      }
   }
   for oi in 0..s.len() {
      //add key
      for k in kb.chars() {
         ts.push(format!("{}{}{}", String::from_iter(&s[..oi]), k, String::from_iter(&s[oi..])));
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
   for w in contraction_mistakes(s).into_iter() {
      ms.insert(w);
   }
   for w in ateji_mistakes(s).into_iter() {
      ms.insert(w);
   }
   if ms.contains(s) {
      ms.remove(s);
   }
   ms
}
