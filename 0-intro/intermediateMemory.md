# Memory in Rust

## Ownership

Rust uses ownership to track memory.
Functions own where the variable is created. \
 We can only borrow the data from own using **&** . Only the owner is allowed to delete the variable.

```rs
enum Light {
    Bright,
    Dull,
}

fn display_light(light: &Light) {
    match light {
        Light::Bright => println!("bright"),
        Light::Dull => println!("dull"),
    }
}

fn main() {
    let dull = Light::Dull;
    let bright = Light::Bright;

    display_light(&dull);
    display_light(&bright);
}

```

```rs

struct Book {
    pages: i32,
    rating: i32,
}

fn display_page_count(book: &Book) {
    println!("Total page count = {:?}", book.pages);
}

fn display_rating(book: &Book) {
    println!("The rating is = {:?}", book.rating);
}

fn main() {
    let book = Book {
        pages: 450,
        rating: 5,
    };

    display_page_count(&book);
    display_rating(&book)
}

```

# Implimentation Blocks

An Impl is used to encloase code related to a specific scope \
We use **self** to refrence the struct

```rs
struct Temprature {
    defrees_f: f64,
}

impl Temprature {
    fn show_temp(temp: &Temprature) {
        println!("The overall temp is :{}", temp.defrees_f)
    }
}

fn main() {
    let hot = Temprature { defrees_f: 99.9 };

    Temprature::show_temp(&hot);
}
```

```rs
struct Temprature {
    defrees_f: f64,
}

// Lower case self  show there is already a temprature declared somewhere
// Uppercase Self
impl Temprature {
    fn freezing() -> Self {
        return Self { defrees_f: 32.5 };
    }

    fn show_temp(&self) {
        println!("The overall temp is :{}", self.defrees_f);
    }
}

fn main() {
    let hot = Temprature { defrees_f: 99.9 };

    // We can access the function using dot notion since we implimented the struct
    hot.show_temp();

    let cold = Temprature::freezing();
    cold.show_temp();
}
```
