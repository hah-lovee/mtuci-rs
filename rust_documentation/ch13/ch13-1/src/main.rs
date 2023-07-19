#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn getaway (&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.counter())
    }

    fn counter(&self) -> ShirtColor {
        let mut red_count = 0;
        let mut blue_count = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => red_count += 1,
                ShirtColor::Blue => blue_count += 1,
            }
        }
        if red_count > blue_count {
            ShirtColor::Red
        } else { ShirtColor::Blue }
    }
}

fn main() {
    let store = Inventory {
        shirts: vec!(ShirtColor::Blue, ShirtColor::Blue, ShirtColor::Red)
    };

    let user1 = Some(ShirtColor::Red);
    let user2 = None;

    let result_1 = store.getaway(user1);
    let result_2 = store.getaway(user2);

    println!("user1 -> {:?}, user2 -> {:?}", result_1, result_2);

}