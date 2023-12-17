// constant
const TWO_HOURS_IN_SECONDS: u32 = 60 * 60 * 2;

fn main() {
    // variables and mutability
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    // constant
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("The value of TWO_HOURS_IN_SECONDS is: {TWO_HOURS_IN_SECONDS}");
    println!("The value of THREE_HOURS_IN_SECONDS is: {THREE_HOURS_IN_SECONDS}");
    // shadowing (not the same as mut)
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");
    // shadowing (type conversion - wont work with mut)
    let spaces = "    ";
    let spaces = spaces.len();
    println!("There are a number of {spaces} in the string provided!");

    // data types
    //  floating points
    let x = 2.0;
    let y: f32 = 3.0;
    println!("{x} {y}");
    //  numeric operations
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3;
    let remainder = 43 % 5;
    println!("{sum} {difference} {product} {quotient} {truncated} {remainder}");
    // boolean
    let t = true;
    let f: bool = false;
    println!("{t} {f}");
    // character type
    let c = 'z';
    let z: char = 'Z';
    let heart_eyed_cat = '😻';
    println!("{c} {z} {heart_eyed_cat}");
    // compound type
    //  tuples
    let tup = (500, 6.4, 1);
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup; // deconstruct tuple
    println!("{x} {y} {z}");
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    println!("{five_hundred} {six_point_four} {one}");
    //  arrays
    let a = [1, 2, 3, 4, 5];
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];
    println!("{first} {second}")
}
