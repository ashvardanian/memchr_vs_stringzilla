use rand::{rngs::ThreadRng, Rng};

pub fn open() -> Vec<u8> {
    let path = std::env::var("FILE").unwrap_or_default();
    let file = std::fs::read(path).unwrap();
    file
}

pub fn random_token<'a, 'b>(rng: &'b mut ThreadRng, file: &'a Vec<u8>) -> &'a [u8] {
    let tokens: Vec<&[u8]> = file.split(|c| *c == b' ').collect();
    tokens[rng.gen_range(0..tokens.len())]
}
