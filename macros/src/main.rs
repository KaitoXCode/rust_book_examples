// - Custom #[derive] macros that specify code added with the derive attribute used on structs and enums
// - Attribute-like macros that define custom attributes usable on any item
// - Function-like macros that look like function calls but operate on the tokens specified as their argument

fn main() {
    declarative_macros();
    procedural_macros();
    attribute_like_macros();
    function_like_macros();
}

fn declarative_macros() {
    use macros::myvec;
    let _v: Vec<u32> = vec![1, 2, 3];
    // see myvec in lib.rs
    let _v: Vec<u32> = myvec![1, 2, 3];
}

fn procedural_macros() {
    use macros::HelloMacro;

    struct Pancakes;

    impl HelloMacro for Pancakes {
        fn hello_macro() {
            println!("Hello, Macro! My name is Pancakes!");
        }
    }

    Pancakes::hello_macro();
}

fn attribute_like_macros() {
    // https://doc.rust-lang.org/book/ch19-06-macros.html#attribute-like-macros
}

// #[proc_macro]
// pub fn sql(input: TokenStream) -> TokenStream {
//     input
// }

fn function_like_macros() {
    // let sql = sql!(SELECT * FROM posts WHERE id=1);
    // https://doc.rust-lang.org/book/ch19-06-macros.html#function-like-macros
}
