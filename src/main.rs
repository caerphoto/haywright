use core::panic;
use std::{
    collections::HashMap,
    env,
    fs,
    io::{self, Write},
    path::Path
};
use rand::{thread_rng, Rng};

const EXTRA_CHARS: &str = "–—“”‘’£œæéèêĊøż×";
const EXTRA_CHARS_BASE_IDX: usize = 129;

struct Input {
    text: String,
    char_to_idx: HashMap<char, usize>,
    idx_to_char: HashMap<usize, char>,
}

impl Input {
    fn from_str(s: &str) -> Self {
        let mut result = Self {
            text: String::from(s),
            char_to_idx: HashMap::new(),
            idx_to_char: HashMap::new(),
        };

        result.build_hashmaps();
        result
    }

    fn from_file<P: AsRef<Path>>(filename: P) -> Self {
        Input::from_str(&fs::read_to_string(filename).unwrap())
    }

    fn from_stdin() -> Self {
        let buf: Vec<String> = io::stdin().lines()
            .map(|l| l.unwrap())
            .collect();
        if buf.is_empty() {
            panic!("Error: no input specified. Either supply a filename, or pipe from stdin.\nBuf: {:?}", buf);
        }

        Input::from_str(&buf.join("\n"))
    }

    fn build_hashmaps(&mut self) {
        for (idx, c) in EXTRA_CHARS.chars().enumerate() {
            self.char_to_idx.insert(c, idx + EXTRA_CHARS_BASE_IDX);
            self.idx_to_char.insert(idx + EXTRA_CHARS_BASE_IDX, c);
        }
    }

    fn generate_output(&self, count: usize, seq_len: usize) -> String {
        let mut rng = thread_rng();
        let input = &self.text;
        let mut freq_table: Vec<usize> = vec![0; 200]; // ASCII plus some room for some common
                                                       // non-ASCII characters
        let mut output = String::with_capacity(count);
        let mut current_len: usize = 0;

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
                    let &freq_idx = self.char_to_idx
                        .get(&next_char)
                        .unwrap_or_else(|| panic!("Non-ASCII char '{next_char}' not found in index map"));
                    freq_table[freq_idx] += 1;
                }
            }

            let mut n: usize = std::cmp::max(1, rng.gen_range(0..freq_table.iter().sum()));
            for (idx, freq) in freq_table.iter().enumerate() {
                n = n.saturating_sub(*freq);
                if n == 0 {
                    let new_char = if idx > 128 {
                        *self.idx_to_char.get(&idx).unwrap_or(&'?')
                    } else {
                        unsafe { char::from_u32_unchecked(idx as u32) }
                    };
                    output.push(new_char);
                    ptn_seq.push(new_char);
                    ptn_seq.remove(0);
                    print!("{new_char}");
                    let _ = std::io::stdout().flush();
                    break;
                }
            }

            current_len += 1;
        }

        output
    }
}

fn read_shell_input() -> Input {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        Input::from_stdin()
    } else {
        Input::from_file(&args[1])
    }
}

fn main() {
    env_logger::init();

    read_shell_input().generate_output(5000, 6);
}
