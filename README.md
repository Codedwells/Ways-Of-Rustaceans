# The Book
The Rust lang book in code. All curated from the [Let's Get Rusty channel]('https://www.youtube.com/watch?v=OX9HJsJUDxA&list=PLai5B987bZ9CoVR-QEIN9foz4QCJ0H2Y8&index=1)

## **1. Hello Universe**
[Here](1_hello_universe/) we write our first Welcome me universe code.\
=> [main.rs](1_hello_universe/main.rs)
```rs
    fn main() {
        println!("Welcome me Universe!!!")
    }
```

### **Cargo**
In rust we have cargo an equal of npm in the Rust Universe.\
**Cargo.lock** being an equivalent of **package.lock.json**\
**Cargo.toml** being an equivalent of **package.json**

To init a **new cargo project** you need to install Rust and Cargo then run this code in your terminal of choice.\
``cargo new <name_of_project> ``\
To compile this Cargo project we would use, while in the directory of your project.\
``cargo build`` Will build out your project.
``cargo run`` Will build and run the project in one step.\
``cargo check`` Will check your file for errors without producing an executable.

## **2. Guessing game in Rust**
In this [chapter](https://www.youtube.com/watch?v=H0xBSbnQYds&list=PLai5B987bZ9CoVR-QEIN9foz4QCJ0H2Y8&index=2) we made a guessing game in about 40 lines of Rust.

