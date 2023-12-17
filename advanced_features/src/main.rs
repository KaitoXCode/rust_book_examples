// unsafe rust superpowers:
// -> dereference a raw pointer
// -> call an unsafe function or method
// -> access or modify a mutatble static variable
// -> implement an unsafe trait
// -> access fields of unions
use std::slice;

fn main() {
    dereference_a_raw_pointer();
    calling_unsafe_func_or_method();
    safe_abstraction_over_unsafe_code();
    using_extern_func_to_call_extern_code();
    accessing_or_moding_a_mut_static_var();
    implementing_unsafe_trait();
}

fn dereference_a_raw_pointer() {
    // creating raw pointers
    {
        let mut num = 5;
        let r1 = &num as *const i32;
        let r2 = &mut num as *const i32;
        // unsafe and will not panic
        unsafe {
            println!("r1 is: {}", *r1);
            println!("r2 is: {}", *r2);
        }
    }
    // arbitrary memory address
    {
        let address = 0x012345usize;
        let _r = address as *const i32;
        // unsafe and will panic
        // unsafe {
        //     println!("r is: {}", *r);
        // }
    }
}

unsafe fn dangerous() {}

fn calling_unsafe_func_or_method() {
    unsafe {
        dangerous();
    }
}

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    // Borrowing different parts of a slice is fundamentally okay
    // because the two slices aren’t overlapping, but Rust isn’t
    // smart enough to know this.
    // ---
    // let len = values.len();
    // assert!(mid <= len);
    // (&mut values[..mid], &mut values[mid..])
    // ---
    let len = values.len();
    let ptr = values.as_mut_ptr();
    assert!(mid <= len);
    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

fn safe_abstraction_over_unsafe_code() {
    {
        let mut v = vec![1, 2, 3, 4, 5, 6];
        let r = &mut v[..];
        let (a, b) = split_at_mut(r, 3);
        assert_eq!(a, &mut [1, 2, 3]);
        assert_eq!(b, &mut [4, 5, 6]);
    }
}

extern "C" {
    fn abs(input: i32) -> i32;
}

fn using_extern_func_to_call_extern_code() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}

static HELLO_WORLD: &str = "Hello, world!";
static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn accessing_or_moding_a_mut_static_var() {
    {
        println!("name is: {}", HELLO_WORLD);
    }
    {
        add_to_count(3);
        unsafe {
            println!("COUNTER: {}", COUNTER);
        }
    }
}

fn implementing_unsafe_trait() {
    unsafe trait Foo {
        // methods here
    }
    unsafe impl Foo for i32 {
        // method implementations go here
    }
}
