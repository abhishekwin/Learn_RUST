fn main(){
    println!("Hello Abhishek");

    // now we are calling another function
let name = 24;
public_function(name);
// storing the return value from another funcction
let value = return_value(name);
println!("Sum is : {value}");
}

fn public_function(s : u8){
    println!("My age is  : {s}");
}

fn return_value(a:u8) -> u8{
    let a1 = 5;
    a1+a    //if we don't use semicolom ";" means we are returning a value.
}