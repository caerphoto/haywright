use std::{
    cmp::max,
    collections::HashMap,
    io::{self, Write},
};
use rand::{thread_rng, Rng};

const COMMON_EXTRA_CHARS: &str = "–—“”‘’£œæéèêĊøż×";
const EXTRA_CHARS_BASE_IDX: usize = 129;

struct CharIdxMap {
    char_to_idx: HashMap<char, usize>,
    idx_to_char: HashMap<usize, char>,
    extra_chars_next_idx: usize,
}

// This handles mapping frequency table indices for non-ASCII characters
impl CharIdxMap {
    fn new() -> Self {
        let mut result = Self {
            char_to_idx: HashMap::new(),
            idx_to_char: HashMap::new(),
            extra_chars_next_idx: EXTRA_CHARS_BASE_IDX,
        };

        for (idx, c) in COMMON_EXTRA_CHARS.chars().enumerate() {
            result.char_to_idx.insert(c, idx + EXTRA_CHARS_BASE_IDX);
            result.idx_to_char.insert(idx + EXTRA_CHARS_BASE_IDX, c);
        }

        result
    }

    fn index_of(&mut self, c: char) -> Option<usize> {
        if c.is_ascii() {
            let code = c as usize;
            if code == 9 || code == 10 || code == 13 || code >= 32 {
                return Some(code)
            } else {
                return None
            }
        }

        match self.char_to_idx.get(&c) {
            Some(&idx) => Some(idx),
            None => {
                self. extra_chars_next_idx += 1;
                self.char_to_idx.insert(c, self.extra_chars_next_idx);
                self.idx_to_char.insert(self.extra_chars_next_idx, c);
                Some(self.extra_chars_next_idx)
            }
        }
    }

    fn char_at(&self, idx: usize) -> char {
        if idx > 128 {
            *self.idx_to_char.get(&idx).unwrap_or(&'?')
        } else {
            unsafe { char::from_u32_unchecked(idx as u32) }
        }
    }
}

pub struct Hay {
    text: String,
}

impl Hay {
    /// Create a Hay struct ready for generation of random output.
    pub fn new(input_text: &str) -> Self {
        Self {
            text: String::from(input_text),
        }
    }

    /// Generate random output based on the stored input.
    pub fn generate_output(&self, count: usize, seq_len: usize, live: bool) -> String {
        let input = &self.text;
        let mut output = String::with_capacity(count);

        let mut current_len: usize = 0;
        let mut char_idx_map = CharIdxMap::new();
        let mut freq_table: Vec<usize> = vec![0; 256]; // ASCII plus some room for some common

        let mut rng = thread_rng();
        let initial_seq_start: usize = rng.gen_range(0..input.chars().count() - seq_len);
        output.push_str(&Hay::get_range(input, initial_seq_start, seq_len));


        while current_len < count {
            freq_table.fill(0);
            let seq = Hay::get_seq(&output, seq_len);

            for (idx, _) in input.match_indices(&seq) {
                let slice = &input[idx..];
                let Some(next_char) = slice.chars().nth(seq_len) else { continue };
                let Some(table_idx) = char_idx_map.index_of(next_char) else { continue };
                freq_table[table_idx] += 1;
            }

            let mut n: usize = max(1, rng.gen_range(0..freq_table.iter().sum()));
            let mut idx = 0;
            for &freq in freq_table.iter() {
                n = n.saturating_sub(freq);
                if n > 0 {
                    idx += 1;
                }
            }
            let new_char = char_idx_map.char_at(idx);
            output.push(new_char);
            if live {
                print!("{new_char}");
                let _ = io::stdout().flush();
            }
            current_len += 1;
        }

        output
    }

    /// Returns a string of `count` chars starting at the given `start` index, collected from the
    /// given input.
    fn get_range(s: &str, start: usize, count: usize) -> String {
        s
            .chars()
            .skip(start)
            .take(count)
            .collect()
    }

    /// Returns a string collected from the last `len` chars of `output`.
    fn get_seq(output: &str, len: usize) -> String {
        let output_len = output.chars().count();
        Hay::get_range(output, output_len - len, len)
    }
}

