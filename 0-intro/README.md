# Intro to Rust programming

## Control flow using Match in Rust

Match expression make decision on all possible  (exhaustive)

_ The underscore matches anything that is not in the availed choices

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

## Repetition using loop keyword in Rust

Loop will "loop" till we break out of it.

```rs

fn main() {
    let mut my_number: i32 = 0;

    loop {
        if my_number > 10 {
            break;
        }
        println!("{:?}", my_number);

        my_number = my_number + 1;
    }
}

```

## Repetition using while keyword

While loops are a way to ensure a loop will exit after a condition is no longer true.

```rs

fn main() {
    let mut my_number: i32 = 0;

    while my_number < 11 {
        println!("{my_number}");

        my_number = my_number + 1;
    }
}

```



