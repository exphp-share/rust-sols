extern crate serde_json;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate num_rational;
extern crate all_pats;
use num_rational::Rational64;
use all_pats::{isqrt, gcd, qsqrt};

fn squarefree_sieve(limit: i64) -> Vec<bool> {
  let mut is_prime = vec![false; limit as usize];
  let mut is_squarefree = vec![true; limit as usize];

  is_prime[0] = false;
  is_prime[1] = false;
  is_squarefree[0] = false;
  for i in 0..limit {
    let ii = i*i;
    if ii >= limit { break; }
    if is_prime[i as usize] {
      for multiple in (ii..limit).step_by(i as usize) {
        is_prime[multiple as usize] = false;
      }
      for multiple in (ii..limit).step_by(ii as usize) {
        is_squarefree[multiple as usize] = false;
      }
    }
  }
  is_squarefree
}

type Triple = (i64, i64, i64);
fn find_primitive_solution(beta: i64, k: i64) -> Triple {
  use std::cmp::Ordering;
  for c in 1.. {
    for a in 0.. {
      if a*a > k*c*c {
        break;
      }
      if gcd(a, c) > 1 {
        continue;
      }
      for b in 0.. {
        match (a*a + beta*b*b).cmp(&(k*c*c)) {
          Ordering::Less => {},
          Ordering::Equal => {
            if gcd(b, c) == 1 && gcd(a, b) == 1 {
              return (a, b, c);
            }
          },
          Ordering::Greater => break,
        }
      }
    }
  }
  panic!("today is not your day...");
}

fn main() {
  let beta: i64 = ::std::env::args().skip(1).next().expect("beta?").parse().unwrap();
  let k: i64 = ::std::env::args().skip(1).next().expect("beta?").parse().unwrap();
  let squarefree = squarefree_sieve(1000);
  assert!(squarefree[beta as usize]);
  assert!(squarefree[k as usize]);

  use ::std::collections::HashMap;
  //let sols: Vec<(i64, i64, i64)> = ::serde_json::from_reader(::std::fs::File::open("sols.json").unwrap()).unwrap();
  //for sol in sols {
  //  if letter(sol) != 'c' {
  //    map.insert(sol, None);
  //  }
  //}
  let (a0, b0, c0) = find_primitive_solution(beta, k);
  let x0 = Rational64::new_raw(b0, c0);
  let y0 = Rational64::new_raw(a0, c0);
  //let mut gammas = HashMap::new();
  for q in 0..1000 {
    for p in 0..=q {
      if gcd(p, q) != 1 {
        continue;
      }
      let m = Rational64::new_raw(q, p);
      let z = m * x0 - y0;
      let A = m*m + beta;
      let B = -m * 2 * z;
      let C = z*z - k;

      //let square = |x| x*x;
      if let Some(d) = qsqrt(B*B - A*C*4) {
        let g_pos = *((-B*2 + d) / (A*2)).denom();
        let g_neg = *((-B*2 - d) / (A*2)).denom();
        assert_eq!(g_pos, g_neg, "({}, {})", p, q);
      } else {
        panic!("({}, {})", p, q);
      }
    }
  }
}

macro_rules! using {
    ([$($bindings:tt)*] -> $Ret:ty $block:block) => {
        _using_impl!{@munch{
            toks: [$($bindings)*]
            done: []
            body: [$block]
            ret: [$Ret]
        }
    }};
    ([$($bindings:tt)*] $($body:tt)+) => {
        _using_impl!{@munch{
            toks: [$($bindings)*]
            done: []
            body: [{$($body)+}]
            ret: [_]
        }
    }};
}

