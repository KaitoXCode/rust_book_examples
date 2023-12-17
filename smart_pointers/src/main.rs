use std::borrow::BorrowMut;
use std::mem::drop;
use std::ops::Deref;

// enum List {
//     Cons(i32, List),
//     Nil,
// }

enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use crate::ListRc::{ConsRc, NilRc};
use crate::Rclist::{Rccons, Rcnil};
use std::cell::RefCell;
use std::rc::{Rc, Weak};

fn main() {
    // using a box to store data on the heap
    let b = Box::new(5);
    println!("b = {}", b);

    // let _list = Cons(1, Cons(2, Cons(3, Nil)));
    let _list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    // treating smart pointers like regular references with the Deref trait
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
    // using Box<T> like a reference
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
    // defining our own smart pointer
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let m = MyBox::new(String::from("Rust"));
    hello(&m);

    // running code on cleanup with the drop trait
    let _c = CustomerSmartPointer {
        data: String::from("my stuff"),
    };
    let _d = CustomerSmartPointer {
        data: String::from("other stuff"),
    };
    println!("2x CustomerSmartPointers created.");

    // dropping a value early wiht std::mem::drop
    let c = CustomerSmartPointer {
        data: String::from("some data"),
    };
    println!("1x CustomerSmartPointer created.");
    drop(c);
    println!("CustomerSmartPointer dropped before the end of main.");

    // using Rc<T> to share data and RefCell to make immutabe mutable
    let value = Rc::new(RefCell::new(5));
    let a = Rc::new(ConsRc(Rc::clone(&value), Rc::new(NilRc)));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let _b = ConsRc(Rc::new(RefCell::new(3)), Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let _c = ConsRc(Rc::new(RefCell::new(4)), Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
    // *value.borrow_mut() += 10;
    println!("a after = {:?}", a);
    println!("b after = {:?}", b);

    // Creating a reference cycle
    let a = Rc::new(Rccons(5, RefCell::new(Rc::new(Rcnil))));
    println!("a initial rc count : {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());
    let b = Rc::new(Rccons(10, RefCell::new(Rc::clone(&a))));
    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());
    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }
    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));
    // uncomment the next line to see that we have a cycle;
    // it will overflow the stack
    // println!("a next item = {:?}", a.tail());

    // preventing reference cycles turning Rc<T> to Weak<T>
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    // visualising changes to strong_count and weak_count
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );
    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch)
        );
        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf)
        );
    }
    println!("leaf parent : {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );
}

// defining our own smart pointer
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// implement the Deref trait to make "*y" work
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // return 0th item in the tuple struct
        &self.0
    }
}

// deref coercion
fn hello(name: &str) {
    println!("Hello, {name}!");
}

// running code on cleanup with the drop trait
struct CustomerSmartPointer {
    data: String,
}

impl Drop for CustomerSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomerSmartPointer with data `{}`!", self.data);
    }
}

// using Rc<T> to share data
#[derive(Debug)]
enum ListRc {
    ConsRc(Rc<RefCell<i32>>, Rc<ListRc>),
    NilRc,
}

#[derive(Debug)]
enum Rclist {
    Rccons(i32, RefCell<Rc<Rclist>>),
    Rcnil,
}

impl Rclist {
    fn tail(&self) -> Option<&RefCell<Rc<Rclist>>> {
        match self {
            Rccons(_, item) => Some(item),
            Rcnil => None,
        }
    }
}

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}
