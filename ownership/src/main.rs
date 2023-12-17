fn main() {
    let s1 = String::from("hello");
    let s2 = s1; // s1 can no longer be accessed (ownership transferred)
    println!("{}", s2);
    // deep copy - duplicate heap data
    let s3 = s2.clone();
    println!("{}", s3);
    // contradiction? NO -> int has known size and is stored entierly on the stack
    // trait: "Copy" is run on ints
    let x = 5;
    let y = x + 5;
    println!("{} {}", x, y);
    // ownership and functions
    let s = String::from("hello");
    takes_ownership(s);
    //  s was borrowed and freed, thus not accessible anymore
    // println!("{}", s);
    let x = 5;
    makes_copy(x);
    //  x was copied then copy was freed, thus is still accessible
    println!("{}", x);
    // return values and scope
    let s = gives_ownership();
    println!("{}", s);
    let s1 = String::from("hello");
    let s2 = takes_and_gives_ownership(s1);
    // s1 not accessible
    println!("{}", s2);

    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{}' is {}.", s2, len);
    // pass by reference
    // & = reference
    // * = dereference
    let s1 = String::from("hello");
    let len = calculate_length_reference(&s1);
    println!("The length of '{}' is {}.", s1, len);
    // make changes when passing by reference
    let mut s = String::from("hello");
    change(&mut s)
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_int: i32) {
    println!("{}", some_int);
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

fn takes_and_gives_ownership(a_string: String) -> String {
    a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn calculate_length_reference(s: &String) -> usize {
    s.len()
} // here s goes out of scope, but since it does not have ownership, it is not dropped

fn change(some_string: &mut String) {
    some_string.push_str(", world!");
}
