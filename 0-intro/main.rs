fn cartesian() -> (i32, i32, i32) {
    return (2, 5, 8);
}

fn main() {
    let (_x, y, _z) = cartesian();

    if y < 5 {
        println!("Less than five!!");
    } else if y > 5 {
        println!("Greater than five!!");
    } else {
        println!("Equal to five");
    }

    match y {
        4 => println!("Less than five!!"),
        5 => println!("Equal to five"),
        _ => println!("Less greater than five!!"),
    }
}
 