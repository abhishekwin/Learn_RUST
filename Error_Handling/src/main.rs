fn main() {
    use std::fs::File;
    use std::io::ErrorKind;

    println!("Hello, world!");
    //  In rust built in type called "Result" is used to handle errors
    //  Result<T, E> is a type that can be either Ok(T) or Err(E)

    // Using "match" to handle errors of both types ok and error
    match File::open("hello.txt") {
        Ok(file) => {
            println!("File opened successfully!");
            // Do something with `file`
        }
        Err(error) => {
            println!("Failed to open the file: {:?}", error);
        }
    }
    // if you know the operation should suceed , you can use "unwrap()""
    // let file = File::open("hello.txt").unwrap(); // Crashes if "hello.txt" doesn't exist

    // expect(message) – Like "unwrap", but with a custom error message.
    // let file2 = File::open("hello.txt").expect("Failed to open file:");

    // If file not exist then create a new file with name "hello.txt"
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("{}", "Problem creating the file: {e:?}"),
            },
            other_error => {
                panic!("{}", "Problem opening the file: {other_error:?}");
            }
        },
    };
}