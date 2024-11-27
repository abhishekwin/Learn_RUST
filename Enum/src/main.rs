enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

enum Coin2 {
    Penny,
    Nickel,
    Dime,
    Quarter(String),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn value_in_cents2(coin: Coin2) -> u8 {
    match coin {
        Coin2::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin2::Nickel => 5,
        Coin2::Dime => 10,
        Coin2::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn main() {
    let coin = value_in_cents(Coin::Penny);
    println!("Penny is {}", coin);

    let coin2 = value_in_cents2(Coin2::Quarter(String::from("New York")));
    println!("Quarter is {}", coin2);
}