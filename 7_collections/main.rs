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