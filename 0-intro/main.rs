enum Color {
    Red,
    Green,
    Blue,
}

struct Person {
    age: i32,
    name: String,
    fav_color: Color,
}

impl Person {
    fn new(age: i32, name: &str, fav_color: Color) -> Self {
        return Self {
            age,
            name: name.to_owned(),
            fav_color,
        };
    }

    fn print_info(&self) {
        let fav_c: String = match self.fav_color {
            Color::Red => "Red".to_owned(),
            Color::Green => "Green".to_owned(),
            Color::Blue => "Blue".to_owned(),
        };
        println!("My name is :{} and {} is my favourite color.", self.name, fav_c);
    }
}

fn main() {
    let more_people = vec![
        Person::new(9, "Thomas", Color::Green),
        Person::new(1, "Bryson", Color::Red),
        Person::new(13, "Jude", Color::Green),
        Person::new(8, "Tione", Color::Red),
        Person::new(2, "Halle", Color::Blue)
    ];

    for person in more_people {
        if person.age < 10 {
            person.print_info();
        }
    }
}
