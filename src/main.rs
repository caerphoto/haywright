use core::panic;
use std::{env, fs, io::{self, Write}, path::Path};
use rand::{thread_rng, Rng};
use log::{log_enabled, Level};


struct Input {
    chars: String,
    words: Vec<String>,
}

impl Input {
    fn from_str(s: &str) -> Self {
        let chars = String::from(s);
        let words = Input::parse_words(&chars);
        Self { chars, words }
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

    fn parse_words(s: &str) -> Vec<String> {
        s.split_whitespace()
            .map(|w| w.into())
            .collect()
    }

    fn output_chars(&self, count: usize) -> String {
        const SEQ_LEN: usize = 5;
        let mut rng = thread_rng();
        let input = &self.chars;
        let mut freq_table: Vec<usize> = vec![0; 200]; // ASCII plus some room for some common
                                                    // non-ASCII characters
        let mut current_len: usize = 0;
        let mut output = String::with_capacity(count);

        let initial_seq_start: usize = rng.gen_range(0..input.len() - SEQ_LEN);
        let mut ptn_seq = String::with_capacity(count + SEQ_LEN);
        ptn_seq.push_str(&input[initial_seq_start..initial_seq_start + SEQ_LEN]);

        log::debug!("Initial ptn_seq: '{ptn_seq}'");

        while current_len < count {
            log::debug!("Scanning for '{ptn_seq}'");
            for (idx, m) in input.match_indices(&ptn_seq) {
                log::debug!("{idx}]: {m}");
                let slice = &input[idx..];
                let Some(next_char) = slice.chars().nth(SEQ_LEN) else { break };
                if next_char.is_ascii() {
                    let code = next_char as usize;
                    log::debug!("  Found match with '{next_char}' ({code}) after");
                    if code == 9 || code == 10 || code == 13 || code >= 32 {
                        freq_table[code] += 1;
                    }
                } else {
                    log::debug!("  Found non-ASCII match with '{next_char}'");
                    freq_table[idx_of_non_ascii(next_char)] += 1;
                }
            }

            log::debug!("Done scanning for '{ptn_seq}'. Freq table:");
            if log_enabled!(Level::Debug) {
                let freqs: Vec<String> = freq_table.iter()
                    .enumerate()
                    .map(|(i, f)| [i, *f])
                    .filter(|fa| fa[1] > 0)
                    .map(|fa| format!("'{}': {}", unsafe { char::from_u32_unchecked(fa[0] as u32) }, fa[1]))
                    .collect();
                log::debug!("{:?}, sum: {}", freqs, freq_table.iter().sum::<usize>());
            }

            let mut n: usize = std::cmp::max(1, rng.gen_range(0..freq_table.iter().sum()));
            for (idx, freq) in freq_table.iter().enumerate() {
                n = n.saturating_sub(*freq);
                if n == 0 {
                    let new_char = if idx > 128 {
                        non_ascii_char_from_idx(idx)
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

            log::debug!("New full ptn_seq: '{ptn_seq}'");
            log::info!("Output length: {} chars", output.chars().count());
            current_len += 1;
            freq_table.fill(0);
        }

        output
    }

    fn bare_word(s: &str) -> String {
        s.chars().filter(|c| c.is_alphanumeric()).collect()
    }

    fn output_words(&self, count: usize) -> String {
        const SEQ_LEN: usize = 2;
        let mut rng = thread_rng();
        let input = &self.words;
        let mut output: Vec<&str> = Vec::with_capacity(count);
        let mut current_len: usize = 0;
        let mut group: Vec<String> = Vec::with_capacity(SEQ_LEN);

        log::debug!("Ready to loop");

        while current_len < count {
            let mut seq_start: usize = rng.gen_range(0..input.len() - SEQ_LEN);
            log::debug!("Starting at idx {seq_start}");
            group.clear();
            for word in input[seq_start..seq_start + SEQ_LEN].iter() {
                group.push(Input::bare_word(word));
                output.push(word);
                current_len += 1;
                print!("{word} ");
            }
            let _ = std::io::stdout().flush();

            seq_start += 1;
            log::debug!("Searching for {group:?}...");
            while seq_start < input.len() - SEQ_LEN && current_len < count {
                let cmp_seq: Vec<String> = input[seq_start..seq_start + SEQ_LEN]
                    .iter().map(|w| Input::bare_word(w)).collect();
                // log::debug!("Comparing {:?} and {:?}", group, cmp_seq);
                if group.iter().eq(cmp_seq.iter()) {
                    log::debug!("Matched {:?} and {:?}", group, cmp_seq);
                    group.clear();
                    for word in input[seq_start + SEQ_LEN..seq_start + SEQ_LEN * 2].iter() {
                        group.push(Input::bare_word(word));
                        output.push(word);
                        current_len += 1;
                        print!("{word} ");
                    }
                    log::debug!("Next group {:?}", group);
                    let _ = std::io::stdout().flush();
                    seq_start += SEQ_LEN + 1;
                } else {
                    seq_start += 1;
                }

            }
            log::debug!("Reached the end with no more matches. Current output:\n{}", output.join(" "));
        }

        output.join(" ")
    }
}

fn read_input() -> Input {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        Input::from_stdin()
    } else {
        Input::from_file(&args[1])
    }
}

fn idx_of_non_ascii(ch: char) -> usize {
    match ch {
        '–' => 129, // en dash
        '—' => 130, // em dash
        '“' => 131, // ldquo
        '”' => 132, // rdquo
        '‘' => 133, // lsquo
        '’' => 134, // rsquo
        '£' => 135,
        'œ' => 136,
        'æ' => 137,
        'é' => 138,
        'è' => 139,
        'ê' => 140,
        _ => panic!("No match for non-ACII char {ch}"),
    }
}

fn non_ascii_char_from_idx(idx: usize) -> char {
    match idx {
        129 => '–', // en dash
        130 => '—', // em dash
        131 => '“', // ldquo
        132 => '”', // rdquo
        133 => '‘', // lsquo
        134 => '’', // rsquo
        135 => '£',
        136 => 'œ',
        137 => 'æ',
        138 => 'é',
        139 => 'è',
        140 => 'ê',
        _ => '?'
    }

}

fn main() {
    env_logger::init();

    let input = read_input();
    // input.output_chars(5000);
    input.output_words(500);
}
