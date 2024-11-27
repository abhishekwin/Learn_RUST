fn main() {
    let v: Vec<i32> = Vec::new();
    let mut v = vec![1, 2, 3];
    println!("Vector: {:#?}", v);
    v.push(4);
    v.push(5);
    println!("Vector: {:#?}", v);

    // Reading elements of vector

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    let fourth: Option<i32> = v.get(3).copied();
    match fourth {
        Some(fourth) => println!("The fourth element is {}", fourth),
        None => println!("There is no fourth element"),
    }

    // let does_not_exist = &v[100];
    // println!("Does not exist {}", does_not_exist);

    // Attempt to add a value to vector when holding a value
    // let first = &v[0];
    // v.push(6); // it gives error
    // println!("First element {}", first);

    // Iterating over a values in vector
    let mut a = v.clone();
    for i in &mut a {
        *i += 50;
    }

    println!("Vector: {:#?}", a);
    println!("Vector: {:#?}", v);

    // Enums in vector
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    println!("Vector: {:#?}", row);
}