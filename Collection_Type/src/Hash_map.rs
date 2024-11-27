fn main() {
    use std::collections::HashMap;
    let mut scores = HashMap::new();
    scores.insert(String::from("Real_madrid"), 1);
    scores.insert(String::from("Barcelona"), 0);
    println!("{:?}", scores);

    // Access value in HashMap
    let team_name = String::from("Barcelona");
    let score = scores.get(&team_name);
    println!("Score of {} is {}", team_name, score.unwrap());

    // Iterate over all the values in HashMap
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // Ownership in Hash_Map
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!
    // println!("{:?}", field_name);

    // Adding a key   value in hashMap

    let mut new_map = HashMap::new();
    new_map.insert(String::from("Blue"), 10);
    new_map.insert(String::from("Blue"), 20);
    println!("{:?}", new_map);
    new_map.entry(String::from("Yellow")).or_insert(300);

    new_map.entry(String::from("Blue")).or_insert(50);
    println!("{:?}", new_map);

    // updating a key value in HashMap
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}