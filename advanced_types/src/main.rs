fn main() {
    creating_type_synonyms_with_type_aliases();
    println!("Hello, world!");
}

fn creating_type_synonyms_with_type_aliases() {
    // Kilometers is a synonym to type i32 here
    type Kilometers = i32;
    {
        let x: i32 = 5;
        let y: Kilometers = 5;
        println!("x + y = {}", x + y);
    }

    // from:
    {
        let f: Box<dyn Fn() + Send + 'static> = Box::new(|| println!("hi"));
        fn takes_long_type(f: Box<dyn Fn() + Send + 'static>) {}
        fn returns_long_type(f: Box<dyn Fn() + Send + 'static>) -> Box<dyn Fn() + Send + 'static> {
            return f;
        }
    }
    // to: (using type alias)
    {
        type Thunk = Box<dyn Fn() + Send + 'static>;
        let f: Thunk = Box::new(|| println!("hi"));
        fn takes_long_type(f: Thunk) {}
        fn returns_long_type(f: Thunk) -> Thunk {
            return f;
        }
    }
}
