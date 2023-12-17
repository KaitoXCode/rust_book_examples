use std::option::Option;
// match VALUE {
//     PATTERN => EXPRESSION,
//     PATTERN => EXPRESSION,
//     PATTERN => EXPRESSION,
// }

fn main() {
    // match arms
    // let x = None;
    let x = Option::Some(5);
    match x {
        None => None,
        Some(i) => Some(i + 1),
    };
    println!("x = {:?}", x);

    conditional_if_let_expressions();
    while_let_conditional_loops();
    for_loops();
    let point = (3, 5);
    print_cordinates(&point);

    // pattern syntax
    matching_literals();
    matching_named_vars();
    multiple_patterns();
    matching_ranges_of_vals_with_dot_dot_equal();

    // destructuring to break apart values
    destructuring_structs();
    destructuring_enums();
    destructuring_nested_structs_enums();
    destructuring_structs_and_tuples();

    extra_conditionals_wit_match_guards();
    at_sybol_bindings();
}

fn conditional_if_let_expressions() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {color}, as the background");
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }
}

fn while_let_conditional_loops() {
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}

fn for_loops() {
    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }
}

fn print_cordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}

fn matching_literals() {
    let x = 1;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}

fn matching_named_vars() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {y}"),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {y}", x);
}

fn multiple_patterns() {
    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}

fn matching_ranges_of_vals_with_dot_dot_equal() {
    let x = 5;

    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    let x = 'c';

    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }
}

fn destructuring_structs() {
    struct Point {
        x: i32,
        y: i32,
    }

    {
        let p = Point { x: 0, y: 7 };
        let Point { x: a, y: b } = p;
        assert_eq!(0, a);
        assert_eq!(7, b);
    }
    {
        let p = Point { x: 0, y: 7 };
        let Point { x, y } = p;
        assert_eq!(0, x);
        assert_eq!(7, y);
    }
    {
        let p = Point { x: 0, y: 7 };
        match p {
            Point { x, y: 0 } => println!("On the x axis at {x}"),
            Point { x: 0, y } => println!("On the y axis at {y}"),
            Point { x, y } => println!("On neither axis: ({x}, {y})"),
        }
    }
}

fn destructuring_enums() {
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    {
        let msg = Message::ChangeColor(0, 160, 255);
        match msg {
            Message::Quit => println!("The Quit variant has no data to destructure."),
            Message::Move { x, y } => {
                println!("Move in the x direction {x} and in the y direction {y}")
            }
            Message::Write(text) => println!("Text message: {text}"),
            Message::ChangeColor(r, g, b) => {
                println!("Chang the color to red {r}, green {g}, blue {b}")
            }
        }
    }
}

fn destructuring_nested_structs_enums() {
    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(Color),
    }
    {
        let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));
        match msg {
            Message::ChangeColor(Color::Rgb(r, g, b)) => {
                println!("Chang the color to red {r}, green {g}, blue {b}")
            }
            Message::ChangeColor(Color::Hsv(h, s, v)) => {
                println!("Chang the color to hue {h}, saturation {s}, value {v}")
            }
            _ => (),
        }
    }
}

fn destructuring_structs_and_tuples() {
    struct Point {
        x: i32,
        y: i32,
    }
    {
        let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
        println!("{feet}, {inches}, {x}, {y}");
    }
}

fn extra_conditionals_wit_match_guards() {
    {
        let num = Some(4);
        match num {
            Some(x) if x % 2 == 0 => println!("The number {} is even", x),
            Some(x) => println!("The number {} is odd", x),
            None => (),
        }
    }
    {
        let x = Some(5);
        let y = 10;
        match x {
            Some(50) => println!("Got 50"),
            Some(n) if n == y => println!("Matched, n = {n}"),
            _ => println!("Default case, x = {:?}", x),
        }
        println!("at the end: x = {:?}, y = {y}", x);
    }
    {
        let x = 4;
        let y = false;
        match x {
            // this: (4 | 5 | 6) if y => ...
            // not : 4 | 5 | (6 if y) => ...
            4 | 5 | 6 if y => println!("yes"),
            _ => println!("no"),
        }
    }
}

fn at_sybol_bindings() {
    enum Message {
        Hello { id: i32 },
    }
    let msg = Message::Hello { id: 5 };
    match msg {
        Message::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range (3 to 7): {}", id_variable),
        Message::Hello { id: 10..=12 } => println!("Found an id in another range"),
        Message::Hello { id } => println!("Found some other id: {}", id),
    }
}
