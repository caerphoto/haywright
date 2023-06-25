use std::{
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

    fn index_of(&mut self, c: char) -> usize {
        match self.char_to_idx.get(&c) {
            Some(&idx) => idx,
            None => {
                self. extra_chars_next_idx += 1;
                self.char_to_idx.insert(c, self.extra_chars_next_idx);
                self.idx_to_char.insert(self.extra_chars_next_idx, c);
                self.extra_chars_next_idx
            }
        }

    }

    fn char_at(&self, idx: usize) -> char {
        *self.idx_to_char.get(&idx).unwrap_or(&'?')
    }
}

pub struct Hay {
    text: String,
}

impl Hay {
    pub fn new(input_text: &str) -> Self {
        Self {
            text: String::from(input_text),
        }
    }

    pub fn generate_output(&self, count: usize, seq_len: usize, live: bool) -> String {
        let mut rng = thread_rng();
        let input = &self.text;
        let mut freq_table: Vec<usize> = vec![0; 256]; // ASCII plus some room for some common
                                                       // non-ASCII characters
        let mut output = String::with_capacity(count);
        let mut current_len: usize = 0;
        let mut char_idx_map = CharIdxMap::new();

        let initial_seq_start: usize = rng.gen_range(0..input.len() - seq_len);
        let mut ptn_seq = String::with_capacity(count + seq_len);
        ptn_seq.push_str(&input[initial_seq_start..initial_seq_start + seq_len]);

        while current_len < count {
            freq_table.fill(0);
            for (idx, _) in input.match_indices(&ptn_seq) {
                let slice = &input[idx..];
                let Some(next_char) = slice.chars().nth(seq_len) else { break };
                if next_char.is_ascii() {
                    let code = next_char as usize;
                    if code == 9 || code == 10 || code == 13 || code >= 32 {
                        freq_table[code] += 1;
                    }
                } else {
                    let freq_idx = char_idx_map.index_of(next_char);
                    freq_table[freq_idx] += 1;
                }
            }

            let mut n: usize = std::cmp::max(1, rng.gen_range(0..freq_table.iter().sum()));
            for (idx, freq) in freq_table.iter().enumerate() {
                n = n.saturating_sub(*freq);
                if n == 0 {
                    let new_char = if idx > 128 {
                        char_idx_map.char_at(idx)
                    } else {
                        unsafe { char::from_u32_unchecked(idx as u32) }
                    };
                    output.push(new_char);
                    ptn_seq.push(new_char);
                    ptn_seq.remove(0);
                    if live {
                        print!("{new_char}");
                        let _ = io::stdout().flush();
                    }
                    break;
                }
            }

            current_len += 1;
        }

        output
    }
}
