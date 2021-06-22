extern crate all_pats;

use std::str::FromStr;
use std::fmt::{self, Display};
use std::io::prelude::*;
use all_pats::Triple;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Letter { A, B }
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct VolDis {
    vol: i64,
    disambig: i64,
    letter: Letter,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PanicError { }
impl<E: std::fmt::Display> From<E> for PanicError {
    fn from(e: E) -> PanicError {
        panic!("{}", e);
    }
}

impl Display for Letter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Letter::A => write!(f, "a"),
            Letter::B => write!(f, "b"),
        }
    }
}

impl Display for VolDis {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}-{}-{}", self.vol, self.disambig, self.letter)
    }
}

impl FromStr for VolDis {
    type Err = PanicError;

    fn from_str(s: &str) -> Result<Self, PanicError> {
        let mut parts = s.split("-");
        let vol = parts.next().unwrap_or_else(|| panic!("expected 3 parts, got {:?}", s)).parse::<u32>()? as i64;
        let disambig = parts.next().unwrap_or_else(|| panic!("expected 3 parts, got {:?}", s)).parse::<u32>()? as i64;
        let letter = match parts.next() {
            None => panic!("expected 3 parts, got {:?}", s),
            Some("a") => Letter::A,
            Some("b") => Letter::B,
            Some(_) => panic!("expected a or b"),
        };
        Ok(VolDis { vol, disambig, letter })
    }
}

impl VolDis {
    fn get_triple(self) -> Triple {
        //for &g in &[1, 2] {
        let g = 2 / all_pats::gcd(2, self.disambig);
        let c = g * self.vol;
        let a = c - self.disambig;
        if let Some(triple) = Triple::checked_from_ac(a, c) {
            return triple;
        }
        //}
        panic!("Invalid solution: {}", self)
    }
}

fn main() -> Result<(), PanicError> {
    let stdin = std::io::stdin();
    let mut stdin = stdin.lock();
    let mut s = String::new();
    stdin.read_to_string(&mut s)?;

    for word in s.split_whitespace() {
        let vol_dis = word.parse::<VolDis>()?;
        let triple = vol_dis.get_triple();
        println!("{} {:?}", vol_dis, triple);
    }
    Ok(())
}
