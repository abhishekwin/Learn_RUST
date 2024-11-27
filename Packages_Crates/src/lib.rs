mod reception_desk {
    pub mod booking_table {
        pub fn add_to_waitlist() {
            println!("Adding to waitlist");
        }
        pub fn seat_at_table() {}
    }

    mod oredering {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}

pub fn eat_at_restaurant() {
    // absolute path
    crate::reception_desk::booking_table::seat_at_table();
    // relative path
    reception_desk::booking_table::add_to_waitlist();

    let mut meal = back_of_house2::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // Next line did not compile if we chage the sesonal_fruit field because it is private
    // mael.seasonal_fruit = String::from("blueberries");
}

fn deliver_order() {}
// calling a function from another module in the same crate using super keyword
mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }
    fn cook_order() {}
}

// Using structs in module

mod back_of_house2 {
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
}

// using enum in module

mod order_salad {
    pub enum Salad {
        GreenSalad,
        VegSalad,
    }
}

pub fn eat_at_restaurant2() {
    let order1 = order_salad::Salad::GreenSalad;
    let order2 = order_salad::Salad::VegSalad;
}

use crate::reception_desk::booking_table;

pub fn eat_at_restaurant3() {
    booking_table::add_to_waitlist();
}

// using use keyword in anothe module
// but the use is now out of scope now we have to use super keyword

mod customer {
    pub fn eat_at_restaurant4() {
        super::booking_table::add_to_waitlist();
    }
}

// fethching module from another file

mod front_of_house;

pub use crate::front_of_house::reception_desk_1::booking_table_1;

pub fn eat_at_restaurant4() {
    booking_table_1::add_to_waitlist1();
}