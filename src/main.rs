use rand::{thread_rng, Rng};

// This alphabet uses `A-Za-z0-9_-` symbols.
// The order of characters is optimized for better gzip and brotli compression.
// References to the same file (works both for gzip and brotli):
// `'use`, `andom`, and `rict'`
// References to the brotli default dictionary:
// `-26T`, `1983`, `40px`, `75px`, `bush`, `jack`, `mind`, `very`, and `wolf`
const URL_ALPHABET: &str = "useandom-26T198340PX75pxJACKVERYMINDBUSHWOLF_GQZbfghjklqvwyzrict";
const ALPHABET_SIZE: usize = URL_ALPHABET.len();

fn nanoid(size: usize) -> String {
    let mut id = String::with_capacity(size);
    let mut rng = thread_rng();
    for _ in 0..size {
        let index = rng.gen_range(0..ALPHABET_SIZE);
        id.push(URL_ALPHABET.chars().nth(index).unwrap());
    }

    id
}

fn main() {
    println!("{}", nanoid(21));
}
