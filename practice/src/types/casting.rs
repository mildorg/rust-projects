pub fn practice_casting() {
    let decimal: f32 = 65.4321;

    // let integer: u8 = decimal;
    let integer = decimal as u8;
    let character = integer as char;

    // let character = decimal as char;
    println!("Casting： {} -> {} -> {}", decimal, integer, character);

    println!("1000 as u16 is: {}", 1000 as u16);
    // println!("1000 as u8 is: {}", 1000 as u8);
    println!("-1 as u8 is: {}", (-1i8) as u8);
    println!("1000 mod 256 is: {}", 1000 % 256);

    println!("128 as i16 is: {}", 128 as i16);
    // println!("128 as i8 is: {}", 128 as i8);

    println!("300 as u8  is: {}", 300.0_f32 as u8);
    println!("-100 as u8 is: {}", -100.0_f32 as u8);
    println!("nan as u8  is: {}", f32::NAN as u8);

    unsafe {
        println!("300  is: {}", 300.0_f32.to_int_unchecked::<u8>());
        println!("-100 as u8 is: {}", (-100.0_f32).to_int_unchecked::<u8>());
        println!("nan as u8  is: {}", f32::NAN.to_int_unchecked::<u8>());
    }
}
