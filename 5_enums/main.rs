fn main (){
  let a: i8 = 1;
  let b: Option<i8> = Some(4);

  // option type allows us to handle null values
  let sum:i8 = a + b.unwrap_or(0);

}