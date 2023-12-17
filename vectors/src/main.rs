// Collections - Vectors

fn main() {
    // creating new empty vector
    let v: Vec<i32> = Vec::new();
    println!("v: {:?}", v);
    // creating new vector with type inference
    let mut v = vec![1, 2, 3];
    println!("v: {:?}", v);
    // updating a vector
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    // reading elements of vectors
    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    println!("the third elem is {}", third);
    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("the third elem is {}", third),
        None => println!("there is no third elem"),
    }
    // itterating over the values in a vector
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }
    // itterate and mutate vector
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }

    // using an enum to store multiple types
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    println!("{:#?}", row)
}
