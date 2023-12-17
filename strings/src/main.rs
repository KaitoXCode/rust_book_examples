// Collections - Strings

fn main() {
    // creating a new string
    let mut _s = String::new();
    // using the to_string method
    let data = "initial contents";
    let s = data.to_string();
    println!("sring: {}", s);
    // or:
    let s = "initial comments".to_string();
    println!("sring: {}", s);
    // using from method
    let s = String::from("initial contents");
    println!("sring: {}", s);

    // updating string
    // appending with push_str method
    let mut s = String::from("foo");
    s.push_str("bar");
    println!("sring: {}", s);
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s1: {}; s2: {}", s1, s2);
    // appending with push method
    let mut s = String::from("lo");
    s.push('l');
    println!("sring: {}", s);
    // concatenation winh the "+" operator
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // NOTE: s1 has been moved and cannot be used
    println!("concatted: {}", s3);
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("double concat: {}", s);
    // concat using format! macro:
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    // does not take ownership, s1 s2 s3 still valid
    let s = format!("{s1}-{s2}-{s3}");
    println!("format macro: {}", s);

    // indexing into strings is not possible!

    // slicing strings
    // bad idea
    let hello = "Здравствуйте";
    let s = &hello[0..4];
    println!("{s}");
    // methods for iterating over strings
    for c in "Зд".chars() {
        println!("{c}");
    }
    // or: byte method
    for b in "Зд".bytes() {
        println!("{b}");
    }
}
