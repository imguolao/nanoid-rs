use rand::{thread_rng, Rng};

// This alphabet uses `A-Za-z0-9_-` symbols.
// The order of characters is optimized for better gzip and brotli compression.
// References to the same file (works both for gzip and brotli):
// `'use`, `andom`, and `rict'`
// References to the brotli default dictionary:
// `-26T`, `1983`, `40px`, `75px`, `bush`, `jack`, `mind`, `very`, and `wolf`
const URL_ALPHABET: &str = "useandom-26T198340PX75pxJACKVERYMINDBUSHWOLF_GQZbfghjklqvwyzrict";

fn format(size: usize, alphabet: &str) -> String {
    let len = alphabet.len();
    if len == 0 {
        panic!("Can not use a empty alphabet to generate a nanoid.");
    }

    let mut rng = thread_rng();
    let id: String = (0..size)
        .map(|_| rng.gen_range(0..len))
        .map(|i| alphabet.chars().nth(i).unwrap())
        .collect();

    id
}

macro_rules! nanoid {
    // default
    () => {
        format(21, URL_ALPHABET)
    };

    ($size:expr) => {
        format($size, URL_ALPHABET)
    };

    ($size:expr, $alphabet:expr) => {
        format($size, $alphabet)
    };
}

fn main() {
    println!("{}", nanoid!());
    println!("{}", nanoid!(21));
    println!("{}", nanoid!(21, "0123456789_-"));
    // println!("{}", nanoid!(21, ""));
}
