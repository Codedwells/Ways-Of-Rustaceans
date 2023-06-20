# Derive

```rs
#[derive(Debug, Clone, Copy)]
enum Color {
    Red,
    Green,
    Blue,
}

#[derive(Debug, Clone, Copy)]
struct Person {
    age: i32,
    fav_color: Color,
}

fn print_person(me: Person) {
    println!("{:?}", me);
}

fn main() {
    let me = Person {
        age: 19,
        fav_color: Color::Red,
    };

    println!("{:?}", me.fav_color);,,, 
    print_person(me);
    print_person(me);
}

```

# Advanced Match

This match will allow you to match more comples structs

```rs
enum Discount {
    Percent(i32),
    Flat(i32),
}

struct Ticket {
    event: String,
    price: i32,
}

fn main() {
    let n = 3;

    match n {
        3 => println!("three"),
        other => println!("number : {}", other),
    }

    let flat = Discount::Flat(2);

    match flat {
        Discount::Flat(2) => println!("Flat discount of amount: 2"),
        Discount::Flat(amount) => println!("Flat discount of amount: {:?}", amount),
        _ => (), // The set of parenthesis show it returns nothing
    }

    let concert = Ticket {
        event: "Concert".to_owned(),
        price: 50,
    };

    // Matching a struct
    // The two dots (..) tells the compiler to ignore everything else in the struct
    match concert {
        Ticket { price: 50, .. } => println!("price of event is  50"),
        Ticket { price, .. } => println!("price = {}", price),
        _ => (),
    }
}
```

## Advanced Match activity

```rs
enum Ticket {
    Backstage(i32, String),
    Standard(i32),
    Vip(i32, String),
}

fn main() {
    let tickets = vec![
        Ticket::Backstage(60, "Jayson".to_owned()),
        Ticket::Vip(50, "Bellice".to_owned()),
        Ticket::Standard(20)
    ];

    for ticket in tickets {
        match ticket {
            Ticket::Backstage(price, holder) =>
                println!("{} ticket for backstage @ {}", holder, price),
            Ticket::Vip(price, holder) => println!("{}'s ticket for VIP @ {}", holder, price),
            Ticket::Standard(price) => println!("Just another standard user @ {}", price),
        }
    }
}
```

## Option

A type that may contain one of two types

```rs
    enum Option<T> {
        Some(T),
        None
}
```

### Demo

```rs
struct Customer {
    age: Option<i32>,
    email: String,
}

fn main() {
    let mark = Customer {
        age: Some(22),
        email: "marktop@custommail.com".to_owned(),
    };
    let becky = Customer {
        age: None,
        email: "beckytop@custommail.com".to_owned(),
    };

    match mark.age {
        Some(age) => {
            println!("Beckys age is : {}", age);
        }
        None => println!("No age was provide"),
    }
}
```

```rs
struct GroceryItem {
    name: String,
    qty: i32,
}

fn find_quantity(name: &str) -> Option<i32> {
    let groceries = vec![
        GroceryItem {
            name: "Banana".to_owned(),
            qty: 4,
        },
        GroceryItem {
            name: "Eggs".to_owned(),
            qty: 10,
        },
        GroceryItem {
            name: "Bread".to_owned(),
            qty: 2,
        }
    ];

    for item in groceries {
        if item.name == name {
            return Some(item.qty);
        }
    }
    return None;
}

fn main() {
    let quantity = find_quantity("Banana");

    println!("{:#?}",quantity);
}
```

```rs
struct Survey {
    q1: Option<i32>,
    q2: Option<bool>,
    q3: Option<String>,
}

fn main() {
    let response = Survey {
        q1: None,
        q2: Some(true),
        q3: Some("A".to_owned()),
    };

    match response.q1 {
        Some(data) => println!("The data is for q1 {}", data),
        None => println!("No data entered"),
    }

    match response.q2 {
        Some(data) => println!("The data is for q2 {}", data),
        None => println!("No data entered"),
    }

    match response.q3 {
        Some(data) => println!("The data is for q3 {}", data),
        None => println!("No data entered"),
    }
}
```

# Result

The result represents either success or failure\
Useful when working with functionality that would possibly fail

```rs
enum Result <T,E> {
    Ok(T),
    Err(E)
}
```

```rs
#[derive(Debug)]
enum MenuChoice {
    MainMenu,
    Start,
    Quit,
}

fn get_choice(input: &str) -> Result<MenuChoice, String> {
    match input {
        "mainmenu" => Ok(MenuChoice::MainMenu),
        "start" => Ok(MenuChoice::Start),
        "quit" => Ok(MenuChoice::Quit),
        _ => Err("Menu is unavailable".to_owned()),
    }
}

fn print_choice(choice: &MenuChoice) {
    println!("{:?}", choice);
}

fn main() {
    let choice: Result<MenuChoice, _> = get_choice("leave");

    match choice {
        Ok(inner_choice) => print_choice(&inner_choice),
        Err(e) => println!("error = {:?}", e),
    }
}
```

```rs
#[derive(Debug)]
enum MenuChoice {
    MainMenu,
    Start,
    Quit,
}

fn get_choice(input: &str) -> Result<MenuChoice, String> {
    match input {
        "mainmenu" => Ok(MenuChoice::MainMenu),
        "start" => Ok(MenuChoice::Start),
        "quit" => Ok(MenuChoice::Quit),
        _ => Err("Menu is unavailable".to_owned()),
    }
}

fn print_choice(choice: &MenuChoice) {
    println!("{:?}", choice);
}

fn pick_choice(input: &str) -> Result<(),String>{
    let choice: MenuChoice = get_choice(input)?; 

    print_choice(&choice);

    Ok(())
}

fn main() {
   let _ =  pick_choice("start");
}
```

```rs
    #[derive(Debug)]
struct Adult {
    name: String,
    age: i32,
}

impl Adult {
    fn new(age: i32, name: &str) -> Result<Self, &str> {
        if age >= 21 {
            return Ok(Self {
                name: name.to_string(),
                age,
            });
        } else {
            return Err("You are not of age little man");
        }
    }
}

fn main() {
    let adult1 = Adult::new(22, "John Doe");
    let adult2 = Adult::new(18, "Jane Doe");

    match adult1 {
        Ok(adult) => println!("{:?}", adult.name),
        Err(e) => println!("Error: {:?}", e),
    }

    match adult2 {
        Ok(adult) => println!("{:?}", adult.name),
        Err(e) => println!("Error: {:?}", e),
    }
}
```
