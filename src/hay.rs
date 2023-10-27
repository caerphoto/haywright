use std::{
    cmp::max,
    collections::HashMap,
    fmt::Formatter,
};
use rand::{thread_rng, rngs::ThreadRng, Rng};
use unicode_normalization::UnicodeNormalization;
use regex::Regex;
use once_cell::sync::Lazy;

// ------------ Word-based stuff --------------------
static NON_WORD_CHARS: Lazy<Regex> = Lazy::new(|| Regex::new(r"\W").unwrap());

#[derive(Debug, PartialEq, Eq)]
struct Word {
    source: String,
    depunct: String,
    bare: String,
}

impl Word {
    fn new(s: &str) -> Self {
        let depunct = NON_WORD_CHARS.replace(s, "").to_string();
        let bare = depunct.to_lowercase();
        Self {
            source: String::from(s),
            depunct,
            bare,
        }
    }
}

struct WordTokens<'a>(Vec<&'a Word>);

impl PartialEq for WordTokens<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.0.iter().enumerate()
            .all(|(idx, word)| word.bare == other.0[idx].bare)
    }
}

impl Eq for WordTokens<'_> {}

impl std::fmt::Debug for WordTokens<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let words: String = self.0.iter()
            .map(|w| w.source.as_str())
            .collect::<Vec<&str>>()
            .join(" ");
        write!(f, "{words}")
    }
}

impl std::convert::From<&WordTokens<'_>> for String {
    fn from(value: &WordTokens) -> Self {
        let capacity = value.0.iter()
            .map(|word| word.depunct.len())
            .sum();
        let mut result = String::with_capacity(capacity);
        for word in &value.0 {
            result.push_str(&word.source);
            result.push(' ');
        }
        result
    }
}

struct Words {
    data: Vec<Word>,
}

impl Words {
    fn new(input: &str) -> Self {
        Self {
            data: input.split_whitespace()
                .map(Word::new)
                .collect(),
        }
    }
    fn len(&self) -> usize {
        self.data.len()
    }

}

// ------------ Character-based stuff --------------------
const EXTRA_CHARS_BASE_IDX: usize = 129;

struct CharIdxMap {
    char_to_idx: HashMap<char, usize>,
    idx_to_char: HashMap<usize, char>,
}

// This handles mapping frequency table indices for ASCII and non-ASCII characters
impl CharIdxMap {
    fn new(input: &str) -> Self {
        let mut result = Self {
            char_to_idx: HashMap::new(),
            idx_to_char: HashMap::new(),
        };

        let mut non_ascii_idx = EXTRA_CHARS_BASE_IDX;
        for c in input.nfc() {
            if c.is_ascii() { continue; }
            if result.char_to_idx.get(&c).is_none() {
                result.char_to_idx.insert(c, non_ascii_idx);
                result.idx_to_char.insert(non_ascii_idx, c);
                non_ascii_idx += 1;
            }
        }

        result
    }

    fn index_for(&self, c: char) -> usize {
        if c.is_ascii() {
            c as usize
        } else {
            *self.char_to_idx.get(&c).unwrap()
        }
    }

    fn char_at(&self, idx: usize) -> char {
        if idx > 128 {
            *self.idx_to_char.get(&idx).unwrap_or(&char::REPLACEMENT_CHARACTER)
        } else {
            unsafe { char::from_u32_unchecked(idx as u32) }
        }
    }
}

pub struct Hay {
    words: Words,
    characters: String,
    seq_len: usize,
    is_word_tokens: bool,
    char_idx_map: CharIdxMap,
}

impl Hay {
    pub fn new(input: &str, seq_len: u8, is_word_tokens: bool) -> Self {
        let words = Words::new(input);
        let seq_len = seq_len as usize;
        let flattened = input.split_whitespace().collect::<Vec<&str>>().join(" ");
        let char_idx_map = CharIdxMap::new(&flattened);
        Self {
            words,
            characters: flattened,
            seq_len,
            is_word_tokens,
            char_idx_map
        }
    }

