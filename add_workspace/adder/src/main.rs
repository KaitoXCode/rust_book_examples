// run only this package in workspace
// cargo run -p adder
use add_one;

fn main() {
    let num_left = 10;
    let num_right = 10;
    println!(
        "Hello, world! {num_left} plus {num_right} is {}!",
        add_one::add(num_left, num_right)
    );
}
