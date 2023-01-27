/*
 * hello_world is a method that prints "Hello, world!" to the console.
 */
pub fn hello_world() {
    println!("Hello, world!");
}

/*
 In both of the below functions, the let keyword is almost overused
 in order to ensure variables are put in the stack and not put in the heap,
 which is more likely to happen with mutable variables.
 */

/*
 * q_rsqrt32 is a direct port of the Q_rsqrt Quake inverse square algorithm.
 */
pub fn q_rsqrt32(x: f32) -> f32 {
    let i: u32 = x.to_bits(); // evil floating point bit hack
    let i: u32 = 0x5f3759df - (i >> 1); // what the fuck?
    let y: f32 = f32::from_bits(i);
    let y: f32 = y * (1.5 - 0.5 * x * y * y); // 1st iteration
    let y: f32 = y * (1.5 - 0.5 * x * y * y); // 2nd iteration, can be removed
    return y;
}

/*
 * q_rsqrt is a 64-bit port of the Q_rsqrt Quake inverse square algorithm complete with a new mystery constant (sqrt(2^1023) in hex for brevity).
 */
pub fn q_rsqrt(f_in: f64) -> f64 {
    let f_in_as_bits: u64 = f_in.to_bits(); // evil floating point bit hack
    let f_in_as_bits: u64 = 0x5fe6a09e667f3bc8 - (f_in_as_bits >> 1); // what the fuck? (now with more 64-bit)
    let f_out: f64 = f64::from_bits(f_in_as_bits);
    let f_out: f64 = f_out * (1.5 - 0.5 * f_in * f_out * f_out); // 1st iteration
    let f_out: f64 = f_out * (1.5 - 0.5 * f_in * f_out * f_out); // 2nd iteration, can be removed
    let f_out: f64 = f_out * (1.5 - 0.5 * f_in * f_out * f_out); // 3rd iteration, can be removed; provides full precision.
    return f_out;
}

pub fn rsqrt(x: f64) -> f64 {
    return (1.0) / x.sqrt();
}
