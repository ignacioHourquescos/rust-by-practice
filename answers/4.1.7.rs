fn main() {
  let x = 1_000.000_1; // f64
  let y: f32 = 0.12; // f32
  let z = 0.01_f64; // f64
}

// The default type is f64 because on modern CPUs it’s roughly the same speed as f32 but is capable of more precision. All floating-point types are signed.