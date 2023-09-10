// Iterators in rust
// Iterators in rust impliment the iterator trait in std lib

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    // .into_iter() takes ownership of the vector
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

fn main() {
    let v1: Vec<i32> = vec![1, 2, 3, 4, 5];

    // iter() returns an iterator over immutable references
    v1.iter().for_each(|x| println!("For each: {}\n", x));

    // .map() takes a closure then returns an iterator that applies a function to each element
    v1.iter()
        .map(|x| x + 1)
        .for_each(|x| println!("For each map:{}\n", x));

    // calling .collect() on an iterator will collect the results into a collection
    let v2: Vec<i32> = v1.iter().map(|x| x + 1).collect();
    println!("v2: {:?}", v2);

    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_my_size(shoes, 10);
        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        );
    }
}
