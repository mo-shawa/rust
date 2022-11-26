fn main() {
    // as usual, Booleans:
    let _is_something = true;

    // for nums, different types based on:
    // signed / unsigned :
    //      signed means it can hold both positive and negative value (the sign being + or -)
    // how many bits allocated:
    //      i32 allocates 32 bits
    //      u8 allocates 8 bits
    let _num = 10; // i32 => (i: signed integer, 32: Number of bits allocated)
    let _small_num: u8 = 255; // u8 => (u: unsigned integer [positive only], 8: number of bits allocated)

    // the maximum number held by an and unsigned type is 2^n - 1
    // e.g. u8 => 2^8 -1 = 255, so 0 to 255

    // the number range held by an unsigned type is between -2^(n-1) <= number <= 2^(n-1) - 1
    // e.g. i8 => -2^7 to 2^7 -1, so -128 to 127
    let _i8: i8 = 127;

    // system types:
    // usize and isize will take the bit value from your operating system (32bit OS => 32bit integer)
    let _sysnum_i: isize = -1258;
    let _sysnum_u: usize = 138183;
}
