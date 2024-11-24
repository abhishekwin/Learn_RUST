fn main() {
    println!("Variables with conditions");
     run_loop(1,1,"loop");
     println!("Loop");
     run_loop(1,5,"while");
     println!("While loop");
     run_loop(1,5,"for");
     println!("For Loop");
     run_loop(1,5,"for_rev");
     println!("Reverse Loop")

}



fn run_loop(mut start:u8, mut end : u8,  loop_type : &str) -> &str {
    // if condition
    end = if end > start {end}else{start+1};

     if loop_type =="loop" {
        // loop for continously 
        loop{
            if start==end{
                break;
            }
            println!("{start}");
            start+=1;
        }

     } else if loop_type=="while"{
        // syntax for while loop
        while start < end {
            println!("{start}");
            start+=1;
        }
     } else if loop_type =="for"{
        for i in start .. end {
            println!("{}",{i});
    
        }
     } else if loop_type=="for_rev" {
        for i in (start .. end).rev(){
            println!("{}",{i});
        }
     }
loop_type

}  