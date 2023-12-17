mod front_of_house;

// external code would have to call the add_to_waitlist function by using the path
// "restaurant::front_of_house::hosting::add_to_waitlist()"
// use crate::front_of_house::hosting;
// but, with "pub"
// external code can call the add_to_waitlist function by using the path
// "restaurant::hosting::add_to_waitlist()"
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // relative path
    front_of_house::hosting::add_to_waitlist();

    // bringing paths into scope with "use" keyword
    hosting::add_to_waitlist();
    // order a Breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // change our mind about bread
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
    // the next line wont compile if we uncomment it; were not allowed
    // to see or modify the seasonal_fruit
    // meal.seasonal_fruit = String::from("blueberries");

    let _order1 = back_of_house::Appetizer::Soup;
    let _order2 = back_of_house::Appetizer::Salad;
}

mod customer {
    // need to bring into scope of child module
    // providing new names with the as keyword
    use crate::front_of_house::hosting as front_hosting;

    pub fn eat_at_restaurant() {
        front_hosting::add_to_waitlist();
    }
}

// grab code from parent module
fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        // here "super::" goes to the parent module: "back_of_house"
        super::deliver_order();
    }

    fn cook_order() {}

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }
}
