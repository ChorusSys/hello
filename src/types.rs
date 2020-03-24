pub fn run() {
    //  Max values for various data types
    println!("Max i32: Decimal={0}, Hex={0:x}, Binary={0:b}", std::i32::MAX);
    println!("Max u32: Decimal={0}, Hex={0:x}, Binary={0:b}", std::u32::MAX);
    println!("Max i64: Decimal={0}, Hex={0:x}, Binary={0:b}", std::i64::MAX);
    println!("Max u64: Decimal={0}, Hex={0:x}, Binary={0:b}", std::u64::MAX);

    let is_active : bool = true;
    let is_weekday : bool = false;
    let is_greater : bool = 14 > 7;
    let smiley : char = '\u{1F600}';
    println!("{:?}", (smiley, is_active, is_weekday, is_greater))
}