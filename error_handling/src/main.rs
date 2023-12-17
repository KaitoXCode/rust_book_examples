use std::error::Error;
use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::net::IpAddr;

fn main() -> Result<(), Box<dyn Error>> {
    // unrecoverable errors with panic! macro
    // panic!("crash and burn");

    // using panic! backtrace
    // let v = vec![1, 2, 3];
    // v[99];

    // recoverable errors with Result
    let greeting_file_result = File::open("hello.txt");
    let _greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };

    // Shortcuts for panic on error: unwrap and expect
    // unwrap will return the value inside the Ok. if Result is Err will panic
    let _greeting_file = File::open("hello.txt").unwrap();
    // overwrite the default panic msg - same functionality as above
    let _greeting_file =
        File::open("hello.txt").expect("hello.txt should be included in this project");
    // except > unwrap

    // propagating errors
    read_username_from_file();
    // a shortcut for propagating errer with the "?" operator
    read_username_from_file_question_mark();

    // to panic or not to panic
    to_panic_or_not_to_panic();

    // main func can return Result
    let greeting_file = File::open("hello.txt")?;
    Ok(())
}

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");
    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    let mut username = String::new();
    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_question_mark() -> Result<String, io::Error> {
    let mut username = String::new();
    // let username_file_result = File::open("hello.txt")?;
    // username_file.read_to_string(&mut username)?;
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
    // or simply:
    // fs::read_to_string("hello.txt")
}

fn using_box_trait() -> Result<(), Box<dyn Error>> {
    let greeting_file = File::open("hello.txt");
    Ok(())
}

fn to_panic_or_not_to_panic() {
    let home: IpAddr = "127.0.0.1"
        .parse()
        .expect("Hardcoded IP address should be valid");
}

// new type with validations in func
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}", value);
        }
        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}
