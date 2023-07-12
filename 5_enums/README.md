# Enums

## Options enum

Option allows us to handle instances where a value could be null (not present). It provides some methods \
e.g unwrap_or() which allow us to default to a value in such edge cases

```rs
fn main (){
  let a: i8 = 1;
  let b: Option<i8> = Some(4);

  // option type allows us to handle null values
  let sum:i8 = a + b.unwrap_or(0);

}
```

## if let syntax

This is a way of perfoming a shorter  match where you would want to handle only a few cases

```rs
fn main (){
    let some_value = Some(5);

    match some_value{
        Some(3)=> println!("three"),
        _ => println!("others"),
    }

    // Same can be achieved using if let syntax
    if let Some(3) = some_value{
        println!("three");   
    }
}
```
