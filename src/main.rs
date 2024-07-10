#![feature(hash_raw_entry)]

use std::collections::HashMap;

use bstr::ByteVec;
use bstr::ByteSlice;
use identity_hash::BuildIdentityHasher;
use xxhash_rust::xxh3::xxh3_64;

use alphabet::*;

mod alphabet;

const PRINT_FLAG: usize = 0x03FFFF80;

fn main() {
    let mut birf = Birf::default();
    birf.run();
    std::mem::forget(birf);
}

struct Birf {
    map: HashMap<u64, usize, BuildIdentityHasher<u64>>,
    /// always the index of the NEXT string to hash
    count: usize,
}

impl Default for Birf {
    fn default() -> Self {
        Self {
            map: HashMap::with_hasher(BuildIdentityHasher::<u64>::default()),
            count: 1,
        }
    }
}

impl Birf {
    fn run(&mut self) {
        let mut buffer: Vec<u8> = Vec::new();

        // insert the empty string
        self.map.insert(xxh3_64(&[]), 0);

        // start with the first char
        buffer.push(FIRST_CHAR);
        let mut collision_found = false;
        loop {
            for (index, char) in CHARS.into_iter().enumerate() {
                collision_found |= self.hash(&buffer, self.count + index);
                unsafe { *buffer.last_mut().unwrap_unchecked() = char; }
            }
            self.count += CHARS.len();
            if collision_found || self.hash(&buffer, self.count) {
                return;
            }
            self.count += 1;

            // we just finished a run of the LSB, so we need to carry now
            // the current last char needs to get bumped by one
            let mut full_carry = true; // if every single digit was maxed
            let len = buffer.len();
            for digit in buffer[..len - 1].iter_mut().rev() {
                if let Some(incremented) = increment_char(*digit) {
                    // we found a digit that was NOT maxed
                    full_carry = false;
                    *digit = incremented;
                    break;
                } else {
                    // this digit was maxed, so we reset it to the '0' value and move on
                    *digit = FIRST_CHAR;
                }
            }
            if full_carry {
                buffer[0] = CHARS[0];
                for digit in buffer[1..len].iter_mut() {
                    *digit = FIRST_CHAR;
                }
                buffer.push(FIRST_CHAR);
            } else {
                unsafe { *buffer.last_mut().unwrap_unchecked() = FIRST_CHAR; }
            }

            if self.count & PRINT_FLAG == 0 {
                println!("{}", self.count);
            }
        }
    }

    #[inline(always)]
    fn hash(&mut self, buffer: &[u8], count: usize) -> bool {
        let hash = xxh3_64(buffer);

        // debug print what we're presently doing
        //println!("{:016X} {} \"{}\"", hash, count, buffer.to_str().unwrap());

        let collision = self.map.insert(hash, count);
        if let Some(collision) = collision {
            println!("{:016X} \"{}\" \"{}\"", hash, lookup(collision), buffer.to_str().unwrap());
            true
        } else {
            false
        }
    }
}

fn lookup(mut count: usize) -> String {
    if count == 0 {
        "".to_string()
    } else if count == 1 {
        (FIRST_CHAR as char).to_string()
    } else {
        count -= 1;
        let len = CHARS.len() + 1;
        let mut buffer = Vec::new();
        while count != 0 {
            let div = count / len;
            let rem = count % len;
            buffer.push(nth_char(rem));
            count = div;
        }
        buffer.reverse();
        buffer.into_string().unwrap()
    }
}

fn nth_char(n: usize) -> u8 {
    if n == 0 {
        FIRST_CHAR
    } else {
        CHARS[n - 1]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_lookup() {
        assert_eq!(lookup(0), "");
        assert_eq!(lookup(1), "\t");
        assert_eq!(lookup(2), " ");
        assert_eq!(lookup(96), "~");
        assert_eq!(lookup(97), " \t");
        assert_eq!(lookup(1920), "2~");
        assert_eq!(lookup(186981), "3:c");
        assert_eq!(lookup(192000), "3n~");
    }
}
