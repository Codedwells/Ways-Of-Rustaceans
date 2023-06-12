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
