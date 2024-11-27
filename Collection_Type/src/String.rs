fn main() {
    let mut string_1 = String::new();
    string_1.push_str("Hello");
    println!("{}", string_1);

    // convert string literal to String

    let mut sample = "Hey Team".to_string();
    println!("Sample String: {}", sample);

    // Appending a string

    sample.push_str(" How are you?");
    println!("Sample String after appending: {}", sample);

    // Adding of two Strings

    let mut a1 = String::from("Hello");
    let mut a2 = String::from("World");
    // let a3 = a1 + &a2;
    println!("Using Format keywork :{}", format!("{} {}", a1, a2));
    // println!("a3: {}", a3);
}