    pub fn generate_output(&self, count: usize) -> String {
        if self.is_word_tokens {
            self.generate_word_output(count)
        } else {
            self.generate_char_output(count)
        }
    }

    fn generate_word_output(&self, count: usize) -> String {
        let mut output = String::with_capacity(count);
        let mut rng = thread_rng();

        let mut seq_start_idx = rng.gen_range(0..self.last_safe_index());
        let mut last_match = self.get_words(seq_start_idx);

        let mut matches = 0;
        let mut misses = 0;

        loop {
            output.push_str(&String::from(&last_match));
            if output.len() > count {
                break;
            }
            if let Some(next_match_idx) = self.get_next_index(&last_match, &mut rng) {
                seq_start_idx = next_match_idx + self.seq_len;
                matches += 1;
            } else {
                seq_start_idx = rng.gen_range(0..self.last_safe_index());
                misses += 1;
            }
            last_match = self.get_words(seq_start_idx);
        }

        dbg!(matches, misses);

        output
    }

    fn content_length(&self) -> usize {
        if self.is_word_tokens { self.words.len() } else { self.characters.len() }
    }

    fn last_safe_index(&self) -> usize {
        self.content_length() - (self.seq_len * 2)
    }

    fn get_words(&self, pos: usize) -> WordTokens {
        let words: Vec<&Word> = self.words.data[pos..pos + self.seq_len].iter()
            .collect();
        WordTokens(words)
    }

    fn get_next_index(&self, last_match: &WordTokens, rng: &mut ThreadRng) -> Option<usize> {
        let mut indices: Vec<usize> = Vec::new();

        for idx in 0..self.last_safe_index() {
            let words = self.get_words(idx);
            if words == *last_match {
                indices.push(idx);
            }
        }
        if indices.len() == 1 {
            // no other instances of this word sequence
            return None;
        }
        let idx = rng.gen_range(0..indices.len());
        Some(indices[idx])
    }

    // ============

    pub fn generate_char_output(&self, count: usize) -> String {
        let input = &self.characters;
        let seq_len = self.seq_len;
        let mut output = String::with_capacity(count);

        let mut current_len: usize = 0;
        let mut freq_table: Vec<usize> = vec![0; 256]; // ASCII plus some room for some common chars

        let mut rng = thread_rng();
        let initial_seq_start: usize = rng.gen_range(0..input.chars().count() - seq_len);
        let initial_str: String = Hay::get_range(input, initial_seq_start, seq_len);
        output.push_str(&initial_str);

        while current_len < count {
            freq_table.fill(0);
            let seq = Hay::get_seq(&output, seq_len);

            // Count frequencies of characters that follow matches.
            for idx in memchr::memmem::find_iter(input.as_bytes(), &seq) {
                let Some(next_char) = input[idx..].chars().nth(seq_len) else { continue };
                let table_idx = self.char_idx_map.index_for(next_char);
                freq_table[table_idx] += 1;
            }

            let sum: usize = freq_table.iter().sum();

            // This can happen when there is only a single instance of a character in the whole
            // input.
            if sum == 0 {
                if output.chars().count() > seq_len {
                    output.pop();
                    continue;
                } else {
                    break;
                }
            }

            // Pick a random next character, biased by frequency.
            let mut n: usize = max(1, rng.gen_range(0..sum));
            let mut idx = 0;
            for &freq in freq_table.iter() {
                n = n.saturating_sub(freq);
                if n == 0 {
                    break
                }
                idx += 1;
            }
            let new_char = self.char_idx_map.char_at(idx);
            output.push(new_char);
            current_len += 1;
        }

        output
    }

    /// Returns a string of `count` chars starting at the given `start` index, collected from the
    /// given input. Note that this operates on *characters*, not *bytes*.
    fn get_range(s: &str, start: usize, count: usize) -> String {
        s
            .chars()
            .skip(start)
            .take(count)
            .collect()
    }

    /// Returns a 'string' collected from the last `len` chars of `output`.
    fn get_seq(output: &str, len: usize) -> Vec<u8> {
        let output_len = output.chars().count();
        Hay::get_range(output, output_len - len, len).as_bytes().to_vec()
    }
}
