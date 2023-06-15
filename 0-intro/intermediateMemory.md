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

## More of Impl, structs and enums

```rs
enum BoxColor {
    RED,
    BLUE,
    GREEN,
}
 
impl BoxColor {
    fn print_color(&self) {
        match self {
            BoxColor::RED => println!("The color of this box is : Red"),
            BoxColor::BLUE => println!("The color of this box is : Blue"),
            BoxColor::GREEN => println!("The color of this box is : Green"),
        }
    }
}

struct Dimensions {
    length: f64,
    width: f64,
    depth: f64,
}

impl Dimensions {
    fn print_dimensions(&self) {
        println!("The length is :{}", self.length);
        println!("The width is :{}", self.width);
        println!("The depth is :{}", self.depth);
    }
}

struct Shippingbox {
    dimensions: Dimensions,
    weight: f64,
    color: BoxColor,
}

impl Shippingbox {
    fn new(weight: f64, color: BoxColor, dimensions:Dimensions) -> Self {
        return Self {
            weight,
            dimensions,
            color,
        };
    }

    fn print_box(&self) {
        self.color.print_color();
        self.dimensions.print_dimensions();
        println!("The weight of this box is :{}", self.weight);
    }
}

fn main() {
    let dimensions = Dimensions {
        length: 34.0,
        width: 24.0,
        depth: 14.0,
    };

    let blue_box = Shippingbox::new(12.0, BoxColor::BLUE, dimensions);

    blue_box.print_box();
}

```

# Vectors

Vectors are arrays or list;
How to create vecors in Rust;

```rs
fn main() {
    let my_nums = vec![1, 2, 3, 4];
    let mut my_numbers = Vec::new();

    my_nums.push(4);

    println!("{}", my_nums[my_nums.len() - 1])
}
```

## Iterating a vector

```rs
struct Test {
    score: i32,
}

fn main() {
    let my_scores = vec![
        Test { score: 90 },
        Test { score: 20 },
        Test { score: 40 },
        Test { score: 60 }
    ];

    let mut total_score: i32 = 0;

    for test in my_scores {
        total_score = total_score + test.score;
    }

    println!("{}", total_score)
}
```

```rs
fn main() {
    let my_list = vec![10, 20, 30, 40];

    for item in &my_list {
        match item {
            30 => println!("Thirty"),
            _ => println!("{}", item),
        }
    }
    println!("The length of the vector is :{}", my_list.len());
}

```

# String and &str

There are two types of strings;

- **Owned String**
- **Borrowd String**

### To Create an owned string

```rs
    let owned_string = "owned string".to_owned();
    let another_owned = String::from("another string");
```

### To create a borrowed string

```rs
fn print_it(data:&str){
    println!("{:?}",data);
}

fn main () {
    print_it("This string will be borrowed");
}

```

```rs
struct LineItem {
    name: String,
    count: i32,
};

fn print_name (name:&str){
    println!("{}",name);
}

fn main() {
    let recipt = vec![
        LineItem {
            count: 3,
            name: "cereal".to_owned(),
        },
        LineItem {
            name: String::from("Apple"),
            count: 3,
        }
    ];

    for item in recipt {
        print_name(&item.name)
    }
}
```

```rs
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
```

