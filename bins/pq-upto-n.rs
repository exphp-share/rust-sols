extern crate serde_json;
extern crate serde;
//#[macro_use]
extern crate serde_derive;

fn gcd(a: i64, b: i64) -> i64 {
  let mut a = a.abs();
  let mut b = b.abs();
  while b != 0 {
    let t = b;
    b = a % b;
    a = t;
  }
  a
}

fn main() {
  let max_area = std::env::args().nth(1).unwrap().parse::<i64>().unwrap();
  let mut solution_count = 0;
  let mut min_angle = f64::to_radians(60.);
  for q in 0.. {
    // This upper bound comes from Shallcross et al. Electronic structure of turbostratic graphene (2010).
    // Take equation 15 and substitute delta=3/gcd(p,3), gamma=gcd(p,3)2/gcd(pq,2). You get
    //
    //    N = (3u^2 + v^2) / [(2/gcd(pq,2))^2 gcd(p,3)]
    //
    // This denominator has a value >= 1 and <= 12. Taking maximum value we get the inequality:
    //
    //    N >= (3u^2 + v^2) / 12
    //    N >= 3u^2 / 12
    //   4N >=  u^2
    //
    if q*q > 4 * max_area {
        break
    }
    for p in 0..=q {
      if gcd(p, q) != 1 {
        continue;
      }
      let ag = 3*q*q - p*p;
      let bg = 2*q*p;
      let cg = 3*q*q + p*p;
      let g = gcd(gcd(ag, bg), cg);
      let a = ag / g;
      let b = bg / g;
      let c = cg / g;
      let angle = f64::acos(a as f64 / c as f64);
      assert!(g == 2 || g == 1 || g == 3 || g == 6);

      let area = if c%2==0 {c/2} else {c};
      if area <= max_area {
        solution_count += 1;
         //println!("@ ({}, {}) => ({}, {}, {}) ({})", p, q, a, b, c, area);
         println!("@ ({}, {}) => ({}, {}, {}) ({}) {}", p, q, a, b, c, area, angle.to_degrees());
        if c != 1 {
          min_angle = f64::min(min_angle, angle);
        }
      } else {
        // println!("  ({}, {}) => ({}, {}, {}) ({})", p, q, a, b, c, area);
        // println!("  ({}, {}) => ({}, {}, {}) ({}) {}", p, q, a, b, c, area, angle);
      }
      let _ = (a, b, c, g); // pretend used
    }
  }
  //println!("solutions found: {}", solution_count);
  println!("{} {}", solution_count, f64::to_degrees(min_angle));
}
