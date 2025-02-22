fn main() {
  let a = 347384_i64;
  let b = 2938425_i64;

  let c = modular_inverse(a, b);

  assert!(a * c % b == 1);
}


fn modular_inverse(a: i64, b: i64) -> i64 {
  let (mut s, mut old_s) = (0, 1);
  let (mut g, mut old_g) = (b, a);
  while g != 0 {
      let q = old_g / g;
      let (new_r, new_s) = (old_g - q * g, old_s - q * s);
      old_g = g; // Not using destructuring to support low version
      g = new_r; // AtCoder is using 1.42.0
      old_s = s;
      s = new_s;
  }

    if old_s < 0 {
      old_s = (old_s % b) + b;
    }
    old_s
}