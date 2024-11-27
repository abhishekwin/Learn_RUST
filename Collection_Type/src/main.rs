use std::collections::HashMap;
fn main() {
    println!("Hello Abhishek, First we are talking about Collection Types");
    println!("There are 3 types of Collection Types");
    println!("1. Vectors");
    println!("2. Strings");
    println!("3. Hashmaps");
    println!("Now let's learn about Vectors");

    // Define a  new vector type
    let mut numbers: Vec<i32> = Vec::new(); // Create a empty vector
    numbers.push(5);
    numbers.push(6); // add elements
    numbers.push(7);
    println!("Numbers: {:#?}", numbers);

    println!("First element {}", numbers[0]);
    println!("length of vector {}", numbers.len());
    println!("POP the last value from the vetor: {:#?}", numbers.pop());
    println!("Numbers: {:#?}", numbers);

    numbers.push(8);
    numbers.push(9);
    println!("Numbers: {:#?}", numbers);
    println!("length of vector {}", numbers.len());

    println!(
        "Remove middle value from the vector: {:#?}",
        numbers.remove(2)
    );
    println!("Numbers: {:#?}", numbers);
    println!("length of vector {}", numbers.len());

    println!("\nNow let's learn about Strings");

    let mut string = String::new(); // Create a empty string
    string.push_str("Hello Abhishek");
    string.push_str("Hello Abhishek Again");
    println!("String: {:#?}", string);

    println!("\nNow let's learn about Hashmaps");

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("Scores: {:#?}", scores);
}