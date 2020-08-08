# misspeller

[![Crates.IO](https://img.shields.io/crates/v/misspeller.svg)](https://crates.rs/crates/misspeller)
[![Documentation](https://img.shields.io/badge/api-rustdoc-blue.svg)](https://docs.rs/misspeller/)
[![Build Nightly](https://github.com/andrew-johnson-4/misspeller/workflows/BuildNightly/badge.svg)](https://github.com/andrew-johnson-4/misspeller)
[![Build](https://github.com/andrew-johnson-4/misspeller/workflows/Build/badge.svg)](https://github.com/andrew-johnson-4/misspeller)

Take correctly spelled words and return common spelling mistakes

Types of Common Mistakes (not all applicable to all languages)
- ☑ Typo
- ☑ Vowel Mistake
- ☑ Consonant Mistake
- ☑ Contraction Mistake
- ☐ Phonetical Misspelling / Ateji
- ☐ Slurred Word
- ☑ Missed or Misplaced Modifier (accent or other modifiers)

```rust
./misspeller [word]
```
