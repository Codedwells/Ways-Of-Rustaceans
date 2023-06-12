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

# Structs in Rust

```rs

struct ShippingBox {
    depth: i32,
    width: i32,
    height: i32,
}


fn main() {
    let my_box = ShippingBox {
        depth: 3,
        width: 2,
        height: 7,
    };

    println!("{0}",my_box.width)
}

```

# Using structs , enums and match

```rs

enum Flavor {
    Orange,
    Cola,
}

struct Drinkbox {
    flavoring: Flavor,
    fluid_oz: f64,
}

fn print_flavor(my_box: Drinkbox) {
    match my_box.flavoring {
        Flavor::Orange => println!("Orange"),
        Flavor::Cola => println!("Cola"),
    }
}

fn main() {
    let my_box = Drinkbox {
        flavoring: Flavor::Orange,
        fluid_oz: 12.35,
    };

    print_flavor(my_box);
}

```
