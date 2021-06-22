#![feature(iterator_step_by)]
//! hex-sols, but constrained to solutions of equal bond length, which
//! simplifies the problem IMMENSELY

extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate all_pats;
use all_pats::{gcd, Triple, isqrt};
use std::collections::HashMap;

#[derive(Serialize)]
#[serde(rename_all = "kebab-case")]
struct SolutionInfo {
    a_sol: Triple,
    b_sol: Triple,
}

type Perm = Vec<usize>;
fn argsort<T: PartialOrd>(xs: &[T]) -> Perm {
    let mut vec = (0..xs.len()).collect::<Vec<_>>();
    vec.sort_by(|&a, &b| xs[a].partial_cmp(&xs[b]).unwrap());
    vec
}
fn permute<T: Clone>(xs: &[T], perm: &Perm) -> Vec<T> {
    perm.iter().map(|&i| xs[i].clone()).collect()
}

fn match_a_and_b(sols: impl IntoIterator<Item=Triple>) -> Vec<(Triple, Triple)> {
    let mut by_volume = HashMap::new();
    for sol in sols {
        by_volume.entry(sol.volume()).or_insert(vec![]).push(sol);
    }

    let mut out = vec![];
    for sols_of_eq_volume in by_volume.values().cloned() {
        let angles = sols_of_eq_volume.iter().map(Triple::angle).collect::<Vec<_>>();
        let perm = argsort(&angles);
        let mut sorted_sols = permute(&sols_of_eq_volume, &perm);
        while let Some(b_sol) = sorted_sols.pop() {
            // (don't worry about the O(n^2) here; more than one pair with the same volume
            //  is a rare occurrence)
            let a_sol = sorted_sols.remove(0);
            assert_eq!(a_sol.letter(), 'a');
            assert_eq!(b_sol.letter(), 'b');
            out.push((a_sol, b_sol))
        }
    }
    out
}

fn main() {
  use ::std::io::prelude::*;
  let mut sols = vec![];
  let volume_limit = std::env::args().nth(1).unwrap_or_else(|| {
    eprintln!("Usage:   cargo run --bin=all-pats -- MAX_VOLUME");
    std::process::exit(1);
  });
  let volume_limit = volume_limit.parse::<i64>().unwrap();
  'c: for c in 1i64..2 * volume_limit {
    eprint!("\r c = {}", c);
    ::std::io::stderr().flush().unwrap();
    let cc = c*c;
    'b: for b in 0.. {
      let bb3 = b*b*3;
      let aa = match cc - bb3 {
        aa if aa >= 0 => aa,
        _ => continue 'c,
      };
      if gcd(c, bb3) != 1 {
        continue 'b;
      }
      if let Some(a) = isqrt(aa) {
        let sol = Triple::validate_new(a, b, c);
        if sol.letter() != 'c' && sol.volume() < volume_limit {
          sols.push(sol);
        }
      }
    }
  }
  ::serde_json::to_writer(
      ::std::io::stdout(),
      &match_a_and_b(sols),
  ).unwrap();
}
