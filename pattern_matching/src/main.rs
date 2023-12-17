// the match control flow construct
#[derive(Debug)]
enum UsState {
    // patterns that bind to values
    Alabama,
    Alaska,
    // --etc--
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quater(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    // match expression
    match coin {
        // match arm #1:
        // {pattern} => {some code}
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        // match arm #2:
        Coin::Nickel => 5,
        // match arm #3:
        Coin::Dime => 10,
        // match arm #4:
        Coin::Quater(state) => {
            println!("State quater from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        // what is i? i binds to the value contained in Some (so <T> of i is i32)
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let coin1 = Coin::Penny;
    let val1 = value_in_cents(coin1);
    println!("coin1 value: {}", val1);
    let coin2 = Coin::Nickel;
    let val2 = value_in_cents(coin2);
    println!("coin2 value: {}", val2);
    let coin3 = Coin::Dime;
    let val3 = value_in_cents(coin3);
    println!("coin3 value: {}", val3);
    let coin4 = Coin::Quater(UsState::Alabama);
    let val4 = value_in_cents(coin4);
    println!("coin4 value: {}", val4);
    // patterns that bind to values
    let coin5 = Coin::Quater(UsState::Alaska);
    let val5 = value_in_cents(coin5);
    println!("coin5 value: {}", val5);

    // matching with Option<T>
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("six: {:?}; none {:?}", six, none);

    // catch-all patterns and the _ placeholder
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        // other (named by you) covers every other possible value
        other => move_player(other),
        // OR: (if no var needed)
        // _ => reroll(),
        // OR: (if nothing needs to happen) - empty tuple
        // _ => ()
    }
    // base
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }
    // concise control flow with if let
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max)
    }
    println!("Hello, world!");
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(_num_spaces: u8) {}
fn reroll() {}
