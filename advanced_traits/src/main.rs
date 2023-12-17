use std::ops::Add;

fn main() {
    default_generic_type_param_and_operator_overloading();
    syntax_for_disambiguation_methods_with_same_name();
    supertrait_require_trait_functionality_within_another_trait();
    newtype_pattern_to_implement_ext_traits_on_ext_types();
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(Debug, PartialEq)]
struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

fn default_generic_type_param_and_operator_overloading() {
    {
        assert_eq!(
            Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
            Point { x: 3, y: 3 }
        );
    }
    // customise Rhs
    {
        assert_eq!(Millimeters(1000).add(Meters(1)), Millimeters(2000));
    }
}

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

fn syntax_for_disambiguation_methods_with_same_name() {
    {
        let person = Human;
        person.fly();
    }
    {
        let person = Human;
        Pilot::fly(&person);
        Wizard::fly(&person);
        Human::fly(&person);
    }
    {
        println!("A baby dog is called a {}", Dog::baby_name());
        // <Type as Trait>::function(receiver_if_method, next_arg, ...);
        println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
    }
}

use std::fmt;

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

struct PointP {
    x: i32,
    y: i32,
}

impl fmt::Display for PointP {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl OutlinePrint for PointP {}

fn supertrait_require_trait_functionality_within_another_trait() {
    {
        let p = PointP { x: 1, y: 2 };
        println!("{:?}", p.outline_print());
    }
}

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn newtype_pattern_to_implement_ext_traits_on_ext_types() {
    {
        let w = Wrapper(vec![String::from("hello"), String::from("world")]);
        println!("w = {}", w);
    }
}
