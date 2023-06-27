use std::{
    cmp::max,
    collections::HashMap,
    io::{self, Write},
};
use rand::{thread_rng, Rng};

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
        for c in input.chars() {
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
        let char_idx_map = CharIdxMap::new(input);
        let mut freq_table: Vec<usize> = vec![0; 256]; // ASCII plus some room for some common

        let mut rng = thread_rng();
        let initial_seq_start: usize = rng.gen_range(0..input.chars().count() - seq_len);
        let initial_str: String = Hay::get_range(input, initial_seq_start, seq_len);
        output.push_str(&initial_str);


        while current_len < count {
            freq_table.fill(0);
            let seq = Hay::get_seq(&output, seq_len);

            // Count frequencies of characters that follow matches.
            for (idx, _) in input.match_indices(&seq) {
                let Some(next_char) = input[idx..].chars().nth(seq_len) else { continue };
                let table_idx = char_idx_map.index_for(next_char);
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

