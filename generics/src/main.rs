fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// with generics
// fn largest_generic<T>(list: &[T]) -> &T {
//     let mut largest = &list[0];
//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }
//     largest
// }

// in struct definitions
struct Point<T> {
    x: T,
    y: T,
}

// in struct def, but with diff types
struct PointMixed<X1, Y1> {
    x: X1,
    y: Y1,
}

// in enum definitions
// enum Option<T> {
//     Some(T),
//     None,
// }

// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

// in method definitions
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// generics - constrained
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

impl<X1, Y1> PointMixed<X1, Y1> {
    fn mixup<X2, Y2>(self, other: PointMixed<X2, Y2>) -> PointMixed<X1, Y2> {
        PointMixed {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    // base
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest_char(&char_list);
    println!("The largest char is {}", result);

    // with generics
    // let result_num = largest_generic(&number_list);
    // println!("The largest number is {}", result_num);
    // let result_char = largest_generic(&char_list);
    // println!("The largest char is {}", result_char);

    // in struct definitions
    let _integer = Point { x: 5, y: 10 };
    let _float = Point { x: 1.0, y: 4.0 };
    // in struct def, but with diff types
    let _both_int = PointMixed { x: 5, y: 10 };
    let _both_float = PointMixed { x: 1.0, y: 4.0 };
    let _mixed = PointMixed { x: 5, y: 4.0 };

    // in enum definitions
    // see Option & Result enums above

    // in method definitions
    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());

    // generics - constrained
    let p = Point { x: 5.0, y: 10.0 };
    let d = p.distance_from_origin();
    println!("{d}");

    let p1 = PointMixed { x: 5, y: 10.4 };
    let p2 = PointMixed { x: "hello", y: 'c' };
    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
