fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    let mut spaces = "   ";
    println!("spaces str {}, woow~~", spaces);
    spaces = spaces.len();
    println!("spaces num {}", spaces);


}