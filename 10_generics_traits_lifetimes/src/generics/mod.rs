pub fn init() {
    let number_list1 = vec![23, 45, 21, 3, 46, 7, 88];
    let largest = get_largest(number_list1);
    println!("The largest number is {}", largest);

    let char_list = vec!['s', 'l', 'w', 'q', 'i', 'd'];
    let largest = get_largest(char_list);
    println!("The largest letter is {}", largest);
}

// Generic methods
fn _generic_methods() {
    struct Point<T> {
        x: T,
        y: T,
    }

    impl<U> Point<U> {
        fn x(&self) -> &U {
            &self.x
        }
    }

    impl Point<f64> {
        fn y(&self) -> f64 {
            self.y
        }
    }

    let p = Point { x: 5, y: 10 };
    p.x();
    let p1 = Point { x: 5.0, y: 1.0 };
    p1.y();
}

// Generics are placed inside the angle brackets
// Here we are narrowing the type to tell the compiler T can be compared
// This is done using traits
fn get_largest<T: PartialOrd + Copy>(number_list: Vec<T>) -> T {
    let mut largest = number_list[0];

    for n in number_list {
        if n > largest {
            largest = n;
        }
    }

    largest
}
