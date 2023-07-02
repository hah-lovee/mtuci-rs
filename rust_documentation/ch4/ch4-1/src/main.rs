fn main() {
    let s = String::from("hello"); 

    let s = takes_ownership(s);             

    let x = 5;                      
    
    makes_copy(x);     

    println!("main s -> {}", s);     
    println!("main x -> {x}");      

} 
fn takes_ownership(some_string: String) -> String { 
    println!("{}", some_string);
    some_string
} 

fn makes_copy(some_integer: i32) { 
    println!("{}", some_integer);
} 
