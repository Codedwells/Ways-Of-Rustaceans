
fn main (){
    let mut x  = 5; // mutable variable
    const MAX_POINTS: u32 = 100_000; // constant variable
    
    let y = 5;
    let y = "five"; // shadowing variable
    
    // INTEGER TYPES
    // Length      Signed	 Unsigned
    // 8-bit        i8	       u8
    // 16-bit       i16        u16
    // 32-bit       i32        u32
    // 64-bit       i64        u64
    // 128-bit      i128       u128
    // arch         isize      usize
    let x = 2; // i32
    
    // FLOATING-POINT TYPES
    // Length      Signed	 Unsigned
    // 32-bit       f32        -
    // 64-bit       f64        -
    let x = 2.0; // f64
    
    // BOOLEAN TYPES
    // Length      Signed	 Unsigned
    // 1-bit        bool       -
    let is_true = true;
    
    // CHARACTER TYPES
    // Length      Signed     Unsigned
    // 8-bit        char       -
    let c = 'z';
    
    // COMPOUND TYPES
    // Length      Signed     Unsigned
    // variable     tuple      array
    let tup: (i32, f64, u8) = (500, 6.4, 1);

}