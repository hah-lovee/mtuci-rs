// fn main() {
//     for i in 1..=100 {

//         if i%3 == 0 && i%5 == 0{
//             println!("FizzBuzz");
//         } else if i%3 == 0 {
//             println!("Fizz");
//         } else if i%5 == 0 {
//             println!("Buzz");
//         } else {println!("{i}")};
//     }
// }

fn main () {
    for i in 1..=100{

        let mut answer = "".to_string();
        if i % 3 == 0 {
            answer.push_str("Fizz");
        } if i % 5 == 0 {
            answer.push_str("Buzz");
        } if answer == "" {
            answer.push_str(&i.to_string());
        }
        println!("{answer}");
    }

}