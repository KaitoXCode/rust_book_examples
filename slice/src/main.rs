fn main() {
    // string slices
    let s = String::from("hello world");
    // let hello = &s[0..5];
    // or: let hello = &s[..5]
    let hello = &s[..5];
    // let world = &s[6..11];
    let world = &s[6..];
    // whole string &s[..];
    println!("{}, {}!", hello, world);
    let find_hello = first_word(&s);
    println!("The first word is: {}", find_hello);
    // other slices:
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
