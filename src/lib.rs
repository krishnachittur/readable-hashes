#[macro_use]
extern crate lazy_static;
extern crate byteorder;
extern crate sha2;

use std::error::Error;
use std::io::Cursor;
use byteorder::{BigEndian, ReadBytesExt};
use sha2::{Sha256, Digest};

const WORDFILE: &'static str = include_str!("../data/common_words.lst");
lazy_static! {
    static ref WORDLIST: Vec<&'static str> = {
        WORDFILE.split_whitespace().collect()
    };
}

pub fn hash_en(bytestring: &[u8]) -> Result<Vec<&str>, Box<dyn Error>> {
    let mut hasher = Sha256::new();
    hasher.input(bytestring);
    let result = hasher.result();
    let mut cur = Cursor::new(result.as_slice());
    let mut indices = [0_u16; 16];
    cur.read_u16_into::<BigEndian>(&mut indices)?;
    let words = indices
        .iter()
        .map(|&idx| WORDLIST[idx as usize])
        .collect();
    Ok(words)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn wordlist_len() {
        assert_eq!(WORDLIST.len(), 1 << 16);
    }
    fn pubkey_example() {
        unimplemented!();
    }
}
