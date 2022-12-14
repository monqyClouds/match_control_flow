#![allow(dead_code)]

#[derive(Debug)]

enum UseState {
    Alabama,
    Alaska,
    California,
    Massachussets,
    // 'New York',
    // North Carolina,
    // North Dakota
    Ohio,
    // South Carolina,
    Texas,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UseState),
}

fn main() {
    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State quarter from {:?}!", state);
                25
            }
        }
    }

    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let _six = plus_one(five);
    let _none = plus_one(None);

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn move_player(_num_spaces: u8) {}

    value_in_cents(Coin::Quarter(UseState::Texas));
    // value_in_cents(Coin::Dime);
    // value_in_cents(Coin::Nickel);
    // value_in_cents(Coin::Penny);
}
