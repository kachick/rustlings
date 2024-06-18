// enums1.rs
//
// No hints this time! ;)

#[derive(Debug)]
enum Message {
    // Expected values...?
    Quit = 42,
    Echo = 43,
    Move = 44,
    ChangeColor = 45,
}

fn main() {
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::ChangeColor);
}
