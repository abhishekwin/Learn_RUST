fn main() {
    println!("Learning Arrays & Tuple");
    tuple_test();
    array_test();
}


fn tuple_test(){
    println!("\n\nTupule Example");
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("Accescig tuple value by destructring");
    let (a,b,_)=tup;
    
    println!("Value of tuple in a is {a}");
    println!("Value of tuple in b is {b}");
    
    println!("\nAccesing value by index");

    println!("First value in tuple is {}",tup.0);
    println!("Second value in tuple is {}",tup.1);
    println!("Third value in tuple is {}",tup.2);
}


fn array_test(){
    println!("\n Array Example");
    let a=[1,2,3,4,5];
    println!("Print The array: \n{:?}",a);

    let months = ["JAN","FEB","MAR","APR","MAY","JUN","JUL","AUG","SEP","OCT","NOV","DEC"];

    println!("Print the awesome array: \n{:#?}",months);
    println!("Accesing the First Month:{}",months[0])
}