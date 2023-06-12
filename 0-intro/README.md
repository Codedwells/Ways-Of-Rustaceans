# Match in rust

Match expression make decision on all possible options

```rs

fn main() {
    let my_name = "Bob";

    match my_name {
        "Jayson" => println!("Hello Jayson I have to tell you something"),
        "Bob" => println!("I saw you yesterday in the mall"),
        _ => println!("Hello there nice to meet you!"),
    }
}

```
