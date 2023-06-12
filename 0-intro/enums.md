# Enumeration in rust

Enums are used to create your custom types in rust

```rs
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

fn choose_direction(go: Direction) {
    match go {
        Direction::DOWN => println!("Going down"),
        Direction::LEFT => println!("Going left"),
        Direction::DOWN => println!("Going down"),
        Direction::UP => println!("Goint up"),
        _ => println!("We could not find match")
    }
}

fn main() {
    let dirr = Direction::UP;
    choose_direction(dirr)
}

```
