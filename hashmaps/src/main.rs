// collections - hash maps
use std::collections::HashMap;

fn main() {
    // creating a new hash map
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("{:?}", scores);

    // accessing values in a hash map
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("{} team score: {}", team_name, score);

    // iterate over hash map
    for (key, value) in &scores {
        println!("key: {key}; value: {value}");
    }

    // hash map and ownership
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point!
    // they were moved into the hash map
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(&field_name, &field_value);
    // passing reference field_name and field_value are still valid
    println!("{:?} = key: {field_name}; val: {field_value}", map);

    // Updating a hash map
    //  overwritting a value
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25); // (kw v) was replaced! NO KEY ERROR
    println!("{:?}", scores);
    //  check if key exists
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.entry(String::from("Yellow")).or_insert(50); // key does not exist = CHANGE
    scores.entry(String::from("Blue")).or_insert(50); // key exists = NO CHANGE!
    println!("{:?}", scores);
    //  update based on old value
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    } // mut reference out of scope after loop
    println!("{:?}", map);
}