macro_rules! _using_impl {
    // FIXME: the reason this matches `ref` and `ref mut` with :tts was that I *thought* it would
    //        improve the spans used by unused_mut warnings.
    //        ...this backfired.
    //
    // warning: variable does not need to be mutable
    //  166 |           pat: [$a $ident]
    //      |                    ^^^^^^ help: remove this `mut`


    // Here we are trying to extract patterns of the form `ident`, `ref ident`, or `ref mut ident`,
    // each optionally followed by a type. We must do this with a muncher, because we need the ident.

    // Just `x`
    (@munch{
        // FIXME when $()? is stabilized, it should be used for $Type.
        //       (currently, more than one type annotation is allowed and the rest are silently ignored
        //        due to how we handle the default type of `_`. This is undesirable but hard to fix)
        toks: [$ident:ident $(: $Type:ty)* , $($more_bindings:tt)*]
        done: [$($done:tt)*]
        body: $body:tt
        ret: $ret:tt
    }) => {
        _using_impl!{@munch{
            toks: [$($more_bindings)*]
            done: [$($done)* {
                ident: [$ident]
                pat: [$ident]
                // type: will be a list containing one or two types, only the first of which will be used.
                //       By inserting `_` at the end, it gets used as the default.
                type: [$([$Type])* [_]]
            }]
            body: $body
            ret: $ret
        }}
    };
    // no trailing comma
    (@munch{ toks: [$ident:ident $(: $Type:ty)*] $($rest:tt)+ }) => {
        _using_impl!{@munch{ toks: [$ident $(: $Type)*,] $($rest)+}}
    };

    // `ref x`, or `mut x`
    (@munch{
        toks: [$a:tt $ident:ident $(: $Type:ty)* , $($more_bindings:tt)*]
        done: [$($done:tt)*]
        body: $body:tt
        ret: $ret:tt
    }) => {
        _using_impl!{@munch{
            toks: [$($more_bindings)*]
            done: [$($done)* {
                ident: [$ident]
                pat: [$a $ident]
                type: [$([$Type])* [_]]
            }]
            body: $body
            ret: $ret
        }}
    };
    (@munch{ toks: [$a:tt $ident:ident $(: $Type:ty)*] $($rest:tt)+ }) => {
        _using_impl!{@munch{ toks: [$a $ident $(: $Type)*,] $($rest)+}}
    };

    // `ref mut x`
    (@munch{
        toks: [$a:tt $b:tt $ident:ident $(: $Type:ty)* , $($more_bindings:tt)*]
        done: [$($done:tt)*]
        body: $body:tt
        ret: $ret:tt
    }) => {
        _using_impl!{@munch{
            toks: [$($more_bindings)*]
            done: [$($done)* {
                ident: [$ident]
                pat: [$a $b $ident]
                type: [$([$Type])* [_]]
            }]
            body: $body
            ret: $ret
        }}
    };
    (@munch{ toks: [$a:tt $b:tt $ident:ident $(: $Type:ty)*] $($rest:tt)+ }) => {
        _using_impl!{@munch{ toks: [$a $b $ident $(: $Type)*,] $($rest)+}}
    };

    // no more! no more!
    (@munch{ toks: [] done:$bindings:tt body:$body:tt ret:$ret:tt }) => {
        _using_impl!{@finish{ bindings:$bindings body:$body ret:$ret }}
    };

    (@finish{
        bindings: [$({
            ident: [$ident:ident]
            pat: [$pat:pat]
            type: [[$Type:ty] $($unused_types:tt)*]
        })*]
        body: [$body:block]
        ret: [$Ret:ty]
    }) => {{
        $(let $pat: $Type = $ident;)*
        $crate::CallHelper(($($ident),*)).call(|$($ident),*| $body)
    }};

    ($($tok:tt)+) => {
        compile_error!(concat!{"using! macro: unknown error at `", stringify!{$($tok)+}}, "`")
    };
}

/// For calling a function in a manner that tricks rust into applying
/// its closure argument type inference hack
#[doc(hidden)]
pub struct CallHelper<T>(T);

macro_rules! impl_call_helper_all {
    ($a0:ident : $A0:ident, $($a:ident : $A:ident,)*) => {
        impl_call_helper!{ $a0:$A0, $($a:$A,)* }
        impl_call_helper_all!{ $($a:$A,)* }
    };
    () => {};
}
macro_rules! impl_call_helper {
    ($($a:ident : $A:ident,)*) => {
        impl<$($A),*> CallHelper<($($A,)*)> {
            pub fn call<R>(
                self,
                // this is deliberately `fn` instead of `impl FnOnce`; that's how access
                // to unlisted bindings is forbidden. (albeit through an awful type error)
                function: fn($($A),*) -> R,
            ) -> R {
                let CallHelper(($($a,)*)) = self;
                function($($a),*)
            }
        }
    };
}
impl_call_helper_all!{
    a:A, b:B, c:C, d:D, e:E, f:F,
    g:G, h:H, i:I, j:J, k:K, l:L,
    m:M, n:N, o:O, p:P, q:Q,
    s:S, t:T, u:U, v:V, w:W, x:X,
    y:Y, z:Z,
}


fn foo() {
    let a = vec![()];
    let b = vec![()];
    let mut c = vec![()];
    let d = vec![()];
    let mut e = vec![()];

    using!{[b, mut c, ref d, ref mut e] {
        let _ = b; // b is Vec<()>. It was moved.
        c.sort(); // c is Vec<()>. It was moved.
        let _ = d.iter(); // d is &Vec<()>
        let _ = e.iter_mut(); // e is &mut Vec<()>
        // let _ = a.iter(); // a cannot be used.
    }}
}
