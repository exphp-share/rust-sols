extern crate serde_json;
extern crate serde;
#[macro_use]
extern crate serde_derive;


fn main() {
  use ::std::collections::HashMap;
  let sols: Vec<(u32, u32, u32)> = ::serde_json::from_reader(::std::fs::File::open("sols.json").unwrap()).unwrap();
  let mut map = HashMap::new();
  for (a, b, c) in sols {
    if f64::acos(a as f64 / c as f64) / std::f64::consts::PI * 180.0 < 30.0 {
      continue;
    }
    println!("({}, {}, {})", a, b, c);
    let v = if c % 2 == 0 { c / 2 } else { c };
    assert_eq!(v % 2, 1);
    assert_eq!(map.insert((b, v), (a, b, c)), None);
  }
}
