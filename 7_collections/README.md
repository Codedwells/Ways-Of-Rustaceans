# Collections

## Vectors

Vectors can increase or decrease in size. They are stored in the heap.\
If you to try to access an element at an index out of bounds the program panicks at runtime to avoid this you would use ``.get()``
which returns a ``Option<Some,None>``

```rs
fn main (){
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
}
```

### Iterating elements in a vector

```rs
fn main (){
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

  for i in &v {
    println!("{}", i);
  }
}
```

You can also modify  elements in the vector

```rs
fn main (){
    let mut v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

  for i in &mut v {
        *i += 50;
    }

    for i in &v {
        println!("{}", i);
  }
}
```

## Strings

Strings are stored as a collection of UTF-8 encoded bytes
