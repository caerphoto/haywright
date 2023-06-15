use core::panic;
use std::{env, fs, io::{self, Write}};
use rand::{thread_rng, Rng};
use log::{log_enabled, Level};

fn abort(msg: &str) {
    println!("{msg}");
    std::process::exit(1);
}

fn read_input() -> String {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        let buf: Vec<String> = io::stdin().lines()
            .map(|l| l.unwrap())
            .collect();
        if buf.is_empty() {
            abort("Error: no input specified. Either supply a filename, or pipe from stdin.");
        }

        buf.join("\n")
    } else {
        fs::read_to_string(&args[1]).unwrap()
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
        _ => '?'
    }

}

fn main() {
    env_logger::init();

    const SEQ_LEN: usize = 5;

    let mut rng = thread_rng();
    let source_text = read_input();
    let mut freq_table: Vec<usize> = vec![0; 200]; // ASCII plus some room for some common
                                                   // non-ASCII characters

    let target_len: usize = 500;
    let mut current_len: usize = 0;
    let mut output_text = String::with_capacity(target_len);

    let initial_seq_start: usize = rng.gen_range(0..source_text.len() - SEQ_LEN);
    let mut ptn_seq = String::with_capacity(target_len + SEQ_LEN);
    ptn_seq.push_str(&source_text[initial_seq_start..initial_seq_start + SEQ_LEN]);

    log::debug!("Initial ptn_seq: '{ptn_seq}'");

    while current_len < target_len {
        let sub_ptn_seq: String = ptn_seq.chars().skip(current_len).take(SEQ_LEN).collect();
        log::debug!("Scanning for '{sub_ptn_seq}'");


        for (idx, m) in source_text.match_indices(&sub_ptn_seq) {
            log::debug!("{idx}]: {m}");
            let slice = &source_text[idx..];
            let next_char = slice.chars().nth(SEQ_LEN).unwrap();
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

        log::debug!("Done scanning for '{sub_ptn_seq}'. Freq table:");
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
                output_text.push(new_char);
                ptn_seq.push(new_char);
                print!("{new_char}");
                let _ = std::io::stdout().flush();
                break;
            }
        }

        log::debug!("New full ptn_seq: '{ptn_seq}'");
        log::info!("Output length: {} chars", output_text.chars().count());
        current_len += 1;
        freq_table.fill(0);
    }

    println!("{output_text}");
}
