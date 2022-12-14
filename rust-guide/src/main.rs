fn main() {
    data_types();
    loop_kinds();
}

fn data_types() {
// Scalar types

    // Specify mut to make it so the variable can be changed
    let mut temp_int:i32 = 0;

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

    println!("u8   | u16 | u32 | u64 | u128 | arch");
    println!("i8   | i16 | i32 | i64 | i128 | arch");
    println!("f32  | f64");
    println!("bool | char");
    println!("tuple| array")


}

fn loop_kinds() {

// Loop
    
    // Loop is an infinite loop unless specifically broken out of
    let mut iterator: u32 = 1;
    let winner = loop {
        iterator *= iterator + 1;
        println!("Loop iteration #{}", iterator);

        if (iterator > 100) {
            break iterator; // Returns iterator, break can be used in this way to return a value
        }
    };


// While
    
    // While does what is expected
    let mut temp: u8 = 0;
    while (temp != 5) {
        println!("While iteration #{}", temp);
        temp += 1;
    }


// For

    // For can be used as an iterator like this
    let arr: [i32; 5] = [0,1,2,3,4];
    for item in arr {
        println!("First For iteration #{}", item);
    }

    // For example using range
    for item in (1..3) {
        println!("Second For iteration #{}", item);
    }
}

fn basic_ownership() {
// Ownership has 3 Rules

    // 1. Each value in Rust has an owner
    // 2. There can only be one owner at a time
    // 3. When the owner goes out of scope, the value is dropped


// Data interactions
    
    // Move
    // Data can be cloned if they are simple types with fixed sizes such as ints
    let x = 5;
    let y = x;
    let z =x;

    // But moving something like a string is a little more complicated, given that its dynamic nature
    let first_name: String = String::from("Nich");
    let copied_name: String = first_name; // first_name is dropped as the ownership of the value in first_name is transfered to copied_name

    // We would not be able to do another assignment from first_name like we did with the ints, because first_name doesn't exist now
    // copied_name now exists with the previous value of first_name

    //Clone
    // This is an example of a deep copy, aka cloning. If I simply want to fully copy the contents of str1 and put it in str2, I can use clone
    let str1: String = String::from("Hello");
    let str2:String = str1.clone();

    println!("We can now use both str1({}), and str2({})", str1, str2)

// 

}