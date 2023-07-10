#[derive(Debug)]
struct Rectangle{
    width: u32,
    height: u32,
} 


fn main() {
    let rect1 = Rectangle{
        width: 30,
        height: 50
    };

    // let rect1 = (30, 50);
    // let width1 = 30;
    // let height1 = 50;

    println!("rect is {:?}", rect1)
}    

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(&rect1)
//     );
// }

// fn area(dimensions: &Rectangle) -> u32 {
//     dimensions.width * dimensions.height
// }