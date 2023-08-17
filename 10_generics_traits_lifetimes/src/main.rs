fn main() {
    let number_list1 = vec![23, 45, 21, 3, 46, 7, 88];
    let largest = get_largest(number_list1);
    println!("The largest number is {}",largest);

    let char_list = vec!['s','l','w','q','i','d'];
    let largest = get_largest(char_list);
    println!("The largest number is {}",largest);
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
