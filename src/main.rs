#![feature(hash_raw_entry)]

use std::collections::HashMap;

use bstr::ByteSlice;
use identity_hash::BuildIdentityHasher;
use xxhash_rust::xxh3::xxh3_64;

use alphabet::*;

mod alphabet;

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
        }
    }

    #[inline(always)]
    fn hash(&mut self, buffer: &[u8], count: usize) -> bool {
        let hash = xxh3_64(buffer);
        let collision = self.map.insert(hash, count);
        if let Some(collision) = collision {
            
            println!("{:016X} \"{}\" \"{}\"", hash, lookup(collision), buffer.to_str().unwrap());
            true
        } else {
            false
        }
    }
}

fn lookup(count: usize) -> String {
    //TODO: map collision index back to collision text
    format!("{count}")
}
