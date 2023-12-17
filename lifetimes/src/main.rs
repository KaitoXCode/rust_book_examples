// validating references with liftimes
// Lifetimes are another kind of generic that we’ve already been using.
// Rather than ensuring that a type has the behavior we want, lifetimes
// ensure that references are valid as long as we need them to be.

fn main() {
    let string1 = String::from("abcd");
    {
        let string2 = "xyz";
        let result = longest(string1.as_str(), string2);
        println!("The longest string is {}", result);
    }

    // lifetime annotations in struct definitions
    let novel = String::from("Call me Ishmael, Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("{:?}", i);
    println!("{:?}", i.part);

    // the static lifetime
    let s: &'static str = "I have a static lifetime.";
    println!("{s}");
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// lifetime annotations in struct definitions
// The lifetime annotation here means an instance of ImportantExcerpt can’t
// outlive the reference it holds in its part field
#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

// lifetime rules applied by the compiler
// base: (e.g. #1)
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
// first rule: (applies)
// fn first_word<'a>(s: &'a str) -> &str {...}
// second rule: (applies)
// fn first_word<'a>(s: &'a str) -> &'a str {...}

// base: (e.g. #2)
// fn longest(x: &str, f: &str) -> &str {...}
// first rule: (applies)
// fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &str {...}
// second rule: (not applicable) | there is more than one input lifetime

// lifetime annotations in method definitions
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}
// e.g. where the third lifetime elision rule applies:
// -> here are two input lifetimes, so Rust applies the first lifetime elision
// rule and gives both &self and announcement their own lifetimes. Then, because
// one of the parameters is &self, the return type gets the lifetime of &self,
// and all lifetimes have been accounted for.
impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

// generic type params, trait bounds, and lifetimes together
use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
