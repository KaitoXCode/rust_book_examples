fn main() {
    function_pointers();
    returning_closures();
}

fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn function_pointers() {
    {
        let answer = do_twice(add_one, 5);
        println!("The answer is: {}", answer);
    }
    {
        let list_of_numbers = vec![1, 2, 3];
        // this:
        let list_of_strings: Vec<String> = list_of_numbers.iter().map(|i| i.to_string()).collect();
        println!("{:?}", list_of_strings);
        // or:
        let list_of_strings: Vec<String> =
            list_of_numbers.iter().map(ToString::to_string).collect();
        println!("{:?}", list_of_strings);
    }
    {
        #[derive(Debug)]
        enum Status {
            Value(u32),
            Stop,
        }
        let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
        println!("{:?}", list_of_statuses);
    }
}

fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

fn returning_closures() {
    let _closure = returns_closure();
}
