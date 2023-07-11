// ---------- Ownership Rules ----------
// 1. Each value in Rust has a variable that’s called its owner.
// 2. There can only be one owner at a time.
// 3. When the owner goes out of scope, the value will be dropped.


fn main(){
    let s1 = String::from("hello");
    let s2 = s1;
    let s3 = &s2;
    let s4 = s2.clone(); // clone() will create a deep copy of the data on the heap (expensive)

    println!("{}, world!", s2); // Will work since s3 only borrows s2
    // println!("{}, world!", s1); // Will cause error since s1 is no longer valid(having been moved to s2)
}