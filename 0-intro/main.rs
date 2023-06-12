enum Color {
    Red,
    Blue,
    Green,
    Yellow,
}

fn choose_color(go: Color) {
    match go {
        Color::Red => println!("red"),
        Color::Blue => println!("Blue"),
        Color::Green => println!("Green"),
        Color::Yellow => println!("Yellow"),
        _ => println!("We could not find match"),
    }
}

fn main() {
    let dirr = Color::Green;
    choose_color(dirr)
}
