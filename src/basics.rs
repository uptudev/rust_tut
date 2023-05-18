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
 * For values between 0 and 1, q_rsqrt is within a negligable margin of error when compared to the rsqrt calculation, while being MUCH faster and less demanding.
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

/*
 * rsqrt is a slower method that returns the true inverse square for referencing. This is how your average Python dev would tackle the inverse square question.
 */
pub fn rsqrt(x: f64) -> f64 {
    return (1.0) / x.sqrt();
}

/*
 * first_word takes a string slice as an argument and returns the first word in said string slice as another string slice.
 */
// 2023/05/15: What the actual fuck is this function?? WHY DID I WRITE THIS?????
pub fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes(); // Deconstruct string slice into individual char bytes

    for (i, &item) in bytes.iter().enumerate() {
        // For each byte (character) in the string slice, pull a reference to that byte and create counter int i
        if item == b' ' {
            // If the byte read is the same as the byte literal for a space...
            return &s[..i]; // Return a string slice spanning from position 0 to position i (from the first char, inclusive, to the space char encountered, exclusive)
        }
    }

    &s // If no space is encountered (and thus no return called), return the entire slice, as it is a word in itself.
}

// 2023/05/16: Like this accomplishes the exact same thing with one line, so these have no reason to
// be functions at all.
pub fn first_word_but_not_awful(s: &str) -> &str {
    s.split(' ').collect::<Vec<_>>()[0]
}

                                                                                                                                                                            
