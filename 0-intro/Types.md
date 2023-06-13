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

# Tuples

Are a record of data

```rs
enum Access {
    Full,
}

fn one_two_three() -> (i32, i32, i32) {
    return (1, 2, 3);
}

fn main() {
    let numbers = one_two_three();
    let (x, y, z) = one_two_three();

    println!("{}", numbers.1);

    let (employee, access) = ("Jake", Access::Full);
}
```

## Ativity Solution

```rs
fn cartesian() -> (i32, i32, i32) {
    return (2, 5, 8);
}

fn main() {
    let (_x, y, _z) = cartesian();

    if y < 5 {
        println!("Less than five!!");
    } else if y > 5 {
        println!("Greater than five!!");
    } else {
        println!("Equal to five");
    }

    match y {
        4 => println!("Less than five!!"),
        5 => println!("Equal to five"),
        _ => println!("Less greater than five!!"),
    }
}

```

# Expresssions

```rs

enum Access {
    Admin,
    Manager,
    User,
    Guest,
}

fn main() {
    let access_level = Access::Guest;
    let can_access_file = match access_level {
        Access::Admin => true,
        _ => false,
    };

    println!("can access: {}", can_access_file);
}

```
