// finding the largest number in a list

fn find_largest_no(list: &[i32]) -> &i32 {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
// finding the largest char in a list
fn find_largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = find_largest_no(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = find_largest_char(&char_list);
    println!("The largest char is {}", result);

    // using generic type to find the largest number or char
    let number_list = vec![34, 50, 25, 100, 65];
    let result = find_largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = find_largest(&char_list);
    println!("The largest char is {}", result);

    // We can use generic types in the struct as well
    #[derive(Debug)]
    struct Value<T> {
        x: T,
        y: T,
    }

    let value1 = Value { x: 1, y: 2 };
    println!("Value1: {:#?}", value1);

    // if you give different values this give error in next line
    // let value2 = Value { x: 1, y: 2.0 };
    // println!("Value2: {:#?}", value2);
    #[derive(Debug)]
    // for giving different generic value we can do this
    struct Demo<T, U> {
        a: T,
        b: U,
    }

    let demo = Demo { a: 1, b: 2.0 };
    println!("Demo: {:#?}", demo);
}

// In generic type we can create only one function to find the largest where it is number or char

fn find_largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}