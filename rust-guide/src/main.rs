fn main() {
    data_types();
}

fn data_types() {
// Scalar types

    // Supports u-int 8-128 (u8, u16, u32, u64, u128, arch)
    let first_int: u8 = 3;
    
    // Supports integer 8-128 (i8, i16, i32, i64, i128, arch)
    let second_int: i32 = 4090;

    // Floating point f32 or f64 (default)
    let float_example = 20.2;

    // Boolean
    let heads: bool = true;

    // Character
    let first_initial: char = 'N';


// Compound Types

    // Tuple
    let tup: (u8, f64, i32) = (1, 2.2, 3);
    let (x, y, z) = tup; // Pattern matching

    // Array
    let arr = [0,1,2,3,4,];




    

}